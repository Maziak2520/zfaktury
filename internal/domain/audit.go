package domain

import "time"

// AuditLogEntry represents a single audit trail record for entity changes.
type AuditLogEntry struct {
	ID         int64
	EntityType string
	EntityID   int64
	Action     string
	OldValues  string
	NewValues  string
	CreatedAt  time.Time
}

// AuditLogFilter defines filtering options for listing audit log entries.
type AuditLogFilter struct {
	EntityType string
	EntityID   *int64
	Action     string
	From       time.Time
	To         time.Time
	// CompanyID, when non-nil, restricts results to audit log entries whose
	// company_id matches the given id. Entries with NULL company_id (system
	// events, cross-company actions) are excluded by this filter.
	CompanyID *int64
	Limit     int
	Offset    int
}
