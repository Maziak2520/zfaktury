# Architectural Review: Multi-Company Support (v2 Design)

**Date:** 2026-05-24  
**Reviewer:** Gemini CLI (Architect Role)  
**Status:** Approved  
**Subject:** `2026-05-24-multi-company-design.md` (v2)

## 1. Executive Summary

The v2 design document is a significant improvement over v1, demonstrating a deep understanding of both the architectural goals and the pragmatic constraints of the current tech stack (SQLite + SvelteKit). The author has successfully balanced "theoretical purity" (universal composite FKs) with "operational safety" (minimizing migration complexity).

I fully endorse the v2 plan. The added "Leak Detector" test suite and the "Production-sized Migration Test" are excellent additions that mitigate the risks I previously identified.

---

## 2. Review of v2 Improvements

### 2.1 Pragmatic Data Integrity (Composite FKs)
The author's decision to use **Selective Composite Foreign Keys** for parent-child aggregation paths (Invoices, Expenses, VAT Filings) is a brilliant middle ground. It protects the integrity of financial totals (where silent corruption is most dangerous) while avoiding the operational risk of rebuilding 25+ tables in a single SQLite transaction.
*   **Verdict:** Approved. The risk/reward ratio is correctly balanced.

### 2.2 Race Condition Mitigation
The v2 plan now captures `companyId` at submission time and uses it to verify the response context. The addition of the `X-Company-Id` response header and the "context-explaining toast" addresses the UI race condition issues effectively.
*   **Verdict:** Approved. This ensures a polished UX even if a user is "fast-switching" between tabs.

### 2.3 Comprehensive Testing Strategy
The inclusion of a **Table-Driven Leak Detector** test is a high-signal, low-maintenance way to ensure that the service-layer isolation remains intact as the codebase grows. This effectively compensates for the lack of universal composite FKs.
*   **Verdict:** Approved. This is a "senior engineer" solution to a scaling problem.

---

## 3. Evaluation of "Push Back" Reasoning

I have reviewed the reasoning provided in the "Review notes (v2)" section and find the arguments technically sound:

1.  **Email Templates (Item 2.4):** Verified. Since the bodies are code-based constants with placeholders and the `settings` table is correctly partitioned, the risk of "stale data" is handled at the right layer.
2.  **Hard Delete / Purge (Item 2.5):** Agreed. YAGNI is the correct approach for v1. Local SQLite access provides a sufficient "escape hatch" for power users.
3.  **BaseRepository (Item 3.1a):** Agreed. Maintaining stylistic consistency with the existing "explicit SQL" pattern is more important than introducing a new abstraction layer for this specific feature.
4.  **4xx Onboarding Codes (Item 3.2):** Agreed. The frontend bootstrap flow is the correct place to handle the "zero companies" state.

---

## 4. Final Recommendation

The design is ready for implementation. I recommend the following sequence for the developer:
1.  **Test First:** Implement the `migrations_test.go` and the `leak_detector_test.go` early in the process to provide immediate feedback on the mechanical changes to repositories.
2.  **Migration Focus:** The "rename-copy-drop" cycle in SQLite is the highest-risk phase. The "Production-sized" test should be run frequently during development.

**Status: READY FOR IMPLEMENTATION**

---
*End of Final Review*
