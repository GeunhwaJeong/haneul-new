---
'@haneullabs/haneul.js': minor
---

Removed dependency on @open-rpc/client-js and replaced it with standard fetch and WebSocket based APIs

If you are using the `subscribeEvent` or `subscribeTransaction` in environments that do not support the `WebSocket` api natively (This will be true for most versions of Node.js) you will need to provide a WebSocket implementation when creating your HaneulClient. You can either use a global polyfill for the WebSocket class, or pass a compatible WebSocket implementation into HaneulHTTPTransport (eg, using the `ws` package)

```typescript
import { getFullnodeUrl, HaneulClient, HaneulHTTPTransport } from '@haneullabs/haneul.js/client';
import { WebSocket } from 'ws';

new HaneulClient({
	transport: new HaneulHTTPTransport({
		url: getFullnodeUrl('mainnet'),
		// The typescript definitions may not match perfectly, casting to never avoids these minor incompatibilities
		WebSocketConstructor: WebSocket as never,
	}),
});
```
