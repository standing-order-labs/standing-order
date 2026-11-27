# Standing Order

**Pull-based recurring payments for Stellar. Subscriptions, memberships and salaries as a Soroban primitive.**

[![CI](https://github.com/standing-order-labs/standing-order/actions/workflows/ci.yml/badge.svg)](https://github.com/standing-order-labs/standing-order/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Status: testnet](https://img.shields.io/badge/status-pre--audit%20%C2%B7%20testnet-orange)

Stellar has fast, cheap stablecoin payments, but no standard way to say *"charge me 10 USDC every month until I cancel."*
Standing Order is an open-source Soroban contract that fills that gap: merchants publish plans, subscribers opt in once, and anyone can trigger each due payment. Funds move straight from subscriber to merchant, and the contract never holds a balance.

> **Status:** `v0.1.0` is a working base, not production software. It has **not been audited**. Do not use it with real funds.

**Live on testnet:** contract `CC35ZGUDYXYVX3JLP4WK5SS4WTTY45ZLI6V53YY7DFOTJACCK5BTN56D` (details in [`docs/deployments/testnet.md`](docs/deployments/testnet.md)).

## Why it matters

- **Real demand.** Software subscriptions, creator memberships, rent, payroll and utilities are all recurring payments. Stablecoin rails are only useful for them if renewals can happen without a manual signature each time.
- **Trust-minimized.** Subscribers keep custody. The merchant can only pull the plan amount, once per period, within the token allowance the subscriber granted. Cancelling is one call.
- **Composable.** Works with any SEP-41 token (USDC, EURC, local-currency assets) and emits indexer-friendly events.

## Design choices

- **One shared contract for all merchants**, not a factory of per-merchant contracts: cheaper and easier to audit.
- **Pull model via token allowance**, not escrow: no locked funds and no custody risk.
- **Permissionless `charge`**: anyone (a keeper bot, the merchant, the subscriber) can trigger a due payment. Destination, amount and timing are fixed by the plan, so the caller cannot change them.
- **First period paid on `subscribe`**, so merchants are paid immediately and the subscriber's balance is verified up front.
- **Missed periods are skipped, never back-billed**, to avoid surprise multi-month charges after keeper downtime.
- **No admin keys and no upgrade path in v0.1**, by design.

## Related work

Other recurring-payment and streaming projects exist in the Stellar ecosystem. Standing Order focuses on a small, auditable, token-agnostic subscription primitive plus open keeper tooling, rather than a full billing platform.

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

## Ecosystem (planned repositories)

| Repository                | Purpose                                                  | Status  |
| ------------------------- | -------------------------------------------------------- | ------- |
| `standing-order`          | Soroban contracts (this repo)                            | Active  |
| `standing-order-sdk`      | TypeScript SDK generated from the contract bindings      | Planned |
| `standing-order-keeper`   | Keeper bot that scans subscriptions and calls `charge`   | Planned |
| `standing-order-app`      | Next.js demo with Freighter on testnet                   | Planned |

## Roadmap

Short version: v0.1 base contract, v0.2 hardening and fuzzing, v0.3 TypeScript SDK and keeper bot, v0.4 plan upgrades and trials, v1.0 audit and mainnet. Details in [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Contributing

Contributions are welcome. Start with [`CONTRIBUTING.md`](CONTRIBUTING.md) and the open issues labeled `good first issue`. A curated backlog lives in [`docs/ISSUES_BACKLOG.md`](docs/ISSUES_BACKLOG.md). Please report security issues privately as described in [`SECURITY.md`](SECURITY.md).

## License

Licensed under the [Apache License 2.0](LICENSE).