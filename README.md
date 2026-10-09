# Standing Order

**Pull-based recurring payments for Stellar. Subscriptions, memberships and salaries as a Soroban primitive.**

[![CI](https://github.com/standing-order-labs/standing-order/actions/workflows/ci.yml/badge.svg)](https://github.com/standing-order-labs/standing-order/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Status: testnet](https://img.shields.io/badge/status-pre--audit%20%C2%B7%20testnet-orange)

Stellar has fast, cheap stablecoin payments, but no standard way to say *"charge me 10 USDC every month until I cancel."*
Standing Order is an open-source Soroban contract that fills that gap: merchants publish plans, subscribers opt in once, and anyone can trigger each due payment. Funds move straight from subscriber to merchant, and the contract never holds a balance.

> **Status:** `v0.1.0` is a working base, not production software. It has **not been audited**. Do not use it with real funds.

## Why it matters

- **Real demand.** Software subscriptions, creator memberships, rent, payroll and utilities are all recurring payments. Stablecoin rails are only useful for them if renewals can happen without a manual signature each time.
- **Trust-minimized.** Subscribers keep custody. The merchant can only pull the plan amount, once per period, within the token allowance the subscriber granted. Cancelling is one call.
- **Composable.** Works with any SEP-41 token (USDC, EURC, local-currency assets) and emits indexer-friendly events.

## How it works

```
Merchant                    Subscriber                 Keeper (anyone)
   |  create_plan(token, amount, period)                     |
   |------------------------>|                               |
   |                         | token.approve(contract, ...)  |
   |                         | subscribe(plan_id)  -> pays period 1 directly to merchant
   |                         |                               |
   |                         |        (one period later)     |
   |                         |                               | charge(sub_id)
   |<-------------------------- transfer_from(subscriber -> merchant)
   |                         | cancel(sub_id) at any time    |
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for storage layout, authorization model, and security notes.

## Contract API (`subscription-manager`)

| Function          | Caller     | Description                                                       |
| ----------------- | ---------- | ----------------------------------------------------------------- |
| `create_plan`     | merchant   | Publish a plan (token, amount, period). Returns `plan_id`.        |
| `deactivate_plan` | merchant   | Stop new subscriptions and further charges on a plan.             |
| `subscribe`       | subscriber | Enrol in a plan; first period is paid immediately.                |
| `cancel`          | subscriber | Cancel a subscription.                                            |
| `charge`          | anyone     | Pull the next payment once due. Missed periods are skipped.       |
| `get_plan` / `get_subscription` / `is_due` / `plan_count` / `subscription_count` / `version` | anyone | Read-only views. |

Errors are listed in [`contracts/subscription-manager/src/errors.rs`](contracts/subscription-manager/src/errors.rs).

## Quickstart

Prerequisites: Rust (stable, 1.91+), the `wasm32v1-none` target, and [`stellar-cli`](https://developers.stellar.org/docs/tools/cli) 25.2+.

```bash
git clone https://github.com/standing-order-labs/standing-order.git
cd standing-order

rustup target add wasm32v1-none
cargo test --workspace        # run the test suite
./scripts/build.sh            # build + optimize WASM

# Deploy to testnet
stellar keys generate deployer --network testnet --fund
./scripts/deploy.sh subscription-manager testnet deployer
```

## Repository layout

```
contracts/subscription-manager/   Soroban contract (lib, types, storage, events, tests)
docs/                             Architecture, roadmap, deployment records, issue backlog
scripts/                          build / test / deploy helpers
.github/                          CI, issue + PR templates, Dependabot
```

## Roadmap

Short version: v0.1 base contract, v0.2 hardening and fuzzing, v0.3 TypeScript SDK and keeper bot, v0.4 plan upgrades and trials, v1.0 audit and mainnet. Details in [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Contributing

Contributions are welcome. Start with [`CONTRIBUTING.md`](CONTRIBUTING.md) and the open issues labeled `good first issue`. A curated backlog lives in [`docs/ISSUES_BACKLOG.md`](docs/ISSUES_BACKLOG.md). Please report security issues privately as described in [`SECURITY.md`](SECURITY.md).

## License

Licensed under the [Apache License 2.0](LICENSE).
