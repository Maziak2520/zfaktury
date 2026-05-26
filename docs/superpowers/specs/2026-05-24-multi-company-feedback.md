# Architectural Feedback: Multi-Company Support

**Date:** 2026-05-24  
**Reviewer:** Gemini CLI (Architect Role)  
**Status:** Comprehensive Review  
**Subject:** `2026-05-24-multi-company-design.md`

## 1. Executive Summary

The proposed design for multi-company support is robust, well-aligned with the existing 3-layer architecture, and provides a clear migration path for existing users. The decision to use a URL-path-based partitioning (`/api/v1/companies/{id}/...`) is superior to header-based or ambient context approaches for a tool where a single human operator might open multiple companies in different browser tabs.

However, there are several "blind spots" regarding database-level integrity, frontend race conditions, and specific edge cases in the Czech tax domain that should be addressed before implementation begins.

---

## 2. Technical Critique & Recommendations

### 2.1 Database Integrity (The "Swiss Cheese" Problem)
The design relies heavily on the **service layer** to prevent cross-company data leakage (e.g., Company A's invoice referencing Company B's contact). While this is manageable for a small team, it creates a high "cognitive load" for future developers.

*   **Observation:** The design dismisses composite Foreign Keys (FKs) as "out of scope for v1".
*   **Risk:** A single bug in a repository `WHERE` clause or a missed `company_id` check in a service could lead to data corruption or incorrect tax filings.
*   **Recommendation:** Use **Composite Foreign Keys** where possible.
    *   Example: `FOREIGN KEY (company_id, contact_id) REFERENCES contacts (company_id, id)`.
    *   This ensures that SQLite itself enforces isolation, making it impossible to "leak" entities across companies even if the application code has a bug. Since SQLite is being rebuilt during migration anyway, adding these constraints now is cheaper than retrofitting them later.

### 2.2 API Consistency: The "Global" vs "Per-Company" Split
The split between global routes (ARES, CNB, Backups) and per-company routes is logical. However, one specific area is ambiguous: **The Audit Log**.

*   **Observation:** The audit log is global but contains an optional `company_id`.
*   **Recommendation:** Ensure the `audit_log` repository remains strictly "Global Tier" but provide a `companyID` filter in the `List` method. This allows the UI to show "Everything I did today" vs "Everything done for Company X".

### 2.3 Frontend: Race Conditions and "Stale UI"
The frontend uses a reactive store and `$effect` to re-trigger data loading on company switch.

*   **Risk:** If a user triggers a "Save" action for Company A but switches to Company B *before* the request completes, what happens?
*   **Analysis:** If the `currentCompany.current.id` is read at the *start* of the function and baked into the API call, the data lands in the correct company. However, if the UI then displays a "Success" toast or redirects to a list page, it might redirect to the list page of the *now-active* Company B, while the user just saved something for Company A.
*   **Recommendation:** 
    1.  **Immutability in Actions:** Ensure that any form submission "captures" the `company_id` at the moment of submission, not at the moment of response.
    2.  **Global Loading/Lock:** Consider a brief global overlay during company switching to ensure all stores are flushed and no stale "Save" buttons are clickable.

### 2.4 Czech Tax Specifics: The "Settings" Migration
The migration moves 17 keys from `settings` to `companies`.

*   **Observation:** Many "settings" in the Czech context are actually linked to the **Business Year** and the **Legal Entity** simultaneously.
*   **Risk:** The design partitions the remaining `settings` table by `company_id`. This is correct. However, ensure that **Email Templates** (which might contain the company name/ICO) are handled carefully. If a template is shared, it will now have stale hardcoded data from the "default" company.
*   **Recommendation:** Introduce "placeholder" variables in the email template engine (e.g., `{{.Supplier.Name}}`) instead of allowing hardcoded company names in settings keys that are now partitioned.

### 2.5 Soft-Delete Semantics: The "Orphan" Risk
The design blocks company deletion if "non-deleted" records exist.

*   **Risk:** "Soft-deleted" records still exist in the database. If a user soft-deletes an invoice, then soft-deletes the company, and later "hard-deletes" the invoice (if such a feature is added), the system must remain stable.
*   **Recommendation:** Implement a `HardDelete` or `Purge` command in the CLI for advanced users to clean up old companies, ensuring it handles all child tables.

---

## 3. Implementation Specifics (Surgical Advice)

### 3.1 Repository Churn
Changing ~20 repositories to accept `companyID` is a massive but mechanical task.

*   **Strategy:** Use a `BaseRepository` pattern or a helper function to inject the `company_id = ?` clause.
*   **Testing:** Instead of just "Isolation" tests, add a **"Leak Detector"** test suite that attempts to `Get` every entity in Company A using Company B's context and asserts `ErrNotFound`.

### 3.2 The Onboarding UX
For fresh installs, the list of companies is empty.

*   **Critique:** The design says "redirect to `/companies/new`". 
*   **Refinement:** Make `/companies/new` a "Welcome to zfaktury" screen for new users. Ensure that if they hit the API directly without a company, it returns a specific `412 Precondition Failed` or `428 Precondition Required` instead of a generic `404`, so the frontend knows to show the onboarding flow.

---

## 4. Final Verdict

**The plan is solid.** The risks are primarily around **Database Integrity** and **UI State Synchronization**. 

**Actionable next steps for the Implementer:**
1.  Verify the `025_multi_company.sql` migration against a production-sized SQLite database to ensure the "rename-copy-drop" cycle doesn't hit memory limits.
2.  Prioritize the **Composite Foreign Keys** in the migration script—it is a one-time cost for a lifetime of safety.
3.  Add a `X-Company-ID` header to all API responses. This allows the frontend to verify that the data it just received actually belongs to the company it thinks is active (detecting race conditions).

---
*End of Feedback*
