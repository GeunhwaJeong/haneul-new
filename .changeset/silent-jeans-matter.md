---
'@haneullabs/haneul': minor
'@haneullabs/zklogin': minor
---

All functionality from `@haneullabs/zklogin` has been moved to `@haneullabs/haneul/zklogin`

For most methods, simply replace the `@haneullabs/zklogin` import with `@haneullabs/haneul/zklogin`

2 Methods require one small additional change:

`computeZkLoginAddress` and `jwtToAddress` have new `legacyAddress` flags which must be set to true for backwards compatibility:

```diff
- import { computeZkLoginAddress, jwtToAddress } from '@haneullabs/zklogin';
+ import { computeZkLoginAddress, jwtToAddress } from '@haneullabs/haneul/zklogin';

  const address = jwtToAddress(
   jwtAsString,
   salt,
+  true
  );
  const address = computeZkLoginAddress({
	claimName,
	claimValue,
	iss,
	aud,
	userSalt: BigInt(salt),
+	legacyAddress: true,
  });
```
