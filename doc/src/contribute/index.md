---
title: Contributing to Haneul
---

This page describes how to add to Haneul. If you merely need to get the Haneul binaries, follow [Install Haneul](../build/install.md).

Find answers to common questions in our [FAQ](../contribute/faq.md). Read other sub-pages in this section for ways to contribute to Haneul.

## See our roadmap

Haneul is evolving quickly. See our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) for the
overall status of Haneul, including timelines for launching devnet, testnet, and mainnet.

## Join the community

To connect with the Haneul community, join our [Discord](https://discord.gg/haneul).

## File issues

Report bugs and make feature requests in the [Haneul GitHub](https://github.com/GeunhwaJeong/haneul/issues) repository
using the [Template for Reporting Issues](https://github.com/GeunhwaJeong/haneul/blob/main/ISSUES.md).

## Provide docs feedback

Send us documentation fixes or requests for improvement at:
doc@haneul-labs.com

You may also suggest changes to the docs directly in GitHub right here using the **Source Code** link below.

Simply edit the file in question and generate a pull request. We will get back to you shortly.

## Download Haneul

In order to obtain the Haneul source code, follow the steps to download (`git clone`) the `haneul` repository
at [Install Haneul](../build/install.md#source-code).

## Send pull requests

Start by creating your own fork of the repo:
```bash
$ gh repo fork https://github.com/GeunhwaJeong/haneul.git # or alternatively, clone your fork
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
