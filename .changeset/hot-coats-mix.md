---
"@haneullabs/haneul.js": minor
---

Remove `generateTransactionDigest`. Use one of the following instead: `signer.getTransactionDigest`, `Transaction.getDigest()` or `TransactionDataBuilder.getDigestFromBytes()` instead.
