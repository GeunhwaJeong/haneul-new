# Create a Coin

Publishing a coin is Haneul is almost as simple as publishing a new type. However it is a bit tricky as it requires using a Witness pattern.

```move
{{#include ../../examples/sources/samples/coin.move:4:}}
```

The `Coin<T>` is a generic implementation of a Coin on Haneul. Owner of the `TreasuryCap` gets control over the minting and burning of coins. Further transactions can be sent directly to the `haneul::coin::Coin` with `TreasuryCap` object as authorization.
