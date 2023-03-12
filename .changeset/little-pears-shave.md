---
"@haneullabs/haneul.js": minor
---

Update `executeTransaction` and `signAndExecuteTransaction` to take in an additional parameter `HaneulTransactionResponseOptions` which is used to specify which fields to include in `HaneulTransactionResponse` (e.g., transaction, effects, events, etc). By default, only the transaction digest will be included.
