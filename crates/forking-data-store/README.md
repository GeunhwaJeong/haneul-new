# Haneul Data for Forking

Multi-tier caching data store for Haneul blockchain data.

This crate provides a flexible data store abstraction for retrieving and caching
Haneul blockchain data (transactions, epochs, objects). The stores are loosely modeled
after the GraphQL schema in `crates/haneul-indexer-alt-graphql/schema.graphql`.

## Capability Traits

- `TransactionStore` / `TransactionStoreWriter`
- `EpochStore` / `EpochStoreWriter`
- `ObjectStore` / `ObjectStoreWriter`
- `CheckpointStore` / `CheckpointStoreWriter`

## Version Queries

The `ObjectStore` trait supports three query modes via `VersionQuery`:

- `Version(v)` - Request object at exact version `v`
- `RootVersion(v)` - Request object at version `<= v` (for dynamic field roots)
- `AtCheckpoint(c)` - Request object as it existed at checkpoint `c`

## Network Configuration

Use the `Node` enum to configure which network to connect to:

```rust
use haneul_data_store::Node;

let mainnet = Node::Mainnet;
let testnet = Node::Testnet;
let custom = Node::Custom("https://my-rpc.example.com".to_string());
```
