package service

import (
	"context"
	"errors"
	"fmt"
	"regexp"
	"strings"

	"github.com/zajca/zfaktury/internal/domain"
	"github.com/zajca/zfaktury/internal/repository"
)

// keyPattern validates that a category key is lowercase alphanumeric with underscores.
var keyPattern = regexp.MustCompile(`^[a-z0-9_]+$`)

// hexColorPattern validates CSS hex color values (#RGB or #RRGGBB).
var hexColorPattern = regexp.MustCompile(`^#[0-9a-fA-F]{3}([0-9a-fA-F]{3})?$`)

// CategoryService provides business logic for expense category management.
type CategoryService struct {
	repo  repository.CategoryRepo
	audit *AuditService
}

// NewCategoryService creates a new CategoryService.
func NewCategoryService(repo repository.CategoryRepo, audit *AuditService) *CategoryService {
	return &CategoryService{repo: repo, audit: audit}
}

// Create validates and persists a new expense category under the given company.
func (s *CategoryService) Create(ctx context.Context, companyID int64, cat *domain.ExpenseCategory) error {
	if err := s.validateCategory(cat); err != nil {
		return err
	}

	// Check for duplicate key within the same company.
	existing, err := s.repo.GetByKey(ctx, companyID, cat.Key)
	if err == nil && existing != nil {
		return fmt.Errorf("category with this key already exists: %w", domain.ErrDuplicateNumber)
	}
	if err != nil && !errors.Is(err, domain.ErrNotFound) {
		return fmt.Errorf("checking category key uniqueness: %w", err)
	}

	if cat.Color == "" {
		cat.Color = "#6B7280"
	}

	if err := s.repo.Create(ctx, companyID, cat); err != nil {
		return fmt.Errorf("creating category: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "category", cat.ID, "create", nil, cat)
	}
	return nil
}

// Update validates and updates an existing expense category within the given company.
func (s *CategoryService) Update(ctx context.Context, companyID int64, cat *domain.ExpenseCategory) error {
	if cat.ID == 0 {
		return fmt.Errorf("category ID is required: %w", domain.ErrInvalidInput)
	}

	if err := s.validateCategory(cat); err != nil {
		return err
	}

	// Check for duplicate key (excluding self) within the same company.
	existingByKey, err := s.repo.GetByKey(ctx, companyID, cat.Key)
	if err == nil && existingByKey != nil && existingByKey.ID != cat.ID {
		return fmt.Errorf("category with this key already exists: %w", domain.ErrDuplicateNumber)
	}
	if err != nil && !errors.Is(err, domain.ErrNotFound) {
		return fmt.Errorf("checking category key uniqueness: %w", err)
	}

	// Fetch existing state for audit logging.
	existing, err := s.repo.GetByID(ctx, companyID, cat.ID)
	if err != nil {
		return fmt.Errorf("fetching category for update: %w", err)
	}

	if err := s.repo.Update(ctx, companyID, cat); err != nil {
		return fmt.Errorf("updating category: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "category", cat.ID, "update", existing, cat)
	}
	return nil
}

// Delete removes an expense category by ID (soft delete) within the given company.
// Default categories (is_default=1) cannot be deleted.
func (s *CategoryService) Delete(ctx context.Context, companyID, id int64) error {
	if id == 0 {
		return fmt.Errorf("category ID is required: %w", domain.ErrInvalidInput)
	}

	cat, err := s.repo.GetByID(ctx, companyID, id)
	if err != nil {
		return fmt.Errorf("fetching category for delete: %w", err)
	}

	if cat.IsDefault {
		return fmt.Errorf("default categories cannot be deleted: %w", domain.ErrInvalidInput)
	}

	if err := s.repo.Delete(ctx, companyID, id); err != nil {
		return fmt.Errorf("deleting category: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "category", id, "delete", nil, nil)
	}
	return nil
}

// GetByID retrieves an expense category by its ID within the given company.
func (s *CategoryService) GetByID(ctx context.Context, companyID, id int64) (*domain.ExpenseCategory, error) {
	if id == 0 {
		return nil, fmt.Errorf("category ID is required: %w", domain.ErrInvalidInput)
	}
	cat, err := s.repo.GetByID(ctx, companyID, id)
	if err != nil {
		return nil, fmt.Errorf("fetching category: %w", err)
	}
	return cat, nil
}

// List retrieves all expense categories for the given company.
func (s *CategoryService) List(ctx context.Context, companyID int64) ([]domain.ExpenseCategory, error) {
	cats, err := s.repo.List(ctx, companyID)
	if err != nil {
		return nil, fmt.Errorf("listing categories: %w", err)
	}
	return cats, nil
}

// validateCategory performs common validation for create and update operations.
func (s *CategoryService) validateCategory(cat *domain.ExpenseCategory) error {
	cat.Key = strings.TrimSpace(cat.Key)
	cat.LabelCS = strings.TrimSpace(cat.LabelCS)
	cat.LabelEN = strings.TrimSpace(cat.LabelEN)

	if cat.Key == "" {
		return fmt.Errorf("category key is required: %w", domain.ErrInvalidInput)
	}
	if !keyPattern.MatchString(cat.Key) {
		return fmt.Errorf("category key must be lowercase alphanumeric with underscores only: %w", domain.ErrInvalidInput)
	}
	if cat.LabelCS == "" {
		return fmt.Errorf("category Czech label is required: %w", domain.ErrInvalidInput)
	}
	if cat.LabelEN == "" {
		return fmt.Errorf("category English label is required: %w", domain.ErrInvalidInput)
	}
	if cat.Color != "" && !hexColorPattern.MatchString(cat.Color) {
		return fmt.Errorf("color must be a valid hex color (e.g. #FFF or #FF00FF): %w", domain.ErrInvalidInput)
	}
	return nil
}

// CategoryCompanyChecker reports whether a company has any non-deleted
// expense categories. It satisfies the EntityChecker interface so
// CompanyService.Delete can refuse to soft-delete a company that still
// owns categories.
type CategoryCompanyChecker struct {
	repo repository.CategoryRepo
}

// NewCategoryCompanyChecker creates a new CategoryCompanyChecker.
func NewCategoryCompanyChecker(repo repository.CategoryRepo) *CategoryCompanyChecker {
	return &CategoryCompanyChecker{repo: repo}
}

// CountNonDeletedForCompany returns the number of non-deleted expense
// categories belonging to the given company.
func (c *CategoryCompanyChecker) CountNonDeletedForCompany(ctx context.Context, companyID int64) (int, error) {
	list, err := c.repo.List(ctx, companyID)
	if err != nil {
		return 0, fmt.Errorf("counting categories for company %d: %w", companyID, err)
	}
	return len(list), nil
}
