-- v024 fixture: a single-company database immediately before migration 025.
-- All company-identity settings keys are populated; a handful of
-- per-company entities are present so the migration's backfill is observable.

INSERT INTO settings (key, value) VALUES
	('company_name',    'Manas OSVČ'),
	('ico',             '12345678'),
	('dic',             'CZ12345678'),
	('vat_registered',  '1'),
	('street',          'Václavské náměstí'),
	('house_number',    '1'),
	('city',            'Praha'),
	('zip',             '11000'),
	('email',           'jiri@manas.cz'),
	('phone',           '+420 123 456 789'),
	('first_name',      'Jiří'),
	('last_name',       'Manas'),
	('bank_account',    '1234567890'),
	('bank_code',       '0100'),
	('iban',            'CZ0001000000001234567890'),
	('swift',           'KOMBCZPP'),
	('logo_path',       ''),
	('accent_color',    '#1a56db'),
	('default_payment_method', 'bank_transfer');

INSERT INTO contacts (id, type, name, ico, dic, email, created_at, updated_at) VALUES
	(1, 'company', 'Keboola s.r.o.',    '25620916', 'CZ25620916', 'fakturace@keboola.com', '2026-01-15T10:00:00Z', '2026-01-15T10:00:00Z'),
	(2, 'company', 'Acme spol. s r.o.', '11223344', NULL,         'billing@acme.cz',       '2026-02-01T10:00:00Z', '2026-02-01T10:00:00Z');

INSERT INTO invoice_sequences (id, prefix, next_number, year, format_pattern) VALUES
	(1, 'FV', 3, 2026, '{prefix}{year}{number:04d}'),
	(2, 'ZF', 1, 2026, '{prefix}{year}{number:04d}');

INSERT INTO invoices (id, invoice_number, customer_id, issue_date, due_date, status, total_amount, created_at, updated_at) VALUES
	(1, 'FV20260001', 1, '2026-02-01', '2026-02-15', 'paid', 5000000, '2026-02-01T10:00:00Z', '2026-02-15T12:00:00Z'),
	(2, 'FV20260002', 2, '2026-03-01', '2026-03-15', 'sent', 2500000, '2026-03-01T10:00:00Z', '2026-03-01T10:00:00Z');

INSERT INTO invoice_items (id, invoice_id, description, quantity, unit_price, total_amount) VALUES
	(1, 1, 'Konzultace',      20, 250000, 5000000),
	(2, 2, 'Vývoj integrace', 10, 250000, 2500000);

INSERT INTO expenses (id, vendor_id, expense_number, description, issue_date, amount, created_at, updated_at) VALUES
	(1, 1, 'AL/2026/0001', 'Hardware nákup', '2026-02-10', 1200000, '2026-02-10T10:00:00Z', '2026-02-10T10:00:00Z');
