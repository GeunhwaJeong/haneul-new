---
"@haneullabs/haneul.js": minor
---

Update `executeTransaction` and `signAndExecuteTransaction` to take in an additional parameter `HaneulTransactionBlockResponseOptions` which is used to specify which fields to include in `HaneulTransactionBlockResponse` (e.g., transaction, effects, events, etc). By default, only the transaction digest will be included.
