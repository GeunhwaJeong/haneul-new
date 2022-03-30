---
title: Contributing to Haneul
---

This page describes how to add to Haneul. If you merely need to get the Haneul binaries, follow [Install Haneul](../build/install.md).

## File issues

Report bugs and make feature requests in the [Haneul GitHub](https://github.com/GeunhwaJeong/haneul/issues) repository
using the [Template for Reporting Issues](https://github.com/GeunhwaJeong/haneul/blob/main/ISSUES.md).

## Provide docs feedback

Send us documentation fixes or requests for improvement at:
doc@haneul-labs.com

You may also suggest changes to the docs directly in GitHub right here using the **Source Code** link below.

Simply edit the file in question and generate a pull request. We will get back to you shortly.

## Download and learn Haneul

In order to obtain the Haneul source code, clone the Haneul repository:

```shell
git clone https://github.com/GeunhwaJeong/haneul.git
```

You can start exploring Haneul's source code by looking into the following primary directories:

* [haneul](https://github.com/GeunhwaJeong/haneul/tree/main/haneul) - the Haneul binaries (`wallet`, `haneul-move`, and more)
* [haneul_programmability](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_programmability) - Haneul's Move language integration also including games and other Move code examples for testing and reuse
* [haneul_core](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_core) - authority server and Haneul Gateway
* [haneul_types](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_types) - coins, gas, and other object types
* [explorer](https://github.com/GeunhwaJeong/haneul/tree/main/explorer) - object explorer for the Haneul network
* [network_utils](https://github.com/GeunhwaJeong/haneul/tree/main/network_utils) - networking utilities and related unit tests

## Send pull requests

Start by creating your own fork of the repo:
```bash
gh fork https://github.com/GeunhwaJeong/haneul.git # or alternatively, clone your fork
cargo install --path haneul/haneul # put Haneul CLI's in your PATH
cd haneul
cargo build --all --all-targets # check that build works
cargo test # check that tests pass
```

To submit your pull request:

1. Make your changes in a descriptively named branch.
2. If you have added code that should be tested, add unit tests.
3. Ensure your code builds and passes the tests: `cargo test`
4. Make sure your code passes the linters and autoformatter: `cargo clippy --all --all-targets && cargo fmt --all`
5. If you have made changes to APIs, update the relevant documentation, and build and test the developer site.
6. Run `git push -f origin <branch_name>`, then open a pull request from the Haneul GitHub site.

## Further reading

* Learn [about Haneul Labs](https://haneul-labs.com/) the company on our public site.
* Read the [Haneul Smart Contract Platform](../../paper/haneul.pdf) white paper.
* Implementing [logging](../contribute/observability.md) in Haneul to observe the behavior of your development.
* Find related [research papers](../contribute/research-papers.md).
* See and adhere to our [code of conduct](../contribute/code-of-conduct.md).
