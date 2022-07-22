---
title: Install Haneul
---

Welcome to the Haneul development environment! Note, this site is built from the upstream `main`
branch and therefore will contain updates not yet found in `devnet`. The instructions here
recommend use of `devnet` as the latest stable release. To [contribute to Haneul](../contribute/index.md),
instead use the `main` branch.

## Summary

To immediately get started using Haneul:

1. Meet the [prerequisites](#prerequisites).
1. Install the [binaries](#binaries).
1. Configure an [Integrated Development Environment (IDE)](#integrated-development-environment).
1. Request [HANEUL tokens](#haneul-tokens) to evaluate Devnet and Haneul Wallet
1. Optionally, download the [source code](#source-code) to have local
   access to examples and modify Haneul itself.

> **Tip:** Assuming you on macOS or Linux, have `curl`, Rust Cargo, the `git` command, and a GitHub account
> (see [Prerequisites](#prerequisites)), you can download the `haneul-setup.sh` script
> and run it to conduct all of the setup below, **including removal of any existing
> haneul assets**. To use it, run these commands in a terminal:
> ```shell
> $ curl https://raw.githubusercontent.com/HaneulLabs/haneul/main/doc/utils/haneul-setup.sh -o haneul-setup.sh
> chmod 755 haneul-setup.sh
> ./haneul-setup.sh
> ```

## Supported OSes

The following operating systems (OSes) have been tested and are supported for
running Haneul:

* [Linux](#linux-specific) - Ubuntu version 18.04 (Bionic Beaver)
* [macOS](#macOS-specific) - macOS Monterey
* [Microsoft Windows](#microsoft-windows-specific) - Windows 11

First install the [General packages](#general-packages) (plus [Brew](#brew) if on macOS), then install the OS-specific packages.

## Prerequisites

At a minimum, you should have a machine capable of installing command line tools (namely, a terminal).
First install the packages outlined this section. Then add the additional dependencies
below for your operating system.

Here are the packages required by operating system:

|Package/OS |Linux  | macOS| Windows 11|
--- | :---: | :---:| :---:|
|Curl|X|X|X|
|Rust|X|X|X|
|Git CLI|X|X|X|
|CMake|X|X|X|
|libssl-dev|X| | |
|libclang-dev|X| | |
|Brew| |X| |
|C++ build tools| | |X|
|LLVM Compiler| | |X|
|Haneul|X|X|X|

Follow the instructions below to install them. Then install the Haneul [binaries](#binaries).

Finally, if you will be altering Haneul itself, also obtain the [Haneul source code](#source-code).
For simplicity, we recommend installing in `~/haneul` or using an environment variable.

>**Important:** You will need to restart your command prompt after installing these prerequisites
>for them to be available in your environment.

### Brew
In macOS, first install [Brew](https://brew.sh/) to install other packages:
```shell
$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### General packages

Ensure each of the packages below exist on each OS:

#### Curl
Confirm that you can run the `curl` command to download dependencies.

See whether you already have curl installed by running:

```shell
$ which curl
```

And if you see no output path, install it with:

*Linux*
```shell
$ sudo apt install curl
```

*macOS*
```shell
$ sudo brew install curl
```

*Microsoft Windows*
Download and install from: https://curl.se/windows/

#### Rust
Haneul is written in Rust, and we are using the latest version of the
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) toolchain
to build and manage the dependencies. You will need Cargo to build and install Haneul on your machine.

Get [rustup](https://rust-lang.github.io/rustup/)
to install Rust and Cargo:

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then update the packages with:

```shell
$ rustup update stable
```

> **Warning:** If you run into issues, you may un-install Rust and Cargo with:
> ```shell
> $ rustup self uninstall
> ```
> And then start the Rust install over.
> For more details, see:
> https://www.rust-lang.org/tools/install

#### Git CLI

Download and install the [`git` command line interface](https://git-scm.com/download/)
for your operating system.

#### CMake

Get the `cmake` command to build Haneul:

*Linux*
```shell
$ sudo apt install cmake
```

*macOS*
```shell
$ sudo brew install cmake
```
*Microsoft Windows*
Download and install from: https://cmake.org/download/

If you run into issues, follow this detailed [CMake Installation](https://riptutorial.com/cmake/example/4459/cmake-installation) tutorial.

### Linux-specific

In Linux, install:

libssl-dev
```shell
$ sudo apt install libssl-dev
```

libclang-dev
```shell
$ sudo apt install libclang-dev
```

### macOS-specific

In macOS, other than the aforementioned [Brew](#brew) package manager, the general prerequisites are sufficient.

### Microsoft Windows-specific

In Microsoft Windows, also install:

[C++ build tools](https://visualstudio.microsoft.com/downloads/)

The [LLVM Compiler Infrastructure](https://releases.llvm.org/)

>**Tip:** The installation progress might appear hanging if the `cmd.exe` window loses focus;
>press the `enter` key in the command prompt fix the issue.

>**Known Issue:** The `haneul console` command does not work in PowerShell.

## Binaries

To develop in Haneul, you will need the Haneul binaries. After installing `cargo`, run:

```shell
$ cargo install --locked --git https://github.com/GeunhwaJeong/haneul.git --branch "devnet" haneul haneul-gateway
```

This will put the following binaries in your `PATH` (ex. under `~/.cargo/bin`) that provide these command line interfaces (CLIs):
* haneul - The Haneul CLI tool contains subcommands for enabling `genesis` of validators and accounts, starting the Haneul network, and [building and testing Move packages](move/index.md), as well as a [client](cli-client.md) for interacting with the Haneul network.
* [`rpc-server`](json-rpc.md) - run a local Haneul gateway service accessible via an RPC interface.

Confirm the installation with:
#### macOS and Linux
```
$ echo $PATH
```
#### Windows
```
$ echo %PATH%
```
And ensure the `.cargo/bin` directory appears. Access the help for any of these binaries by passing the `--help` argument to it.

## Integrated Development Environment
For Move development, we recommend the [Visual Studio Code (vscode)](https://code.visualstudio.com/) IDE with the Move Analyzer language server plugin installed:

```shell
$ cargo install --git https://github.com/move-language/move move-analyzer --features "address20"
```

Then follow the Visual Studio Marketplace instructions to install the [Move Analyzer extension](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer). (The `cargo install` command for the language server is broken there; hence, we include the correct command above.)

See more [IDE options](https://github.com/GeunhwaJeong/awesome-move#ides) in the [Awesome Move](https://github.com/GeunhwaJeong/awesome-move) docs.

## HANEUL tokens

To [experiment with Devnet](../explore/devnet.md) or [use the Haneul Wallet Browser Extension](../explore/wallet-browser.md), you will need HANEUL tokens. These coins have no financial value and will disappear each time we reset the network.

To request HANEUL test tokens:

1. Join the [Haneul Discord](https://discord.com/invite/haneul) If you haven’t already.
1. Identify your address through either the Haneul Wallet Browser Extension or by running the command:
   ```shell
   $ haneul client active-address
   ```
1. Request tokens in the [#devnet-faucet](https://discord.com/channels/916379725201563759/971488439931392130) channel using the syntax: `!faucet <YOUR_ADDRESS>`, for example:
      ```shell
      !faucet 0xd72c2c90ed9d923cb0ed2ca91db5be9e1c9b5ccb
      ```
1. A bot on the channel will distribute tokens to you automatically.

## Source code

If you need to download and understand the Haneul source code:
https://github.com/GeunhwaJeong/haneul

Clone the Haneul repository:

```shell
$ git clone https://github.com/GeunhwaJeong/haneul.git --branch devnet
```

You can start exploring Haneul's source code by looking into the following primary directories:
* [haneul](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul) - the Haneul CLI binary
* [haneul_programmability](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_programmability) - Haneul's Move language integration also including games and other Move code examples for testing and reuse
* [haneul_core](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-core) - authority server and Haneul Gateway
* [haneul-types](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-types) - coins, gas, and other object types
* [explorer](https://github.com/GeunhwaJeong/haneul/tree/main/explorer) - object explorer for the Haneul network
* [haneul-network](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-network) - networking interfaces

## Rustdoc

See the Rust [Crates](https://doc.rust-lang.org/rust-by-example/crates.html) in use at:
* https://haneullabs.github.io/haneul/ - the Haneul blockchain
* https://haneullabs.github.io/narwhal/ - the Narwhal and Tusk consensus engine
* https://haneullabs.github.io/haneullabs-infra/ - Haneul Labs infrastructure

## Help

To contribute updates to Haneul code, [send pull requests](../contribute/index.md#send-pull-requests) our way.

> NOTE: the above `git clone` command syncs with the `devnet` branch, which makes sure the source code is compatible with our Devnet. If you want to run network locally using the latest version and don't need to interact with our Devnet, you should switch to `main` branch.
 
## Next steps

Continue your journey through:

* [Smart Contracts with Move](move/index.md)
* [Haneul client Quick Start](cli-client.md)
* [RPC Server API](json-rpc.md)
* [End-to-End tutorial](../explore/tutorials.md)
