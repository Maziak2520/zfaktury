package repository

import (
	"context"
	"testing"

	"github.com/zajca/zfaktury/internal/domain"
	"github.com/zajca/zfaktury/internal/testutil"
)

func TestAuditLogRepository_Create(t *testing.T) {
	db := testutil.NewTestDB(t)
	ctx := context.Background()
	repo := NewAuditLogRepository(db)

	entry := &domain.AuditLogEntry{
		EntityType: "invoice",
		EntityID:   42,
		Action:     "create",
		NewValues:  `{"number":"FV-001"}`,
	}

	if err := repo.Create(ctx, entry); err != nil {
		t.Fatalf("Create() error: %v", err)
	}
	if entry.ID == 0 {
		t.Fatal("expected non-zero ID after create")
	}
	if entry.CreatedAt.IsZero() {
		t.Fatal("expected non-zero CreatedAt after create")
	}
}

func TestAuditLogRepository_ListByEntity(t *testing.T) {
	db := testutil.NewTestDB(t)
	ctx := context.Background()
	repo := NewAuditLogRepository(db)

	// Create entries for two different entities.
	entry1 := &domain.AuditLogEntry{
		EntityType: "contact",
		EntityID:   1,
		Action:     "create",
		NewValues:  `{"name":"Alice"}`,
	}
	entry2 := &domain.AuditLogEntry{
		EntityType: "contact",
		EntityID:   1,
		Action:     "update",
		OldValues:  `{"name":"Alice"}`,
		NewValues:  `{"name":"Bob"}`,
	}
	entry3 := &domain.AuditLogEntry{
		EntityType: "contact",
		EntityID:   2,
		Action:     "create",
		NewValues:  `{"name":"Charlie"}`,
	}

	for _, e := range []*domain.AuditLogEntry{entry1, entry2, entry3} {
		if err := repo.Create(ctx, e); err != nil {
			t.Fatalf("Create() error: %v", err)
		}
	}

	// List entries for entity (contact, 1).
	entries, err := repo.ListByEntity(ctx, "contact", 1)
	if err != nil {
		t.Fatalf("ListByEntity() error: %v", err)
	}
	if len(entries) != 2 {
		t.Fatalf("expected 2 entries, got %d", len(entries))
	}

	// Verify DESC ordering (newest first).
	if entries[0].Action != "update" {
		t.Errorf("first entry action = %q, want %q", entries[0].Action, "update")
	}
	if entries[1].Action != "create" {
		t.Errorf("second entry action = %q, want %q", entries[1].Action, "create")
	}

	// Verify old/new values are preserved.
	if entries[0].OldValues != `{"name":"Alice"}` {
		t.Errorf("OldValues = %q, want %q", entries[0].OldValues, `{"name":"Alice"}`)
	}
	if entries[0].NewValues != `{"name":"Bob"}` {
		t.Errorf("NewValues = %q, want %q", entries[0].NewValues, `{"name":"Bob"}`)
	}
}

func TestAuditLogRepository_ListByEntity_Empty(t *testing.T) {
	db := testutil.NewTestDB(t)
	ctx := context.Background()
	repo := NewAuditLogRepository(db)

	// Non-existent entity should return empty slice, not error.
	entries, err := repo.ListByEntity(ctx, "invoice", 9999)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 0 {
		t.Fatalf("expected 0 entries, got %d", len(entries))
	}
}

func TestAuditLogRepository_ListByCompany(t *testing.T) {
	db := testutil.NewTestDB(t)
	ctx := context.Background()
	repo := NewAuditLogRepository(db)

	// Seed company 2 so company_id=2 satisfies the FK reference. Company 1
	// is seeded by NewTestDB.
	if _, err := db.Exec(`INSERT INTO companies (id, name, legal_name, ico, vat_registered, created_at, updated_at)
		VALUES (2, 'Second', 'Second', '11111111', 0, strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))`); err != nil {
		t.Fatalf("seeding company 2: %v", err)
	}

	// Seed: one row for company 1, one for company 2, one with NULL company.
	if _, err := db.Exec(`INSERT INTO audit_log (action, entity_type, entity_id, company_id, created_at)
		VALUES ('a', 'x', 1, 1, strftime('%Y-%m-%dT%H:%M:%SZ','now')),
		       ('b', 'x', 2, 2, strftime('%Y-%m-%dT%H:%M:%SZ','now')),
		       ('c', 'x', 3, NULL, strftime('%Y-%m-%dT%H:%M:%SZ','now'))`); err != nil {
		t.Fatalf("seed: %v", err)
	}

	one := int64(1)
	got, _, err := repo.List(ctx, domain.AuditLogFilter{CompanyID: &one})
	if err != nil {
		t.Fatalf("List: %v", err)
	}
	if len(got) != 1 || got[0].Action != "a" {
		t.Errorf("got %d rows, want 1 with action 'a'", len(got))
	}

	// Without filter, all three returned.
	got, _, err = repo.List(ctx, domain.AuditLogFilter{})
	if err != nil {
		t.Fatalf("List unfiltered: %v", err)
	}
	if len(got) != 3 {
		t.Errorf("unfiltered = %d, want 3", len(got))
	}
}

func TestAuditLogRepository_Create_NullableValues(t *testing.T) {
	db := testutil.NewTestDB(t)
	ctx := context.Background()
	repo := NewAuditLogRepository(db)

	// Create entry with empty old/new values.
	entry := &domain.AuditLogEntry{
		EntityType: "expense",
		EntityID:   10,
		Action:     "delete",
	}

	if err := repo.Create(ctx, entry); err != nil {
		t.Fatalf("Create() error: %v", err)
	}

	entries, err := repo.ListByEntity(ctx, "expense", 10)
	if err != nil {
		t.Fatalf("ListByEntity() error: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected 1 entry, got %d", len(entries))
	}
	if entries[0].OldValues != "" {
		t.Errorf("OldValues = %q, want empty", entries[0].OldValues)
	}
	if entries[0].NewValues != "" {
		t.Errorf("NewValues = %q, want empty", entries[0].NewValues)
	}
}
