# Upgrading to multi-company (v0.0.51 and later)

Migration `025_multi_company.sql` is the first multi-company migration.
It rewrites the database in-place — back up your `~/.zfaktury/zfaktury.db`
before upgrading.

## What happens

1. A `companies` table is created.
2. A default company (id = 1) is created from your existing settings
   (company_name, ico, dic, address, bank details, etc.).
3. Every per-company table (contacts, invoices, expenses, tax filings,
   etc.) gains a `company_id` column, backfilled to 1.
4. Composite foreign keys are added to invoice_items, expense_items,
   recurring_invoice_items, vat_return_invoices, and vat_return_expenses
   for physical cross-company isolation.

## Known limitation: tax-year tables

The following tables retain their original `UNIQUE(year)` constraint
from migrations 015-017 and are not yet partitioned for multi-company
UPSERT collisions:

- `tax_year_settings`
- `tax_prepayments`
- `tax_spouse_credits`
- `tax_child_credits`
- `tax_personal_credits`
- `tax_deductions`

Reads from these tables ARE properly scoped by `company_id`, so cross-
company leakage is impossible. The limitation only manifests if two
companies try to UPSERT the same year row simultaneously. A follow-up
migration will extend the unique constraints to `(company_id, year)`.

## Downgrade is destructive

The `Down` migration restores the 17 identity keys from id = 1 only.
Any data you created in companies other than the first will be lost
on downgrade. Back up your database before testing the downgrade
path.

## Renaming the default company

After upgrade, visit `/companies/[id]` (clickable from the header
dropdown's "Spravovat firmy →" link) to rename or fill in additional
fields. You can add more companies from "+ Přidat firmu" — IČO
lookup against ARES auto-fills most fields.
