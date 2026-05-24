-- +goose Up
-- +goose StatementBegin

-- 1. Companies table — the new home for what was 17 settings keys.
CREATE TABLE companies (
	id              INTEGER PRIMARY KEY AUTOINCREMENT,
	name            TEXT    NOT NULL,
	legal_name      TEXT    NOT NULL,
	ico             TEXT    NOT NULL,
	dic             TEXT,
	vat_registered  INTEGER NOT NULL DEFAULT 0,
	street          TEXT, house_number TEXT, city TEXT, zip TEXT,
	email           TEXT, phone TEXT,
	first_name      TEXT, last_name TEXT,
	bank_account    TEXT, bank_code TEXT, iban TEXT, swift TEXT,
	logo_path       TEXT, accent_color TEXT,
	created_at      TEXT NOT NULL,
	updated_at      TEXT NOT NULL,
	deleted_at      TEXT
);
CREATE UNIQUE INDEX idx_companies_ico_active ON companies(ico) WHERE deleted_at IS NULL;

-- 2. Seed the default company from existing settings.
-- The WHERE EXISTS guard makes fresh installs (empty settings) a no-op.
INSERT INTO companies (
	id, name, legal_name, ico, dic, vat_registered,
	street, house_number, city, zip,
	email, phone,
	first_name, last_name,
	bank_account, bank_code, iban, swift,
	logo_path, accent_color,
	created_at, updated_at
)
SELECT
	1,
	COALESCE((SELECT value FROM settings WHERE key='company_name'), 'My Company'),
	COALESCE((SELECT value FROM settings WHERE key='company_name'), 'My Company'),
	COALESCE((SELECT value FROM settings WHERE key='ico'), ''),
	NULLIF((SELECT value FROM settings WHERE key='dic'), ''),
	CASE WHEN COALESCE((SELECT value FROM settings WHERE key='vat_registered'), '0') = '1' THEN 1 ELSE 0 END,
	NULLIF((SELECT value FROM settings WHERE key='street'), ''),
	NULLIF((SELECT value FROM settings WHERE key='house_number'), ''),
	NULLIF((SELECT value FROM settings WHERE key='city'), ''),
	NULLIF((SELECT value FROM settings WHERE key='zip'), ''),
	NULLIF((SELECT value FROM settings WHERE key='email'), ''),
	NULLIF((SELECT value FROM settings WHERE key='phone'), ''),
	NULLIF((SELECT value FROM settings WHERE key='first_name'), ''),
	NULLIF((SELECT value FROM settings WHERE key='last_name'), ''),
	NULLIF((SELECT value FROM settings WHERE key='bank_account'), ''),
	NULLIF((SELECT value FROM settings WHERE key='bank_code'), ''),
	NULLIF((SELECT value FROM settings WHERE key='iban'), ''),
	NULLIF((SELECT value FROM settings WHERE key='swift'), ''),
	NULLIF((SELECT value FROM settings WHERE key='logo_path'), ''),
	NULLIF((SELECT value FROM settings WHERE key='accent_color'), ''),
	datetime('now'),
	datetime('now')
WHERE EXISTS (SELECT 1 FROM settings LIMIT 1);

-- 3. Strip the 17 identity keys from settings (now lifted into companies).
DELETE FROM settings WHERE key IN (
	'company_name', 'ico', 'dic', 'vat_registered',
	'street', 'house_number', 'city', 'zip',
	'email', 'phone',
	'first_name', 'last_name',
	'bank_account', 'bank_code', 'iban', 'swift',
	'logo_path', 'accent_color'
);

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin

-- Restore the 17 identity keys from company id=1 (best-effort; multi-company
-- users will lose everything but the first company on downgrade).
INSERT INTO settings (key, value)
SELECT 'company_name', name FROM companies WHERE id = 1
UNION ALL SELECT 'ico', ico FROM companies WHERE id = 1
UNION ALL SELECT 'dic', COALESCE(dic, '') FROM companies WHERE id = 1
UNION ALL SELECT 'vat_registered', CASE WHEN vat_registered = 1 THEN '1' ELSE '0' END FROM companies WHERE id = 1
UNION ALL SELECT 'street', COALESCE(street, '') FROM companies WHERE id = 1
UNION ALL SELECT 'house_number', COALESCE(house_number, '') FROM companies WHERE id = 1
UNION ALL SELECT 'city', COALESCE(city, '') FROM companies WHERE id = 1
UNION ALL SELECT 'zip', COALESCE(zip, '') FROM companies WHERE id = 1
UNION ALL SELECT 'email', COALESCE(email, '') FROM companies WHERE id = 1
UNION ALL SELECT 'phone', COALESCE(phone, '') FROM companies WHERE id = 1
UNION ALL SELECT 'first_name', COALESCE(first_name, '') FROM companies WHERE id = 1
UNION ALL SELECT 'last_name', COALESCE(last_name, '') FROM companies WHERE id = 1
UNION ALL SELECT 'bank_account', COALESCE(bank_account, '') FROM companies WHERE id = 1
UNION ALL SELECT 'bank_code', COALESCE(bank_code, '') FROM companies WHERE id = 1
UNION ALL SELECT 'iban', COALESCE(iban, '') FROM companies WHERE id = 1
UNION ALL SELECT 'swift', COALESCE(swift, '') FROM companies WHERE id = 1
UNION ALL SELECT 'logo_path', COALESCE(logo_path, '') FROM companies WHERE id = 1
UNION ALL SELECT 'accent_color', COALESCE(accent_color, '') FROM companies WHERE id = 1;

DROP INDEX IF EXISTS idx_companies_ico_active;
DROP TABLE IF EXISTS companies;

-- +goose StatementEnd
