---
title: Install Haneul to Build
---

Learn how to install and configure Haneul to develop smart contracts on the Haneul blockchain.

Before you install Haneul, you need to install some prerequisite tools and configure your development environment.

The steps to install Haneul include:

1. Install [prerequisites](#prerequisites) for your operating system.
1. Install the [Haneul binaries](#install-haneul-binaries).
1. Configure an [Integrated Development Environment (IDE)](#integrated-development-environment).
1. Request [HANEUL tokens](#haneul-tokens) to evaluate Devnet and Haneul Wallet.
1. Optionally, download the [source code](#source-code) to have local
   access to examples and contribute to Haneul.

## Haneul repository

The Haneul repository includes two primary branches, `devnet` and `main`.

 * The `devnet` branch includes the latest stable build of Haneul. Choose the `devnet` branch if you want to build or test on Haneul. If you encounter an issue or find a bug, it may already be fixed in the `main` branch. To submit a pull request (PR), you should push commits to your fork of the `main` branch.
 * The `main` branch includes the most recent changes and updates. Use the `main` branch if you want to contribute to the Haneul project. The `main` branch may include unreleased changes, or introduce changes that cause issues in apps created using an earlier version.

## Documentation in the Haneul repository

The `main` and `devnet` branches of the Haneul repository contain the relevant documentation for each branch. A version toggle on the documentation site enables you to switch between `main` branch content (labeled **Latest build**) and `devnet` branch content (labeled **Devnet**). Make sure the toggle is set to **Devnet** for to learn how to install, configure, and build on Haneul. The content in **Latest build** is useful to learn about potential updates to Haneul, but the features and functionality described might not ever become available in the `devnet` branch.  

## Supported operating systems

Haneul supports the following operating systems, beginning with the versions indicated.

* Linux - Ubuntu version 20.04 (Bionic Beaver)
* macOS - macOS Monterey
* Microsoft Windows - Windows 11

## Prerequisites

Install the prerequisites and tools you need to work with Haneul. Click a marker in the table to jump to the relevant section.

| Package/OS | Linux  | macOS | Windows 11 |
| --- | :---: | :---: | :---: |
| cURL | [X](#curl) | [X](#curl-1) | [X](#curl-2) |
| Rust and Cargo | [X](#rust-and-cargo) | [X](#rust-and-cargo) | [X](#rust-and-cargo) |
| Git CLI | [X](#git-cli) | [X](#git-cli-1) | [X](#git-cli-2) |
| CMake | [X](#cmake) | [X](#cmake-1) | [X](#cmake-2) |
| libssl-dev | [X](#libssl-dev) | | |
| libclang-dev | [X](#libclang-dev) | | |
| Brew | | [X](#brew) | |
| C++ build tools | | | [X](#additional-tools-for-windows) |
| LLVM Compiler | | | [X](#additional-tools-for-windows) |


### Rust and Cargo

Haneul requires Rust and Cargo on all supported operating systems. Some operating systems require cURL to download Rust and Cargo, so check the relevant prerequisite section to install cURL first, if necessary.

Use the following command to install Rust and Cargo on macOS or Linux:
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows 11 users can use the [Rust installer](https://www.rust-lang.org/tools/install) available on the Rust website. The installer detects if you do not have the required C++ build tools and prompts you to install. Select the option that best defines your environment and follow the instructions in the install wizard. 

For additional installation options, see [Install Rust](https://www.rust-lang.org/tools/install) on the Rust website.

Haneul uses the latest version of Cargo to build and manage dependencies. See the [Cargo installation](https://doc.rust-lang.org/cargo/getting-started/installation.html) page on the Rust website for more information.

Use the following command to update Rust:

```shell
$ rustup update stable
```

After you install Rust, proceed to the prerequisites for your operating system.
 * [Linux prerequisites](#linux-prerequisites)
 * [macOS prerequisites](#macos-prerequisites)
 * [Windows prerequisites](#windows-prerequisites)

## Linux prerequisites 

> **Note:** The Linux instructions assume a distribution that uses the APT package manager. Adapt the instructions as needed for other package managers.

Install the prerequisites listed in this section. You should make sure that your system has the latest version of `apt-get`. Use the following command to update `apt-get`:

```shell
$ sudo apt-get update
```

### cURL

Install cURL with the following command:
```shell
$ sudo apt install curl
```

Verify that cURL installed correctly with the following command:
```shell
$ curl --version
```

### Git CLI

Run the following command to install Git, including the [Git CLI](https://cli.github.com/):

```shell
$ sudo apt-get install git-all
```

For more information, see [Install Git on Linux](https://github.com/git-guides/install-git#install-git-on-linux) on the GitHub website.

### CMake

Install CMake using the instructions at [Installing CMake](https://cmake.org/install/) on the CMake website.

### libssl-dev

Use the following command to install `libssl-dev`:

```shell
$ sudo apt-get install libssl-dev
```

### libclang-dev

Use the following command to install `libclang-dev`:

```shell
$ sudo apt-get install libclang-dev
```

Proceed to [Install Haneul binaries](#binaries) to continue installing Haneul.


## macOS prerequisites

macOS includes a version of cURL you can use to install Brew. Use Brew to install other tools, including a newer version of cURL.

### Brew

Use the following command to install [Brew](https://brew.sh/):
```shell
$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### cURL

Use the following command to update the default [cURL](https://curl.se) on macOS:
```shell
$ brew install curl
```

### CMake

Use the following command to install CMake:
```shell
$ brew install cmake
```

### Git CLI

Use the following command to install Git:
```shell
$ brew install git
```

After installing Git, download and install the [Git command line interface](https://git-scm.com/download/).

Proceed to [Install Haneul binaries](#install-haneul-binaries) to continue installing Haneul.


## Windows prerequisites

Install the following prerequisites to work with Haneul on Microsoft Windows 11.

### cURL

Windows 11 ships with a Microsoft version of [cURL](https://curl.se/windows/microsoft.html) already installed. If you want to use the curl project version instead, download and install it from [https://curl.se/windows/](https://curl.se/windows/).

### Git CLI

Download and install the [Git command line interface](https://git-scm.com/download/).

### CMake

Download and install [CMake](https://cmake.org/download/) from the CMake website.

### Protocol Buffers

Download [Protocol Buffers](https://github.com/protocolbuffers/protobuf/releases) (protoc-xx.x-win32.zip or protoc-xx.x-win64.zip) and add the \bin directory to your Windows PATH environment variable.

### Additional tools for Windows

Haneul requires the following additional tools on computers running Windows.

 * For Windows on ARM64 only - [Visual Studio 2022 Preview](https://visualstudio.microsoft.com/vs/preview/).
 * [C++ build tools](https://visualstudio.microsoft.com/downloads/) are required to [install Rust](#rust-and-cargo), so you should already have these installed if you followed these instructions.
 * The [LLVM Compiler Infrastructure](https://releases.llvm.org/).

>**Tip:** The installation progress might appear hanging if the `cmd.exe` window loses focus;
>press the `enter` key in the command prompt to fix the issue.

>**Known issue:** The `haneul console` command does not work in PowerShell.


## Install Haneul binaries

With Cargo installed, use the following command to install Haneul binaries:

```shell
$ cargo install --locked --git https://github.com/GeunhwaJeong/haneul.git --branch devnet haneul
```

The command installs the following Haneul components in `~/.cargo/bin`:
* [`haneul`](cli-client.md) - The Haneul CLI tool contains subcommands for enabling `genesis` of validators and accounts, starting the Haneul network, and [building and testing Move packages](move/index.md), as well as a [client](cli-client.md) for interacting with the Haneul network.

If the previous command fails, make sure you have the latest version of Rust installed:

```
rustup update stable
source "$HOME/.cargo/env"
```

### macOS and Linux

Confirm the binaries are installed with `.cargo/bin` appearing in your PATH variable:
```
$ echo $PATH
```
### Windows

Confirm the binaries are installed with `.cargo\bin` appearing in your PATH variable:
```
$ echo %PATH%
```
Use the `--help` flag to access helpful information for any of these binaries.

> **Important:** Make sure your entire toolchain stays up-to-date. If you encounter issues building and installing the Haneul binaries, update all packages and re-install.

## Integrated development environment

The recommended IDE for Move development is [Visual Studio Code](https://code.visualstudio.com/) with the move-analyzer extension. Follow the Visual Studio Marketplace instructions to install the [move-nalyzer extension](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer), passing `address20` using the `--features` flag:

```shell
$ cargo install --git https://github.com/move-language/move move-analyzer --features "address20"
```

See more [IDE options](https://github.com/GeunhwaJeong/awesome-move#ides) in the [Awesome Move](https://github.com/GeunhwaJeong/awesome-move) documentation.

## HANEUL tokens

To [experiment with Devnet](../build/devnet.md) or [use the Haneul Wallet browser extension](../explore/wallet-browser.md), add HANEUL tokens to your account address. 

To request HANEUL test tokens from the browser extension:

1. Use the Haneul Wallet browser extension to open your wallet.
2. Click the **Request Haneul Devnet HANEUL Tokens** button.

To request HANEUL test tokens in Discord:

1. Join the [Haneul Discord](https://discord.com/invite/haneul) If you haven’t already.
1. Identify your address through either the Haneul Wallet browser extension or by running the following command and electing to connect to a Haneul RPC server if prompted:
   ```shell
   $ haneul client active-address
   ```
1. Request tokens in the [#devnet-faucet](https://discord.com/channels/916379725201563759/971488439931392130) channel using the syntax: `!faucet <YOUR_ADDRESS>`, for example:
      ```shell
      !faucet 0xd72c2c90ed9d923cb0ed2ca91db5be9e1c9b5ccb
      ```
1. A bot on the channel distributes tokens to you automatically.

## Source code

View the Haneul repository on GitHub:
https://github.com/GeunhwaJeong/haneul

Clone the Haneul repository:

```shell
$ git clone https://github.com/GeunhwaJeong/haneul.git --branch devnet
```

The following primary directories offer a good starting point for exploring Haneul's source code:
* [haneul](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul) - the Haneul CLI binary
* [haneul_programmability](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_programmability) - Haneul's Move language integration also including games and other Move code examples for testing and reuse
* [haneul_core](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-core) - authority server and Haneul Gateway
* [haneul-types](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-types) - coins, gas, and other object types
* [explorer](https://github.com/GeunhwaJeong/haneul/tree/main/apps/explorer) - object explorer for the Haneul network
* [haneul-network](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-network) - networking interfaces

## Rustdoc

See the Rust [Crates](https://doc.rust-lang.org/rust-by-example/crates.html) in use at:
* https://haneullabs.github.io/haneul/ - the Haneul blockchain
* https://haneullabs.github.io/narwhal/ - the Narwhal and Bullshark consensus engine
* https://haneullabs.github.io/haneullabs-infra/ - Haneul Labs infrastructure

## Help

To contribute updates to Haneul code, [send pull requests](../contribute/index.md#send-pull-requests) our way.

> **Note:** The previous `git clone` command syncs with the `devnet` branch, which makes sure the source code is compatible with our Devnet. If you want to run a network locally using the latest version and don't need to interact with our Devnet, you should switch to `main` branch.
 
## Next steps

Continue your journey through:

* [Smart Contracts with Move](move/index.md)
* [Haneul client Quick Start](cli-client.md)
* [RPC Server API](json-rpc.md)
* [End-to-End tutorial](../explore/tutorials.md)
