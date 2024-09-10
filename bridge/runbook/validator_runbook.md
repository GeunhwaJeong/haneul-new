---
title: Haneul Bridge Validator Runbook
---

## Prerequisite

Install `haneul`, `haneul-bridge-cli` binaries:
```bash
# install from tip of `main`
cargo install --locked --git "https://github.com/GeunhwaJeong/haneul.git" haneul haneul-bridge-cli
# install with a commit sha
cargo install --locked --git "https://github.com/GeunhwaJeong/haneul.git" --rev {SHA} haneul haneul-bridge-cli
```

## Committee Registeration

### Prepare for Metadata

The required metadata includes two things:
* `BridgeAuthorityKey`, a ECDSA key to sign messages. Since this is a hot key that is kept in memory, it’s fine to use the following tool to generate one and write to file.
* a REST API URL where the bridge node listens to and serves requests. Example: `https://bridge.example-haneul-validator.io:443`. Make sure the port is correct and the url does not contain any invalid characters, for exmaple quotes.

To create a `BridgeAuthorityKey`, run
```bash
haneul-bridge-cli create-bridge-validator-key {PATH_TO_WRITE}
```
This creates the keypair and writes it to `{PATH_TO_WRITE}`.

*Note: it's highly recommended you create a new key pair in a secure environment (e.g. in the same machine where your node will run) to avoid key compromise.*

### Registration
Once you have both authority key file and REST API URL ready, you can register them by using haneul cli:
```bash
haneul validator register-bridge-committee --bridge-authority-key-path <BRIDGE_AUTHORITY_KEY_PATH> --bridge-authority-url <BRIDGE_AUTHORITY_URL>
```

#### Offline Signing
If your validator account key is kept in cold storage or you want to do offline signing, use flag `--print-only` and provide validator address with `--validator-address`. This prints serialized unsigned transaction bytes, then you can use your preferred signing process to produce signed bytes. Run the following command to execute it:
```bash
haneul client execute-signed-tx
```

#### Update Metadata
Both key and URL are changeable **before the committee is finalized**. If you wish to update metadata, simply rerun `haneul validator register-bridge-committee`.

#### View Registered Metadata
To double check your registered the correct metadata onchain, run
```
haneul-bridge-cli view-bridge-registration --haneul-rpc-url {HANEUL_FULLNODE_URL}
```

### Bridge Node Hardware Requirements

Suggested hardware requirements:
* CPU: 6 physical cores
* Memory: 16GB
* Storage: 200GB
* Network: 100Mbps
