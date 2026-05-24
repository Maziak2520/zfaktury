package repository

import (
	"context"
	"database/sql"
	"errors"
	"testing"
	"time"

	"github.com/zajca/zfaktury/internal/domain"
	"github.com/zajca/zfaktury/internal/testutil"
)

// TestCrossCompanyLeakDetection exhaustively asserts that every per-company
// repository's Get and List methods refuse to surface rows belonging to a
// different company. Populated incrementally across Phase 3 tasks as each
// repo gains the companyID parameter:
//
//   - T20: Contact, Category, Sequence (this file's initial coverage)
//   - T21: Invoice, Expense (+ their child item tables)
//   - T22: tax, insurance, vat, vies, investment verticals
//
// Each leakCase seeds an entity under company id=1 and asserts that
// company id=2 cannot see it via GetByID (ErrNotFound) or List (empty).
type leakCase struct {
	name string
	// seed inserts a representative entity for companyID and returns its ID.
	seed func(t *testing.T, db *sql.DB, companyID int64) int64
	// getByOtherCompany attempts GetByID with the wrong company; must
	// return domain.ErrNotFound.
	getByOtherCompany func(t *testing.T, db *sql.DB, wrongCompanyID, entityID int64) error
	// listByOtherCompany returns the number of rows the wrong company can
	// see via List; must be 0.
	listByOtherCompany func(t *testing.T, db *sql.DB, wrongCompanyID int64) int
}

// setupLeakDetectorDB returns a fresh in-memory test DB with the default
// company (id=1) plus a second company (id=2) so cross-company queries have
// a valid second tenant to target.
func setupLeakDetectorDB(t *testing.T) *sql.DB {
	t.Helper()
	db := testutil.NewTestDB(t)
	now := time.Now().UTC().Format(time.RFC3339)
	if _, err := db.Exec(`
		INSERT INTO companies (id, name, legal_name, ico, vat_registered, created_at, updated_at)
		VALUES (2, 'Other Company', 'Other Company', '99999999', 0, ?, ?)`, now, now); err != nil {
		t.Fatalf("seeding second company: %v", err)
	}
	return db
}

var leakCases = []leakCase{
	{
		name: "ContactRepository",
		seed: func(t *testing.T, db *sql.DB, companyID int64) int64 {
			c := testutil.SeedContact(t, db, companyID, &domain.Contact{
				Name: "Leak-test Contact",
				ICO:  "11223344",
			})
			return c.ID
		},
		getByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID, entityID int64) error {
			repo := NewContactRepository(db)
			_, err := repo.GetByID(context.Background(), wrongCompanyID, entityID)
			return err
		},
		listByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID int64) int {
			repo := NewContactRepository(db)
			_, total, err := repo.List(context.Background(), wrongCompanyID, domain.ContactFilter{})
			if err != nil {
				t.Fatalf("ContactRepository.List(other company) error: %v", err)
			}
			return total
		},
	},
	{
		name: "CategoryRepository",
		seed: func(t *testing.T, db *sql.DB, companyID int64) int64 {
			return testutil.SeedCategory(t, db, companyID, "leak_test_cat",
				"Leak-test", "Leak test")
		},
		getByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID, entityID int64) error {
			repo := NewCategoryRepository(db)
			_, err := repo.GetByID(context.Background(), wrongCompanyID, entityID)
			return err
		},
		listByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID int64) int {
			repo := NewCategoryRepository(db)
			// Seed-default categories live under company 1 only (the
			// migration backfill), so company 2 sees only what tests put
			// there. The leak-test category was seeded into company 1 in
			// the seed() callback, so company 2's list must be empty of
			// THAT particular row; but it will still contain the 16
			// default seeded categories (which were inserted by migration
			// 010 before company_id existed, so they currently live in
			// company 1 too). For this test we only care that company 2
			// cannot see company 1's data — count must be 0.
			list, err := repo.List(context.Background(), wrongCompanyID)
			if err != nil {
				t.Fatalf("CategoryRepository.List(other company) error: %v", err)
			}
			return len(list)
		},
	},
	{
		name: "SequenceRepository",
		seed: func(t *testing.T, db *sql.DB, companyID int64) int64 {
			return testutil.SeedInvoiceSequence(t, db, companyID, "LK", 2030)
		},
		getByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID, entityID int64) error {
			repo := NewSequenceRepository(db)
			_, err := repo.GetByID(context.Background(), wrongCompanyID, entityID)
			return err
		},
		listByOtherCompany: func(t *testing.T, db *sql.DB, wrongCompanyID int64) int {
			repo := NewSequenceRepository(db)
			list, err := repo.List(context.Background(), wrongCompanyID)
			if err != nil {
				t.Fatalf("SequenceRepository.List(other company) error: %v", err)
			}
			return len(list)
		},
	},
}

func TestCrossCompanyLeakDetection(t *testing.T) {
	for _, lc := range leakCases {
		t.Run(lc.name+"_GetByID", func(t *testing.T) {
			db := setupLeakDetectorDB(t)
			id := lc.seed(t, db, 1)

			err := lc.getByOtherCompany(t, db, 2, id)
			if err == nil {
				t.Errorf("%s: company 2 can read company 1's entity %d (no error)", lc.name, id)
				return
			}
			if !errors.Is(err, domain.ErrNotFound) {
				t.Errorf("%s: got %v, want ErrNotFound", lc.name, err)
			}
		})
		t.Run(lc.name+"_List", func(t *testing.T) {
			db := setupLeakDetectorDB(t)
			lc.seed(t, db, 1)

			n := lc.listByOtherCompany(t, db, 2)
			if n != 0 {
				t.Errorf("%s: company 2 can list %d of company 1's rows; want 0", lc.name, n)
			}
		})
	}
}
