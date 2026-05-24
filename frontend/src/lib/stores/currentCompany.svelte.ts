import { browser } from '$app/environment';
import type { Company } from '$lib/api/client';
import { toastWarning } from '$lib/data/toast-state.svelte';

const STORAGE_KEY = 'zfaktury.company';

let current = $state<Company | null>(null);
let companies = $state<Company[]>([]);

export const currentCompany = {
	get current() {
		return current;
	},
	get companies() {
		return companies;
	},

	setCompanies(list: Company[]) {
		companies = list;
		// If current is no longer in the list (e.g. soft-deleted), clear it.
		if (current && !list.find((c) => c.id === current!.id)) {
			current = null;
			if (browser) localStorage.removeItem(STORAGE_KEY);
		}
	},

	select(id: number) {
		const found = companies.find((c) => c.id === id);
		if (!found) return;
		current = found;
		if (browser) localStorage.setItem(STORAGE_KEY, String(id));
	},

	restoreSelection(): number | null {
		if (!browser) return null;
		const raw = localStorage.getItem(STORAGE_KEY);
		if (!raw) return null;
		const id = Number(raw);
		return Number.isFinite(id) && id > 0 ? id : null;
	},

	nameOf(id: number | null | undefined): string {
		if (id == null) return '?';
		return companies.find((c) => c.id === id)?.name ?? '?';
	},

	reset() {
		current = null;
		companies = [];
		if (browser) localStorage.removeItem(STORAGE_KEY);
	}
};

// Returns true when the response came back for a different company than the
// one currently active (the user switched mid-flight). When true, a warning
// toast is shown and the caller should skip its post-success navigation.
export function notifyIfSwitchedCompany(
	submittedFor: number,
	respondedFor?: number
): boolean {
	const activeId = currentCompany.current?.id;
	if (submittedFor === activeId) return false;
	const submittedName = currentCompany.nameOf(submittedFor);
	const activeName = currentCompany.nameOf(activeId);
	toastWarning(
		`Uloženo do firmy ${submittedName} — mezitím jste přepnuli na ${activeName}.`
	);
	return true;
}
