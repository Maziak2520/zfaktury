package service

import (
	"context"
	"fmt"
	"time"

	"github.com/zajca/zfaktury/internal/domain"
	"github.com/zajca/zfaktury/internal/repository"
)

// RecurringExpenseService provides business logic for recurring expense management.
type RecurringExpenseService struct {
	repo     repository.RecurringExpenseRepo
	expenses *ExpenseService
	audit    *AuditService
}

// NewRecurringExpenseService creates a new RecurringExpenseService.
func NewRecurringExpenseService(repo repository.RecurringExpenseRepo, expenses *ExpenseService, audit *AuditService) *RecurringExpenseService {
	return &RecurringExpenseService{repo: repo, expenses: expenses, audit: audit}
}

// validFrequencies lists accepted frequency values.
var validFrequencies = map[string]bool{
	"weekly":    true,
	"monthly":   true,
	"quarterly": true,
	"yearly":    true,
}

// Create validates and persists a new recurring expense under the given company.
func (s *RecurringExpenseService) Create(ctx context.Context, companyID int64, re *domain.RecurringExpense) error {
	if re.Name == "" {
		return fmt.Errorf("recurring expense name is required: %w", domain.ErrInvalidInput)
	}
	if re.Description == "" {
		return fmt.Errorf("recurring expense description is required: %w", domain.ErrInvalidInput)
	}
	if re.Amount == 0 {
		return fmt.Errorf("recurring expense amount is required: %w", domain.ErrInvalidInput)
	}
	if re.NextIssueDate.IsZero() {
		return fmt.Errorf("recurring expense next issue date is required: %w", domain.ErrInvalidInput)
	}
	if !validFrequencies[re.Frequency] {
		return fmt.Errorf("invalid frequency %q, must be one of: weekly, monthly, quarterly, yearly: %w", re.Frequency, domain.ErrInvalidInput)
	}
	if re.CurrencyCode == "" {
		re.CurrencyCode = domain.CurrencyCZK
	}
	if re.BusinessPercent == 0 {
		re.BusinessPercent = 100
	}
	if re.BusinessPercent < 0 || re.BusinessPercent > 100 {
		return fmt.Errorf("business share must be between 0 and 100: %w", domain.ErrInvalidInput)
	}
	if re.PaymentMethod == "" {
		re.PaymentMethod = "bank_transfer"
	}

	// Calculate VAT amount from rate if not set.
	if re.VATAmount == 0 && re.VATRatePercent > 0 {
		re.VATAmount = re.Amount.Multiply(float64(re.VATRatePercent) / (100.0 + float64(re.VATRatePercent)))
	}

	if err := s.repo.Create(ctx, companyID, re); err != nil {
		return fmt.Errorf("creating recurring expense: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "recurring_expense", re.ID, "create", nil, re)
	}
	return nil
}

// Update validates and updates an existing recurring expense within the given company.
func (s *RecurringExpenseService) Update(ctx context.Context, companyID int64, re *domain.RecurringExpense) error {
	if re.ID == 0 {
		return fmt.Errorf("recurring expense ID is required: %w", domain.ErrInvalidInput)
	}
	if re.Name == "" {
		return fmt.Errorf("recurring expense name is required: %w", domain.ErrInvalidInput)
	}
	if re.Description == "" {
		return fmt.Errorf("recurring expense description is required: %w", domain.ErrInvalidInput)
	}
	if re.Amount == 0 {
		return fmt.Errorf("recurring expense amount is required: %w", domain.ErrInvalidInput)
	}
	if re.NextIssueDate.IsZero() {
		return fmt.Errorf("recurring expense next issue date is required: %w", domain.ErrInvalidInput)
	}
	if !validFrequencies[re.Frequency] {
		return fmt.Errorf("invalid frequency %q, must be one of: weekly, monthly, quarterly, yearly: %w", re.Frequency, domain.ErrInvalidInput)
	}
	if re.BusinessPercent < 0 || re.BusinessPercent > 100 {
		return fmt.Errorf("business share must be between 0 and 100: %w", domain.ErrInvalidInput)
	}

	// Recalculate VAT amount from rate if not set.
	if re.VATAmount == 0 && re.VATRatePercent > 0 {
		re.VATAmount = re.Amount.Multiply(float64(re.VATRatePercent) / (100.0 + float64(re.VATRatePercent)))
	}

	existing, err := s.repo.GetByID(ctx, companyID, re.ID)
	if err != nil {
		return fmt.Errorf("fetching recurring expense for audit: %w", err)
	}

	if err := s.repo.Update(ctx, companyID, re); err != nil {
		return fmt.Errorf("updating recurring expense: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "recurring_expense", re.ID, "update", existing, re)
	}
	return nil
}

// Delete removes a recurring expense by ID (soft delete) within the given company.
func (s *RecurringExpenseService) Delete(ctx context.Context, companyID, id int64) error {
	if id == 0 {
		return fmt.Errorf("recurring expense ID is required: %w", domain.ErrInvalidInput)
	}
	if err := s.repo.Delete(ctx, companyID, id); err != nil {
		return fmt.Errorf("deleting recurring expense: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "recurring_expense", id, "delete", nil, nil)
	}
	return nil
}

// GetByID retrieves a recurring expense by its ID within the given company.
func (s *RecurringExpenseService) GetByID(ctx context.Context, companyID, id int64) (*domain.RecurringExpense, error) {
	if id == 0 {
		return nil, fmt.Errorf("recurring expense ID is required: %w", domain.ErrInvalidInput)
	}
	re, err := s.repo.GetByID(ctx, companyID, id)
	if err != nil {
		return nil, fmt.Errorf("fetching recurring expense: %w", err)
	}
	return re, nil
}

// List retrieves recurring expenses with pagination within the given company.
func (s *RecurringExpenseService) List(ctx context.Context, companyID int64, limit, offset int) ([]domain.RecurringExpense, int, error) {
	if limit <= 0 {
		limit = 20
	}
	if limit > 100 {
		limit = 100
	}
	if offset < 0 {
		offset = 0
	}
	items, count, err := s.repo.List(ctx, companyID, limit, offset)
	if err != nil {
		return nil, 0, fmt.Errorf("listing recurring expenses: %w", err)
	}
	return items, count, nil
}

// Activate enables a recurring expense for generation within the given company.
func (s *RecurringExpenseService) Activate(ctx context.Context, companyID, id int64) error {
	if id == 0 {
		return fmt.Errorf("recurring expense ID is required: %w", domain.ErrInvalidInput)
	}
	if err := s.repo.Activate(ctx, companyID, id); err != nil {
		return fmt.Errorf("activating recurring expense: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "recurring_expense", id, "activate", nil, nil)
	}
	return nil
}

// Deactivate disables a recurring expense from generation within the given company.
func (s *RecurringExpenseService) Deactivate(ctx context.Context, companyID, id int64) error {
	if id == 0 {
		return fmt.Errorf("recurring expense ID is required: %w", domain.ErrInvalidInput)
	}
	if err := s.repo.Deactivate(ctx, companyID, id); err != nil {
		return fmt.Errorf("deactivating recurring expense: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "recurring_expense", id, "deactivate", nil, nil)
	}
	return nil
}

// GeneratePending finds all due recurring expenses and creates actual expenses for them
// within the given company. It advances the next_issue_date and deactivates any that
// have passed their end_date. Returns the number of expenses generated.
func (s *RecurringExpenseService) GeneratePending(ctx context.Context, companyID int64, asOfDate time.Time) (int, error) {
	due, err := s.repo.ListDue(ctx, companyID, asOfDate)
	if err != nil {
		return 0, fmt.Errorf("listing due recurring expenses: %w", err)
	}

	generated := 0
	for i := range due {
		re := &due[i]

		// Create an expense from the recurring template.
		expense := &domain.Expense{
			VendorID:        re.VendorID,
			Category:        re.Category,
			Description:     re.Description,
			IssueDate:       re.NextIssueDate,
			Amount:          re.Amount,
			CurrencyCode:    re.CurrencyCode,
			ExchangeRate:    re.ExchangeRate,
			VATRatePercent:  re.VATRatePercent,
			VATAmount:       re.VATAmount,
			IsTaxDeductible: re.IsTaxDeductible,
			BusinessPercent: re.BusinessPercent,
			PaymentMethod:   re.PaymentMethod,
			Notes:           re.Notes,
		}

		if err := s.expenses.Create(ctx, companyID, expense); err != nil {
			return generated, fmt.Errorf("creating expense from recurring %d: %w", re.ID, err)
		}
		generated++

		// Advance next_issue_date.
		nextDate := re.NextDate()
		re.NextIssueDate = nextDate

		// Check if past end_date and deactivate.
		if re.EndDate != nil && nextDate.After(*re.EndDate) {
			re.IsActive = false
		}

		// Save updated next_issue_date (and is_active if deactivated).
		if err := s.repo.Update(ctx, companyID, re); err != nil {
			return generated, fmt.Errorf("updating recurring expense %d next date: %w", re.ID, err)
		}
	}

	return generated, nil
}
