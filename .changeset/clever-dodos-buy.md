---
"@haneullabs/haneul.js": minor
---

Change functions in transactions.ts of ts-sdk such that: `getTotalGasUsed` and `getTotalGasUsedUpperBound` of ts-sdk return a `bigint`,fields of `gasCostSummary` are defined as `string`, `epochId` is defined as `string`. In `haneul-json-rpc` the corresponding types are defined as `BigInt`. Introduce `HaneulEpochId` type to `haneul-json-rpc` types that is a `BigInt`.
