# Testnet deployments

Network: Stellar Testnet (`Test SDF Network ; September 2015`)

| Contract               | Version | Contract ID | WASM hash | Date |
| ---------------------- | ------- | ----------- | --------- | ---- |
| `subscription-manager` | 0.1.0   | CC35ZGUDYXYVX3JLP4WK5SS4WTTY45ZLI6V53YY7DFOTJACCK5BTN56D | 4d252cfa017a11bb4f24807b887fa4671ce0bc5a88b3a716df8f25e29cc6ce59 | 2026-10-09 |

## Recording a new deployment

1. Build and optimize: `./scripts/build.sh subscription-manager`
2. Deploy: `./scripts/deploy.sh subscription-manager testnet <account>`
3. Add a new row above with the printed contract ID, the WASM hash and the date.
