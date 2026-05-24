package service

import (
	"context"
	"errors"
	"fmt"

	"github.com/zajca/zfaktury/internal/domain"
	"github.com/zajca/zfaktury/internal/repository"
)

// EntityChecker reports whether a company has any non-deleted records
// of a particular kind (invoices, expenses, etc.). The CompanyService
// consults every registered checker before allowing a soft-delete.
type EntityChecker interface {
	CountNonDeletedForCompany(ctx context.Context, companyID int64) (int, error)
}

// CompanyService provides business logic for managing companies.
//
// It enforces the two spec invariants on delete:
//  1. The last remaining company cannot be soft-deleted (ErrLastCompany).
//  2. A company with any non-deleted child records cannot be soft-deleted
//     (ErrInUse). Child types are pluggable via the EntityChecker list.
//
// Real EntityChecker implementations (for invoices, expenses, etc.) are
// wired in once those repositories gain a companyID parameter in Phase 3.
type CompanyService struct {
	repo     repository.CompanyRepo
	checkers []EntityChecker
	audit    *AuditService
}

// NewCompanyService creates a new CompanyService.
//
// repo is the CompanyRepo interface; checkers is the list of child-entity
// checkers consulted on delete (may be empty/nil); audit is best-effort and
// may be nil — when present, create/update/delete actions are logged.
func NewCompanyService(repo repository.CompanyRepo, checkers []EntityChecker, audit *AuditService) *CompanyService {
	return &CompanyService{repo: repo, checkers: checkers, audit: audit}
}

// Create validates and persists a new company.
func (s *CompanyService) Create(ctx context.Context, c domain.Company) (int64, error) {
	if err := c.Validate(); err != nil {
		return 0, err
	}
	id, err := s.repo.Create(ctx, c)
	if err != nil {
		return 0, fmt.Errorf("creating company: %w", err)
	}
	if s.audit != nil {
		c.ID = id
		s.audit.Log(ctx, "company", id, "create", nil, c)
	}
	return id, nil
}

// Get returns a company by ID.
func (s *CompanyService) Get(ctx context.Context, id int64) (domain.Company, error) {
	c, err := s.repo.GetByID(ctx, id)
	if err != nil {
		return domain.Company{}, fmt.Errorf("fetching company: %w", err)
	}
	return c, nil
}

// List returns all active (non-deleted) companies.
func (s *CompanyService) List(ctx context.Context) ([]domain.Company, error) {
	list, err := s.repo.List(ctx)
	if err != nil {
		return nil, fmt.Errorf("listing companies: %w", err)
	}
	return list, nil
}

// Update validates and updates an existing company.
func (s *CompanyService) Update(ctx context.Context, c domain.Company) error {
	if err := c.Validate(); err != nil {
		return err
	}
	if err := s.repo.Update(ctx, c); err != nil {
		return fmt.Errorf("updating company: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "company", c.ID, "update", nil, c)
	}
	return nil
}

// Delete soft-deletes a company after enforcing the two spec invariants:
//
//  1. ErrLastCompany when only one active company remains.
//  2. ErrInUse when any registered EntityChecker reports non-deleted
//     child records for this company.
//
// The cheap last-company guard runs first to short-circuit checker queries.
func (s *CompanyService) Delete(ctx context.Context, id int64) error {
	// Rule 1: cannot delete the last company.
	n, err := s.repo.CountActive(ctx)
	if err != nil {
		return fmt.Errorf("counting active companies: %w", err)
	}
	if n <= 1 {
		return domain.ErrLastCompany
	}

	// Rule 2: cannot delete a company with any non-deleted children.
	for _, ck := range s.checkers {
		count, err := ck.CountNonDeletedForCompany(ctx, id)
		if err != nil {
			return fmt.Errorf("checking for child records: %w", err)
		}
		if count > 0 {
			return domain.ErrInUse
		}
	}

	if err := s.repo.SoftDelete(ctx, id); err != nil {
		if errors.Is(err, domain.ErrNotFound) {
			return err
		}
		return fmt.Errorf("soft-deleting company: %w", err)
	}
	if s.audit != nil {
		s.audit.Log(ctx, "company", id, "delete", nil, nil)
	}
	return nil
}
