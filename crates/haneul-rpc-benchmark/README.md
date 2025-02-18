# haneul-rpc-benchmark: Benchmarking Tool for HANEUL RPC Performance

`haneul-rpc-benchmark` is a benchmarking utility designed to measure performance across different RPC access methods in Haneul:
- Direct database reads
- JSON RPC endpoints 
- GraphQL queries

## Usage Examples
Run benchmarks with:
```
# Direct database queries:
cargo run --bin haneul-rpc-benchmark direct --db-url postgres://postgres:postgres@localhost:5432/haneul --concurrency 10  --duration-secs 10

# JSON RPC endpoints:
cargo run --bin haneul-rpc-benchmark jsonrpc --endpoint http://127.0.0.1:9000

# GraphQL queries:
cargo run --bin haneul-rpc-benchmark graphql --endpoint http://127.0.0.1:9000/graphql
```
