// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde_reflection::{Registry, Result, Samples, Tracer, TracerConfig};
use std::{fs::File, io::Write};
use structopt::{clap::arg_enum, StructOpt};
use haneul_types::{error, messages, serialize};

fn get_registry() -> Result<Registry> {
    let mut tracer = Tracer::new(TracerConfig::default());
    let samples = Samples::new();
    // 1. Record samples for types with custom deserializers.
    // tracer.trace_value(&mut samples, ...)?;

    // 2. Trace the main entry point(s) + every enum separately.
    tracer.trace_type::<messages::CertifiedOrder>(&samples)?;
    tracer.trace_type::<error::HaneulError>(&samples)?;
    tracer.trace_type::<serialize::SerializedMessage>(&samples)?;
    tracer.registry()
}

arg_enum! {
#[derive(Debug, StructOpt, Clone, Copy)]
enum Action {
    Print,
    Test,
    Record,
}
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Haneul format generator",
    about = "Trace serde (de)serialization to generate format descriptions for Haneul types"
)]
struct Options {
    #[structopt(possible_values = &Action::variants(), default_value = "Print", case_insensitive = true)]
    action: Action,
}

const FILE_PATH: &str = "haneul_core/tests/staged/haneul.yaml";

fn main() {
    let options = Options::from_args();
    let registry = get_registry().unwrap();
    match options.action {
        Action::Print => {
            let content = serde_yaml::to_string(&registry).unwrap();
            println!("{}", content);
        }
        Action::Record => {
            let content = serde_yaml::to_string(&registry).unwrap();
            let mut f = File::create(FILE_PATH).unwrap();
            writeln!(f, "{}", content).unwrap();
        }
        Action::Test => {
            let reference = std::fs::read_to_string(FILE_PATH).unwrap();
            let content = serde_yaml::to_string(&registry).unwrap() + "\n";
            assert_str::assert_str_eq!(&reference, &content);
        }
    }
}
