---
title: Install Haneul
---

Haneul is written in Rust, and we are using Cargo to build and manage the
dependencies.  As a prerequisite, you will need to [install
Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
version 1.59.0 or higher in order to build and install Haneul on your machine.

## Set up

Although you may create whatever directory structure you desire, both our
[Wallet Quick Start](wallet.md) and [end-to-end tutorial](../explore/tutorials.md)
assume Haneul is installed in a directory found by a `$HANEUL_ROOT` environment variable.

To set this up, run the following commands and substitute in your path and
desired directory name:

```shell
mkdir some-dir
export HANEUL_ROOT=/path/to/some-dir
```

## Download

Navigate to your desired install location, for example:

```shell
cd "$HANEUL_ROOT"
```

### Binaries only

If you'd like to install only Haneul binaries (`haneul`, `wallet`,
`haneul-move`, and `rest_server`), use the following command:

```shell
cargo install --git https://github.com/GeunhwaJeong/haneul.git
```

### Whole repository

Alternatively, clone the [Haneul
GitHub](https://github.com/GeunhwaJeong/haneul) repository and then `cargo
install` with the repository clone:

```shell
git clone https://github.com/GeunhwaJeong/haneul.git
cargo install --path haneul/haneul
```

## Use

Either method will install `haneul`, `wallet`, `haneul-move`, and `rest_server`
binaries in a `~/.cargo/bin` directory that can be executed directly.

## Next steps

Continue your journey through:

* [Smart Contracts with Move](move.md)
* [Wallet Quick Start](wallet.md)
* [REST Server API](rest-api.md)
* [End-to-End tutorial](../explore/tutorials.md)
