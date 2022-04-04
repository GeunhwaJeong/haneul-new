# Roadmap

Currently, builders can:
* Write and test Move smart contracts
* Spin up a local Haneul network
* Publish and run Move smart contracts on a local network

In the coming months, we will release:
1. A public devnet that allows Haneul devs to do all of the above on a shared network powered by Haneullabs-operated authorities
2. A public testnet that onboards a diverse set of non-Haneullabs authorities to the network
3. A public mainnet with real assets and production applications!

A more fine-grained description of the upcoming features and improvements to the Haneul codebase follows.

## Ongoing work and upcoming features

### Internal devnet
* Constantly running Haneul network
* Productionizing network stack
* Benchmarking throughput and latency in various configurations

### Protocol stabilization

* Implementing reconfiguration and staking
* Ledger and state checkpoints
* Aligning Gateway Service Rust/REST/wallet APIs
* Finalizing REST data model (aka HaneulJSON)
* Finalizing core data types
* Authority "Follower" APIs to support replicas
* Integration of shared objects and consensus
* Selecting principled gas costs
* Event indexing hints

### SDK and ecosystem
* Block explorer
* Key management and wallet prototypes
* Support package publishing via REST API
* More informative error messages
* More convenient APIs that hide gas object selection and nested object authentication

### Move development improvements
* Allow objects used in authentication, but not passed to entrypoints
* Explicit syntax and compiler enforcement for entrypoints
* Adding Move Prover specs to the Haneul framework and verifying in continuous integration (CI)
