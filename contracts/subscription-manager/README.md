# subscription-manager

Soroban contract implementing pull-based recurring payments. See the [root README](../../README.md) for the overview and [`docs/ARCHITECTURE.md`](../../docs/ARCHITECTURE.md) for design details.

## Authorization model

| Function          | Requires auth from      |
| ----------------- | ----------------------- |
| `create_plan`     | `merchant` argument     |
| `deactivate_plan` | the plan's merchant     |
| `subscribe`       | `subscriber` argument   |
| `cancel`          | the subscription's subscriber |
| `charge`          | nobody (permissionless) |

## Error codes

| Code | Name                    | Meaning                                         |
| ---- | ----------------------- | ----------------------------------------------- |
| 1    | `InvalidAmount`         | Amount must be > 0                              |
| 2    | `InvalidPeriod`         | Period below `MIN_PERIOD_SECS` (3600)           |
| 3    | `PlanNotFound`          | Unknown plan id                                 |
| 4    | `PlanInactive`          | Plan was deactivated                            |
| 5    | `SubscriptionNotFound`  | Unknown subscription id                         |
| 6    | `SubscriptionNotActive` | Subscription cancelled                          |
| 7    | `NotDue`                | Next charge time not reached                    |
| 8    | `PaymentFailed`         | Token transfer failed (balance/allowance)       |
| 9    | `PlanAlreadyInactive`   | Plan already deactivated                        |

## Events

`PlanCreated`, `PlanDeactivated`, `Subscribed`, `Charged`, `Cancelled`. Ids are indexed as topics.

## Develop

```bash
cargo test -p subscription-manager
```
