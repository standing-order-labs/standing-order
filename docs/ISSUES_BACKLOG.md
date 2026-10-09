# Issues backlog

Seed list for the issue tracker. Create each as a GitHub issue with the listed labels. Every item is scoped to one PR with explicit acceptance criteria so contributors know when it is done. Remove an entry once its issue exists.

Suggested labels: `good first issue`, `help wanted`, `bug`, `enhancement`, `contract`, `tests`, `docs`, `ci`, `dx`, `security`, `sdk`, `tooling`.

## 1. Assert events in unit tests

**Labels:** good first issue,tests

Add tests that verify `PlanCreated`, `Subscribed`, `Charged`, `Cancelled` and `PlanDeactivated` are published with the expected topics and data using `env.events().all()`.

**Acceptance criteria:** One test per event; failing the contract's event output makes the test fail.

## 2. Test: subscribe fails when subscriber balance is insufficient

**Labels:** good first issue,tests

Add a test showing `subscribe` reverts when the subscriber cannot afford the first period and that no subscription or counter state is written.

**Acceptance criteria:** `subscription_count()` stays 0 after the failed call.

## 3. Test: two merchants and many subscribers do not interfere

**Labels:** good first issue,tests

Add a test with 2 plans and 3 subscribers confirming balances and schedules stay isolated.

**Acceptance criteria:** Merchant balances equal expected totals after multiple charges.

## 4. Test: boundary of `MIN_PERIOD_SECS`

**Labels:** good first issue,tests

Add tests for period exactly equal to `MIN_PERIOD_SECS` (accepted) and `u64::MAX` (accepted, no overflow panic on subscribe).

**Acceptance criteria:** Both tests pass without panics.

## 5. Add a testnet walkthrough to docs

**Labels:** good first issue,docs

Write `docs/TESTNET_WALKTHROUGH.md` with stellar-cli commands to create a token, create a plan, approve, subscribe and charge.

**Acceptance criteria:** A new contributor can follow it end-to-end on testnet.

## 6. Document the error codes in ARCHITECTURE.md

**Labels:** good first issue,docs

Add an error-code table to the architecture doc matching `errors.rs`, with when each is returned.

**Acceptance criteria:** Table matches source; linked from README.

## 7. Add a FAQ page

**Labels:** good first issue,docs

Create `docs/FAQ.md` covering: who pays gas, what happens if funds run out, how to cancel, how allowances work.

**Acceptance criteria:** At least 8 accurate Q&As, reviewed by a maintainer.

## 8. Add a CI job that checks the WASM size

**Labels:** good first issue,ci

Fail CI if the optimized WASM exceeds a documented size budget and print the size in the job summary.

**Acceptance criteria:** Job runs on PRs; budget documented in CONTRIBUTING.

## 9. Add `cargo deny` / `cargo audit` workflow

**Labels:** good first issue,ci

Add a scheduled workflow that runs `cargo audit` and reports advisories.

**Acceptance criteria:** Workflow runs weekly and on dependency PRs.

## 10. Add a `Makefile` target for coverage

**Labels:** good first issue,dx

Add `make coverage` using `cargo llvm-cov` and document it.

**Acceptance criteria:** Command produces an HTML or lcov report.

## 11. Add `subscriptions_of(subscriber)` index

**Labels:** enhancement,contract

Store a per-subscriber list of subscription ids with paginated view `list_subscriptions(subscriber, start, limit)`.

**Acceptance criteria:** Pagination tested; storage layout documented.

## 12. Add `plans_of(merchant)` index

**Labels:** enhancement,contract

Same as above for merchants with `list_plans(merchant, start, limit)`.

**Acceptance criteria:** Pagination tested; TTLs handled.

## 13. Add `restore`/`bump` entrypoint for subscriptions

**Labels:** enhancement,contract

Add a permissionless `bump_subscription(sub_id)` that extends TTL for a subscription and its plan.

**Acceptance criteria:** Tests cover extension; docs updated.

## 14. Admin role with two-step transfer and pause switch

**Labels:** enhancement,contract

Add `init_admin`, `nominate_admin`, `accept_admin`, `pause`, `unpause`. While paused, `create_plan`, `subscribe` and `charge` revert; `cancel` still works.

**Acceptance criteria:** All paths tested; ARCHITECTURE auth table updated.

## 15. Token allow-list

**Labels:** enhancement,contract

Admin-managed allow-list of tokens that plans may use.

**Acceptance criteria:** Plans with non-listed tokens are rejected; default behavior documented.

## 16. Optional max-subscribers per plan

**Labels:** enhancement,contract

Add an optional cap set at plan creation; `subscribe` fails when the cap is reached.

**Acceptance criteria:** New error code, tests, docs.

## 17. Free-trial support

**Labels:** enhancement,contract

Allow a plan to define a trial period during which the first payment is deferred.

**Acceptance criteria:** Schedule math tested, including cancel during trial.

## 18. `charge_many` batch entrypoint

**Labels:** enhancement,contract

Charge up to N due subscriptions in one call, skipping those not due or failing, and return a per-item result.

**Acceptance criteria:** Gas usage benchmarked against N single calls.

## 19. Fuzz the scheduling logic

**Labels:** enhancement,security

Add property tests (e.g., `proptest`) asserting `next_charge` is strictly increasing and never overflows for arbitrary periods and delays.

**Acceptance criteria:** Fuzz target runs in CI for a bounded time.

## 20. Reentrancy and malicious-token test

**Labels:** enhancement,security

Write a test token contract that attempts to re-enter `charge` and verify state remains consistent.

**Acceptance criteria:** Test documents the Soroban reentrancy model.

## 21. Generate TypeScript bindings

**Labels:** enhancement,sdk

Use `stellar contract bindings typescript` to create a `packages/sdk` workspace with a README and a basic example script.

**Acceptance criteria:** `npm run build` succeeds; example subscribes on testnet.

## 22. Keeper bot prototype

**Labels:** enhancement,tooling

Create `tools/keeper` (TypeScript) that scans subscriptions and calls `charge` when `is_due` is true, with retry and logging.

**Acceptance criteria:** Runs against testnet; configurable via env vars.

## 23. Event schema documentation

**Labels:** enhancement,docs

Document each event's topics and data with JSON examples for indexer authors.

**Acceptance criteria:** Matches `events.rs`; reviewed by maintainer.

## 24. Guide: revoking allowances on cancel

**Labels:** enhancement,security

Document and example-code the recommended client flow: `cancel` then `approve(..., 0, ...)` in one transaction.

**Acceptance criteria:** Example script and docs section added.
