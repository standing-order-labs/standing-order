# Roadmap

Dates are intentionally omitted; milestones are ordered by priority. Each item maps to issues in the tracker.

## v0.1 - Base contract (this release)

- [x] Plans, subscriptions, permissionless `charge`, cancellation
- [x] Unit tests, CI, build/deploy scripts
- [x] Architecture and security docs
- [ ] Testnet deployment recorded in `docs/deployments/testnet.md`

## v0.2 - Hardening

- [ ] Property/fuzz tests for the scheduling math
- [ ] Event assertions in tests
- [ ] Restore/extend-TTL helper for archived entries
- [ ] Pause switch and admin role (two-step transfer)
- [ ] Per-plan subscriber cap and token allow-list
- [ ] Gas/resource benchmarks tracked in CI

## v0.3 - Tooling

- [ ] TypeScript SDK generated from contract bindings, with examples
- [ ] Keeper bot that watches due subscriptions and calls `charge`
- [ ] Indexer-friendly event schema doc
- [ ] CLI walkthrough in `docs/`

## v0.4 - Features

- [ ] Free trials and first-period discounts
- [ ] Plan price updates with subscriber consent
- [ ] Optional protocol fee configuration
- [ ] Batch `charge_many` for keepers

## v1.0 - Production

- [ ] External security audit and published report
- [ ] Mainnet deployment and verified WASM hashes
- [ ] Stable ABI and semantic-versioning guarantees
- [ ] Reference frontend (separate repository)

## How to influence the roadmap

Open a feature request. Items with a clear problem statement and acceptance criteria are the easiest to prioritize.
