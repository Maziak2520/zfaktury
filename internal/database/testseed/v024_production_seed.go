// Package testseed provides generator-style fixtures for migration tests.
//
// The fixtures here are NOT placed in the conventional `testdata/` directory
// because Go's build tool ignores any directory named `testdata`, which would
// prevent it from being imported by tests in sibling packages.
package testseed

import (
	"database/sql"
	"fmt"
)

// SeedProductionSized populates a v024-shaped DB with ~5k invoices,
// ~10k invoice items, ~2.5k contacts, and ~5k expenses associated
// with one default company's settings. Used by the production-sized
// migration test (TestMultiCompanyMigrationProductionSized) to gauge
// how long migration 025 takes on a realistic dataset.
//
// The fixture matches the v024 schema exactly -- no company_id columns
// yet, and only the column lists permitted by the pre-multi-company
// table definitions.
func SeedProductionSized(db *sql.DB) error {
	// Identity settings -- mirrors what migration 025 reads to create
	// the default company row.
	identity := []string{
		`INSERT INTO settings (key, value) VALUES ('company_name', 'Bench Co')`,
		`INSERT INTO settings (key, value) VALUES ('ico', '99999999')`,
		`INSERT INTO settings (key, value) VALUES ('vat_registered', '1')`,
		`INSERT INTO settings (key, value) VALUES ('dic', 'CZ99999999')`,
	}
	for _, s := range identity {
		if _, err := db.Exec(s); err != nil {
			return fmt.Errorf("seed identity: %w", err)
		}
	}

	tx, err := db.Begin()
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	// Contacts: 2500 rows. Only NOT NULL no-default columns are `name`;
	// `type` defaults to 'company', `country` to 'CZ'. created_at /
	// updated_at default to current time but we set them explicitly to
	// keep the seed deterministic enough for debugging.
	const contactCount = 2500
	for i := 1; i <= contactCount; i++ {
		if _, err := tx.Exec(
			`INSERT INTO contacts (id, type, name, ico, email, created_at, updated_at)
			 VALUES (?, 'company', ?, ?, ?,
			         strftime('%Y-%m-%dT%H:%M:%SZ','now'),
			         strftime('%Y-%m-%dT%H:%M:%SZ','now'))`,
			i,
			fmt.Sprintf("Contact %d", i),
			fmt.Sprintf("%08d", 10000000+i),
			fmt.Sprintf("c%d@example.cz", i),
		); err != nil {
			return fmt.Errorf("contact %d: %w", i, err)
		}
	}

	// Invoice sequence: a single sequence referenced by every invoice.
	if _, err := tx.Exec(
		`INSERT INTO invoice_sequences (id, prefix, next_number, year, format_pattern, created_at, updated_at)
		 VALUES (1, 'FV', 5001, 2026, '{prefix}{year}{number:04d}',
		         strftime('%Y-%m-%dT%H:%M:%SZ','now'),
		         strftime('%Y-%m-%dT%H:%M:%SZ','now'))`,
	); err != nil {
		return fmt.Errorf("invoice_sequences: %w", err)
	}

	// Invoices: 5000 rows, each with 2 line items (= 10000 invoice_items).
	const invoiceCount = 5000
	for i := 1; i <= invoiceCount; i++ {
		if _, err := tx.Exec(
			`INSERT INTO invoices (
				id, invoice_number, customer_id, sequence_id, type, status,
				issue_date, due_date, currency_code, exchange_rate,
				subtotal_amount, vat_amount, total_amount, paid_amount,
				created_at, updated_at
			) VALUES (?, ?, ?, 1, 'regular', 'paid',
			          '2026-01-01', '2026-01-15', 'CZK', 100,
			          ?, 0, ?, 0,
			          strftime('%Y-%m-%dT%H:%M:%SZ','now'),
			          strftime('%Y-%m-%dT%H:%M:%SZ','now'))`,
			i,
			fmt.Sprintf("FV2026%05d", i),
			1+(i%contactCount), // customer_id rotates over the 2500 contacts
			100000+i*100,
			100000+i*100,
		); err != nil {
			return fmt.Errorf("invoice %d: %w", i, err)
		}
		for j := 1; j <= 2; j++ {
			itemID := (i-1)*2 + j
			if _, err := tx.Exec(
				`INSERT INTO invoice_items (
					id, invoice_id, description, quantity, unit_price,
					vat_rate_percent, vat_amount, total_amount
				) VALUES (?, ?, ?, 100, ?, 0, 0, ?)`,
				itemID,
				i,
				fmt.Sprintf("line %d", j),
				50000+j,
				50000+j,
			); err != nil {
				return fmt.Errorf("invoice_item %d: %w", itemID, err)
			}
		}
	}

	// Expenses: 5000 rows.
	const expenseCount = 5000
	for i := 1; i <= expenseCount; i++ {
		if _, err := tx.Exec(
			`INSERT INTO expenses (
				id, vendor_id, expense_number, issue_date, amount, description,
				created_at, updated_at
			) VALUES (?, ?, ?, '2026-01-01', ?, ?,
			          strftime('%Y-%m-%dT%H:%M:%SZ','now'),
			          strftime('%Y-%m-%dT%H:%M:%SZ','now'))`,
			i,
			1+(i%contactCount),
			fmt.Sprintf("DOC/%05d", i),
			10000+i,
			fmt.Sprintf("Expense %d", i),
		); err != nil {
			return fmt.Errorf("expense %d: %w", i, err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("commit: %w", err)
	}
	return nil
}
