package service

import (
	"context"
	"errors"
	"testing"

	"github.com/zajca/zfaktury/internal/domain"
)

type stubCompanyRepo struct {
	all         []domain.Company
	countActive int
	deleted     []int64
	deleteErr   error
}

func (s *stubCompanyRepo) Create(_ context.Context, c domain.Company) (int64, error) {
	c.ID = int64(len(s.all) + 1)
	s.all = append(s.all, c)
	s.countActive++
	return c.ID, nil
}
func (s *stubCompanyRepo) GetByID(_ context.Context, id int64) (domain.Company, error) {
	for _, c := range s.all {
		if c.ID == id {
			return c, nil
		}
	}
	return domain.Company{}, domain.ErrNotFound
}
func (s *stubCompanyRepo) List(_ context.Context) ([]domain.Company, error) {
	return s.all, nil
}
func (s *stubCompanyRepo) Update(_ context.Context, _ domain.Company) error { return nil }
func (s *stubCompanyRepo) SoftDelete(_ context.Context, id int64) error {
	if s.deleteErr != nil {
		return s.deleteErr
	}
	s.deleted = append(s.deleted, id)
	s.countActive--
	return nil
}
func (s *stubCompanyRepo) CountActive(_ context.Context) (int, error) { return s.countActive, nil }

type stubEntityChecker struct{ count int }

func (s *stubEntityChecker) CountNonDeletedForCompany(_ context.Context, _ int64) (int, error) {
	return s.count, nil
}

func TestCompanyService_Delete_blocksLastCompany(t *testing.T) {
	repo := &stubCompanyRepo{countActive: 1, all: []domain.Company{{ID: 1, Name: "Only"}}}
	svc := NewCompanyService(repo, []EntityChecker{&stubEntityChecker{0}}, nil)
	err := svc.Delete(context.Background(), 1)
	if !errors.Is(err, domain.ErrLastCompany) {
		t.Errorf("err = %v, want ErrLastCompany", err)
	}
}

func TestCompanyService_Delete_blocksNonEmptyCompany(t *testing.T) {
	repo := &stubCompanyRepo{countActive: 2, all: []domain.Company{{ID: 1}, {ID: 2}}}
	svc := NewCompanyService(repo, []EntityChecker{&stubEntityChecker{count: 3}}, nil)
	err := svc.Delete(context.Background(), 1)
	if !errors.Is(err, domain.ErrInUse) {
		t.Errorf("err = %v, want ErrInUse", err)
	}
}

func TestCompanyService_Delete_succeedsWhenEmptyAndNotLast(t *testing.T) {
	repo := &stubCompanyRepo{countActive: 2, all: []domain.Company{{ID: 1}, {ID: 2}}}
	svc := NewCompanyService(repo, []EntityChecker{&stubEntityChecker{0}}, nil)
	if err := svc.Delete(context.Background(), 2); err != nil {
		t.Errorf("Delete: %v", err)
	}
	if len(repo.deleted) != 1 || repo.deleted[0] != 2 {
		t.Errorf("repo.deleted = %v, want [2]", repo.deleted)
	}
}

func TestCompanyService_Create_validates(t *testing.T) {
	svc := NewCompanyService(&stubCompanyRepo{}, nil, nil)
	_, err := svc.Create(context.Background(), domain.Company{})
	if !errors.Is(err, domain.ErrInvalidInput) {
		t.Errorf("err = %v, want ErrInvalidInput", err)
	}
}
