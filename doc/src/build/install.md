---
title: Install Haneul
---

Haneul is written in Rust, and we are using Cargo to build and manage the
dependencies.  As a prerequisite, you will need to [install
Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
version 1.59.0 or higher in order to build and install Haneul on your machine.

## CLIs

After installing `cargo`, run:

```shell
cargo install --git https://github.com/GeunhwaJeong/haneul.git
```

This will put three binaries in your `PATH` (ex. under `~/.cargo/bin`):
* [`haneul-move`](move.md): Build and test Move packages.
* [`wallet`](wallet.md): Run a local Haneul network and gateway service accessible via the wallet CLI. The wallet CLI manage keypairs to sign/send transactions.
* [`rest_server`](rest-api.md): Run a local Haneul network and gateway service accessible via a REST interface.

Confirm the install with:

```
$ echo $PATH
```

And ensure the `.cargo/bin` directory appears.

## Contribute

If you need to download and understand the Haneul source code, follow [contributing to Haneul](../contribute/index.md).

## IDE
For Move development, we recommend the [Visual Studio Code (vscode)](https://code.visualstudio.com/) IDE with the [Move Analyzer](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer) plugin. See more [IDE options](https://github.com/GeunhwaJeong/awesome-move#ides) in the [Awesome Move](https://github.com/GeunhwaJeong/awesome-move) docs.

## Next steps

Continue your journey through:

* [Smart Contracts with Move](move.md)
* [Wallet Quick Start](wallet.md)
* [REST Server API](rest-api.md)
* [End-to-End tutorial](../explore/tutorials.md)
