---
title: Install Haneul
---

Haneul is written in Rust, and we are using Cargo to build and manage the
dependencies.  As a prerequisite, you will need to [install
cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
version 1.59.0 or higher in order to build and install Haneul on your machine.

If you'd like to install only Haneul binaries (`haneul`, `wallet`, and
`haneul-move`), use the following command:

```shell
cargo install --git ssh://git@github.com/GeunhwaJeong/haneul.git
```

Alternatively, clone the Haneul [Haneul
GitHub](https://github.com/GeunhwaJeong/haneul) repository and then `cargo
install` with the repository clone:

```shell
git clone https://github.com/GeunhwaJeong/haneul.git
cargo install --path haneul
```

In both cases, this will install `haneul`, `wallet`, and `haneul-move`
binaries in a `~/.cargo/bin` directory that can be executed directly.
