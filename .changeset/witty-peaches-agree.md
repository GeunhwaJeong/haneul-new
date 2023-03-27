---
"@haneullabs/wallet-adapter-unsafe-burner": minor
"@haneullabs/wallet-standard": minor
---

Add an optional `contentOptions` field to `HaneulSignAndExecuteTransactionOptions` to specify which fields to include in `HaneulTransactionBlockResponse` (e.g., transaction, effects, events, etc). By default, only the transaction digest will be included.
