import '@testing-library/jest-dom/vitest';
import { beforeEach } from 'vitest';
import { currentCompany } from '$lib/stores/currentCompany.svelte';
import type { Company } from '$lib/api/client';

// Default test company seeded before every test. Pages / API methods that
// reach for `currentCompany.current` would otherwise throw NoCompanyError
// because the per-company URL helpers refuse to build a path without an
// active company id.
//
// Tests that intentionally exercise the "no company" path should call
// `currentCompany.reset()` in their own `beforeEach`.
export const TEST_COMPANY: Company = {
	id: 1,
	name: 'Test s.r.o.',
	legal_name: 'Test s.r.o.',
	ico: '12345678',
	vat_registered: false,
	created_at: '',
	updated_at: ''
};

beforeEach(() => {
	currentCompany.reset();
	currentCompany.setCompanies([TEST_COMPANY]);
	currentCompany.select(TEST_COMPANY.id);
});
