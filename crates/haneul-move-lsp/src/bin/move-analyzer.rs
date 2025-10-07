// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use move_analyzer::analyzer;
use move_compiler::editions::Flavor;
use haneul_move_build::{implicit_deps, HaneulPackageHooks};
use haneul_package_management::system_package_versions::latest_system_packages;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[derive(Parser)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    rename_all = "kebab-case",
    author,
    version = VERSION,
)]
struct App {}

fn main() {
    App::parse();
    let haneul_implicit_deps = implicit_deps(latest_system_packages());
    let flavor = Flavor::Haneul;
    let haneul_pkg_hooks = Box::new(HaneulPackageHooks);
    analyzer::run(haneul_implicit_deps, Some(flavor), Some(haneul_pkg_hooks));
}
