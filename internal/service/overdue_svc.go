package service

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/zajca/zfaktury/internal/domain"
)

// overdueInvoiceRepo defines the invoice repository methods needed by OverdueService.
//
// All methods are scoped to a single company via the companyID parameter.
type overdueInvoiceRepo interface {
	ListOverdueCandidateIDs(ctx context.Context, companyID int64, beforeDate time.Time) ([]int64, error)
	UpdateStatus(ctx context.Context, companyID, id int64, status string) error
}

// statusHistoryRepo defines the status history repository methods needed by OverdueService.
//
// All methods are scoped to a single company via the companyID parameter.
type statusHistoryRepo interface {
	Create(ctx context.Context, companyID int64, change *domain.InvoiceStatusChange) error
	ListByInvoiceID(ctx context.Context, companyID, invoiceID int64) ([]domain.InvoiceStatusChange, error)
}

// OverdueService handles overdue detection and invoice status history.
type OverdueService struct {
	invoiceRepo overdueInvoiceRepo
	historyRepo statusHistoryRepo
}

// NewOverdueService creates a new OverdueService.
func NewOverdueService(invoiceRepo overdueInvoiceRepo, historyRepo statusHistoryRepo) *OverdueService {
	return &OverdueService{
		invoiceRepo: invoiceRepo,
		historyRepo: historyRepo,
	}
}

// CheckOverdue finds all sent invoices past their due date within the given company,
// marks them as overdue, and records the status change in history.
// Returns the number of invoices marked.
func (s *OverdueService) CheckOverdue(ctx context.Context, companyID int64) (int, error) {
	now := time.Now()

	// First, find all candidates before updating.
	ids, err := s.invoiceRepo.ListOverdueCandidateIDs(ctx, companyID, now)
	if err != nil {
		return 0, fmt.Errorf("listing overdue candidates: %w", err)
	}

	if len(ids) == 0 {
		return 0, nil
	}

	count := 0
	for _, id := range ids {
		if err := s.invoiceRepo.UpdateStatus(ctx, companyID, id, domain.InvoiceStatusOverdue); err != nil {
			slog.Error("failed to mark invoice as overdue", "invoice_id", id, "error", err)
			continue
		}
		count++

		change := &domain.InvoiceStatusChange{
			InvoiceID: id,
			OldStatus: domain.InvoiceStatusSent,
			NewStatus: domain.InvoiceStatusOverdue,
			ChangedAt: now,
			Note:      "automatically marked as overdue",
		}
		if err := s.historyRepo.Create(ctx, companyID, change); err != nil {
			slog.Error("failed to record status change for overdue invoice", "invoice_id", id, "error", err)
		}
	}

	return count, nil
}

// GetHistory returns the status change history for a given invoice within the given company.
func (s *OverdueService) GetHistory(ctx context.Context, companyID, invoiceID int64) ([]domain.InvoiceStatusChange, error) {
	history, err := s.historyRepo.ListByInvoiceID(ctx, companyID, invoiceID)
	if err != nil {
		return nil, fmt.Errorf("fetching invoice status history: %w", err)
	}
	return history, nil
}
