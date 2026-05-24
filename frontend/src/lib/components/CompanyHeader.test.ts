import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { render, fireEvent, screen, cleanup } from '@testing-library/svelte';
import { currentCompany } from '$lib/stores/currentCompany.svelte';
import CompanyHeader from './CompanyHeader.svelte';
import type { Company } from '$lib/api/client';

vi.mock('$app/navigation', () => ({ goto: vi.fn() }));

const A: Company = {
	id: 1,
	name: 'Manas OSVČ',
	legal_name: 'A',
	ico: '1',
	vat_registered: false,
	created_at: '',
	updated_at: ''
};
const B: Company = {
	id: 2,
	name: 'Manas s.r.o.',
	legal_name: 'B',
	ico: '2',
	vat_registered: false,
	created_at: '',
	updated_at: ''
};

beforeEach(async () => {
	currentCompany.reset();
	currentCompany.setCompanies([A, B]);
	currentCompany.select(1);
	const { goto } = await import('$app/navigation');
	vi.mocked(goto).mockClear();
});

afterEach(() => {
	cleanup();
});

describe('CompanyHeader', () => {
	it('renders current company name', () => {
		render(CompanyHeader);
		expect(screen.getByText('Manas OSVČ')).toBeInTheDocument();
	});

	it('opens dropdown listing all companies', async () => {
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		expect(screen.getByText('Manas s.r.o.')).toBeInTheDocument();
	});

	it('selects another company on click', async () => {
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		await fireEvent.click(screen.getByText('Manas s.r.o.'));
		expect(currentCompany.current?.id).toBe(2);
	});

	it('navigates to /companies when Manage is clicked', async () => {
		const { goto } = await import('$app/navigation');
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		await fireEvent.click(screen.getByText(/spravovat/i));
		expect(goto).toHaveBeenCalledWith('/companies');
	});

	it('navigates to /companies/new when Add is clicked', async () => {
		const { goto } = await import('$app/navigation');
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		await fireEvent.click(screen.getByText(/přidat firmu/i));
		expect(goto).toHaveBeenCalledWith('/companies/new');
	});

	it('shows fallback label when no company is selected', () => {
		currentCompany.reset();
		render(CompanyHeader);
		expect(screen.getByText('Žádná firma')).toBeInTheDocument();
	});

	it('marks the active company with a check mark', async () => {
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		const activeMarker = screen.getByLabelText('aktivní');
		expect(activeMarker).toBeInTheDocument();
		expect(activeMarker.textContent).toBe('✓');
	});

	it('closes dropdown after selecting a company', async () => {
		render(CompanyHeader);
		await fireEvent.click(screen.getByRole('button', { name: /manas osvč/i }));
		expect(screen.getByText(/spravovat/i)).toBeInTheDocument();
		await fireEvent.click(screen.getByText('Manas s.r.o.'));
		expect(screen.queryByText(/spravovat/i)).not.toBeInTheDocument();
	});
});
