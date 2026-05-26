package repository

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"time"

	"github.com/zajca/zfaktury/internal/domain"
)

// CategoryRepository handles persistence of ExpenseCategory entities.
type CategoryRepository struct {
	db *sql.DB
}

// NewCategoryRepository creates a new CategoryRepository.
func NewCategoryRepository(db *sql.DB) *CategoryRepository {
	return &CategoryRepository{db: db}
}

// Create inserts a new expense category under the given company.
func (r *CategoryRepository) Create(ctx context.Context, companyID int64, cat *domain.ExpenseCategory) error {
	now := time.Now()
	cat.CreatedAt = now

	result, err := r.db.ExecContext(ctx, `
		INSERT INTO expense_categories (company_id, key, label_cs, label_en, color, sort_order, is_default, created_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
		companyID, cat.Key, cat.LabelCS, cat.LabelEN, cat.Color, cat.SortOrder, cat.IsDefault, cat.CreatedAt.Format(time.RFC3339),
	)
	if err != nil {
		return fmt.Errorf("inserting expense category: %w", err)
	}

	id, err := result.LastInsertId()
	if err != nil {
		return fmt.Errorf("getting last insert id for expense category: %w", err)
	}
	cat.ID = id
	return nil
}

// Update modifies an existing expense category within the given company.
func (r *CategoryRepository) Update(ctx context.Context, companyID int64, cat *domain.ExpenseCategory) error {
	result, err := r.db.ExecContext(ctx, `
		UPDATE expense_categories SET
			key = ?, label_cs = ?, label_en = ?, color = ?, sort_order = ?
		WHERE id = ? AND company_id = ? AND deleted_at IS NULL`,
		cat.Key, cat.LabelCS, cat.LabelEN, cat.Color, cat.SortOrder, cat.ID, companyID,
	)
	if err != nil {
		return fmt.Errorf("updating expense category %d: %w", cat.ID, err)
	}

	rows, err := result.RowsAffected()
	if err != nil {
		return fmt.Errorf("checking rows affected for expense category %d: %w", cat.ID, err)
	}
	if rows == 0 {
		return fmt.Errorf("expense category %d not found or already deleted: %w", cat.ID, domain.ErrNotFound)
	}
	return nil
}

// Delete performs a soft delete on an expense category within the given company.
func (r *CategoryRepository) Delete(ctx context.Context, companyID, id int64) error {
	now := time.Now()
	result, err := r.db.ExecContext(ctx, `
		UPDATE expense_categories SET deleted_at = ? WHERE id = ? AND company_id = ? AND deleted_at IS NULL`,
		now.Format(time.RFC3339), id, companyID,
	)
	if err != nil {
		return fmt.Errorf("soft-deleting expense category %d: %w", id, err)
	}

	rows, err := result.RowsAffected()
	if err != nil {
		return fmt.Errorf("checking rows affected for expense category %d: %w", id, err)
	}
	if rows == 0 {
		return fmt.Errorf("expense category %d not found or already deleted: %w", id, domain.ErrNotFound)
	}
	return nil
}

// GetByID retrieves a single expense category by ID within the given company.
func (r *CategoryRepository) GetByID(ctx context.Context, companyID, id int64) (*domain.ExpenseCategory, error) {
	cat := &domain.ExpenseCategory{}
	var createdAtStr string
	var deletedAtStr sql.NullString

	err := r.db.QueryRowContext(ctx, `
		SELECT id, key, label_cs, label_en, color, sort_order, is_default, created_at, deleted_at
		FROM expense_categories
		WHERE id = ? AND company_id = ? AND deleted_at IS NULL`, id, companyID,
	).Scan(
		&cat.ID, &cat.Key, &cat.LabelCS, &cat.LabelEN, &cat.Color,
		&cat.SortOrder, &cat.IsDefault, &createdAtStr, &deletedAtStr,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, fmt.Errorf("expense category %d not found: %w", id, domain.ErrNotFound)
		}
		return nil, fmt.Errorf("querying expense category %d: %w", id, err)
	}

	cat.CreatedAt, err = parseDate(time.RFC3339, createdAtStr)
	if err != nil {
		return nil, fmt.Errorf("scanning expense category: %w", err)
	}
	cat.DeletedAt, err = parseDatePtr(time.RFC3339, deletedAtStr)
	if err != nil {
		return nil, fmt.Errorf("scanning expense category: %w", err)
	}
	return cat, nil
}

// GetByKey retrieves a single expense category by its unique key within the given company.
func (r *CategoryRepository) GetByKey(ctx context.Context, companyID int64, key string) (*domain.ExpenseCategory, error) {
	cat := &domain.ExpenseCategory{}
	var createdAtStr string
	var deletedAtStr sql.NullString

	err := r.db.QueryRowContext(ctx, `
		SELECT id, key, label_cs, label_en, color, sort_order, is_default, created_at, deleted_at
		FROM expense_categories
		WHERE key = ? AND company_id = ? AND deleted_at IS NULL`, key, companyID,
	).Scan(
		&cat.ID, &cat.Key, &cat.LabelCS, &cat.LabelEN, &cat.Color,
		&cat.SortOrder, &cat.IsDefault, &createdAtStr, &deletedAtStr,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, fmt.Errorf("expense category with key %q not found: %w", key, domain.ErrNotFound)
		}
		return nil, fmt.Errorf("querying expense category by key %q: %w", key, err)
	}

	cat.CreatedAt, err = parseDate(time.RFC3339, createdAtStr)
	if err != nil {
		return nil, fmt.Errorf("scanning expense category by key: %w", err)
	}
	cat.DeletedAt, err = parseDatePtr(time.RFC3339, deletedAtStr)
	if err != nil {
		return nil, fmt.Errorf("scanning expense category by key: %w", err)
	}
	return cat, nil
}

// List retrieves all non-deleted expense categories for the given company,
// ordered by sort_order.
func (r *CategoryRepository) List(ctx context.Context, companyID int64) ([]domain.ExpenseCategory, error) {
	rows, err := r.db.QueryContext(ctx, `
		SELECT id, key, label_cs, label_en, color, sort_order, is_default, created_at, deleted_at
		FROM expense_categories
		WHERE company_id = ? AND deleted_at IS NULL
		ORDER BY sort_order ASC, key ASC`, companyID,
	)
	if err != nil {
		return nil, fmt.Errorf("listing expense categories: %w", err)
	}
	defer func() { _ = rows.Close() }()

	var categories []domain.ExpenseCategory
	for rows.Next() {
		var cat domain.ExpenseCategory
		var createdAtStr string
		var deletedAtStr sql.NullString

		if err := rows.Scan(
			&cat.ID, &cat.Key, &cat.LabelCS, &cat.LabelEN, &cat.Color,
			&cat.SortOrder, &cat.IsDefault, &createdAtStr, &deletedAtStr,
		); err != nil {
			return nil, fmt.Errorf("scanning expense category row: %w", err)
		}

		cat.CreatedAt, err = parseDate(time.RFC3339, createdAtStr)
		if err != nil {
			return nil, fmt.Errorf("scanning expense category row: %w", err)
		}
		cat.DeletedAt, err = parseDatePtr(time.RFC3339, deletedAtStr)
		if err != nil {
			return nil, fmt.Errorf("scanning expense category row: %w", err)
		}
		categories = append(categories, cat)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterating expense category rows: %w", err)
	}
	return categories, nil
}
