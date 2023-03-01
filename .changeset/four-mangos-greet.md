---
"@haneullabs/wallet-adapter-wallet-standard": minor
"@haneullabs/wallet-adapter-unsafe-burner": minor
"@haneullabs/wallet-adapter-base": minor
"@haneullabs/wallet-adapter-all-wallets": minor
"@haneullabs/wallet-kit-core": minor
"@haneullabs/wallet-standard": minor
"@haneullabs/wallet-kit": minor
"@haneullabs/haneul.js": minor
"@haneullabs/bcs": minor
---

Unified self- and delegated staking flows. Removed fields from `Validator` (`stake_amount`, `pending_stake`, and `pending_withdraw`) and renamed `delegation_staking_pool` to `staking_pool`. Additionally removed the `validator_stake` and `delegated_stake` fields in the `ValidatorSet` type and replaced them with a `total_stake` field.
