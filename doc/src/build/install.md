---
title: Install Haneul
---

Haneul is written in Rust, and we are using Cargo to build and manage the
dependencies.  As a prerequisite, you will need to [install
Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
version 1.59.0 or higher in order to build and install Haneul on your machine.

## Binaries

To develop in Haneul, you will need the Haneul binaries. After installing `cargo`, run:

```shell
$ cargo install --git https://github.com/GeunhwaJeong/haneul.git
```

This will put three binaries in your `PATH` (ex. under `~/.cargo/bin`) that provide these command line interfaces (CLIs):
* [`haneul-move`](move.md): Build and test Move packages.
* [`wallet`](wallet.md): Run a local Haneul network and gateway service accessible via the wallet CLI. The wallet CLI manage keypairs to sign/send transactions.
* [`rest_server`](rest-api.md): Run a local Haneul network and gateway service accessible via a REST interface.

Confirm the install with:

```
$ echo $PATH
```

And ensure the `.cargo/bin` directory appears.

## Integrated Development Environment
For Move development, we recommend the [Visual Studio Code (vscode)](https://code.visualstudio.com/) IDE with the Move Analyzer language server plugin installed:

```shell
$ cargo install --git https://github.com/diem/move move-analyzer
```

Then follow the Visual Studio Marketplace instructions to install the [Move Analyzer extension](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer). (The `cargo install` command for the language server is broken there; hence, we include the correct command above.)

See more [IDE options](https://github.com/GeunhwaJeong/awesome-move#ides) in the [Awesome Move](https://github.com/GeunhwaJeong/awesome-move) docs.

## Source code

If you need to download and understand the Haneul source code, clone the Haneul repository:

```shell
$ git clone https://github.com/GeunhwaJeong/haneul.git
```

You can start exploring Haneul's source code by looking into the following primary directories:

* [haneul](https://github.com/GeunhwaJeong/haneul/tree/main/haneul) - the Haneul binaries (`wallet`, `haneul-move`, and more)
* [haneul_programmability](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_programmability) - Haneul's Move language integration also including games and other Move code examples for testing and reuse
* [haneul_core](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_core) - authority server and Haneul Gateway
* [haneul_types](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_types) - coins, gas, and other object types
* [explorer](https://github.com/GeunhwaJeong/haneul/tree/main/explorer) - object explorer for the Haneul network
* [network_utils](https://github.com/GeunhwaJeong/haneul/tree/main/network_utils) - networking utilities and related unit tests

## Next steps

Continue your journey through:

* [Smart Contracts with Move](move.md)
* [Wallet Quick Start](wallet.md)
* [REST Server API](rest-api.md)
* [End-to-End tutorial](../explore/tutorials.md)
