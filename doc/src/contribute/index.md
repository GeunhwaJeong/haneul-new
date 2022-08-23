---
title: Contributing to Haneul
---

This page describes how to add to Haneul. If you merely need to get the Haneul binaries, follow [Install Haneul](../build/install.md).

This site is available in two versions in the menu at top left: the default and stable [Devnet](https://docs.haneul.io/devnet/learn) branch and the [Latest build](https://docs.haneul.io/learn) upstream `main` branch. Use the `devnet` version for app development on top of Haneul. Use the Latest build `main` branch for contributing to the Haneul blockchain as described on this page. Always check and submit fixes to the `main` branch.

Find answers to common questions in our [FAQ](../contribute/faq.md). Read other sub-pages in this section for ways to contribute to Haneul.

## See our roadmap

Haneul is evolving quickly. See our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) for the
overall status of Haneul, including timelines for launching Devnet, Testnet, and Mainnet.

## Join the community

To connect with the Haneul community, join our [Discord](https://discord.gg/haneul).

## File issues

Report bugs and make feature requests in the [Haneul GitHub](https://github.com/GeunhwaJeong/haneul/issues) repository
using the [Template for Reporting Issues](https://github.com/GeunhwaJeong/haneul/blob/main/ISSUES.md).

## Help docs

### Ideas
Send ideas to:
doc@haneul-labs.com

Or report them in the [#docs](https://discord.com/channels/916379725201563759/1001562806862233701) channel of Discord.

### Issues
And file documentation fixes or requests for improvement at:
https://github.com/GeunhwaJeong/haneul/issues/new/choose

Select the **Haneul Doc Bug** template, adjust fields, and describe the issue.

### Updates

You may also make changes to the docs directly in GitHub right here using the **Source Code** link below. Make sure you are in the `main` rather than `devnet` branch by being on the [Latest build](https://docs.haneul.io/learn) upstream view as described at the top of this page.

Simply edit the file in question and generate a pull request. You may even use our [Haneul doc templates](https://github.com/GeunhwaJeong/haneul/tree/main/doc/template) to create overviews and procedures (uses).

Then send your work our way. We will get back to you shortly.

## Download Haneul

In order to obtain the Haneul source code, follow the steps to download (`git clone`) the `haneul` repository
at [Install Haneul](../build/install.md#source-code).

And see the Rust [Crates](https://doc.rust-lang.org/rust-by-example/crates.html) in use at:
* https://haneullabs.github.io/haneul/ - the Haneul blockchain
* https://haneullabs.github.io/narwhal/ - the Narwhal and Bullshark consensus engine
* https://haneullabs.github.io/haneullabs-infra/ - Haneul Labs infrastructure

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
