// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;
use multiaddr::Multiaddr;
use std::path::PathBuf;
use haneul_config::{
    genesis::{Builder, Genesis},
    HANEUL_GENESIS_FILENAME,
};
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    crypto::PublicKeyBytes,
    object::Object,
};

#[derive(Parser)]
pub struct Ceremony {
    #[clap(long)]
    path: Option<PathBuf>,

    #[clap(subcommand)]
    command: CeremonyCommand,
}

#[derive(Parser)]
pub enum CeremonyCommand {
    Init,

    AddValidator {
        name: String,
        public_key: PublicKeyBytes,
        network_address: Multiaddr,
        narwhal_primary_to_primary: Multiaddr,
        narwhal_worker_to_primary: Multiaddr,
        narwhal_primary_to_worker: Multiaddr,
        narwhal_worker_to_worker: Multiaddr,
        narwhal_consensus_address: Multiaddr,
    },

    AddGasObject {
        address: HaneulAddress,
        object_id: Option<ObjectID>,
        value: u64,
    },

    Finalize,

    Verify,
}

pub fn run(cmd: Ceremony) -> Result<()> {
    let dir = if let Some(path) = cmd.path {
        path
    } else {
        std::env::current_dir()?
    };

    match cmd.command {
        CeremonyCommand::Init => {
            let builder = Builder::new();
            builder.save(dir)?;
        }

        CeremonyCommand::AddValidator {
            name,
            public_key,
            network_address,
            narwhal_primary_to_primary,
            narwhal_worker_to_primary,
            narwhal_primary_to_worker,
            narwhal_worker_to_worker,
            narwhal_consensus_address,
        } => {
            let mut builder = Builder::load(&dir)?;
            builder = builder.add_validator(haneul_config::ValidatorInfo {
                name,
                public_key,
                stake: 1,
                delegation: 0,
                network_address,
                narwhal_primary_to_primary,
                narwhal_worker_to_primary,
                narwhal_primary_to_worker,
                narwhal_worker_to_worker,
                narwhal_consensus_address,
            });
            builder.save(dir)?;
        }

        CeremonyCommand::AddGasObject {
            address,
            object_id,
            value,
        } => {
            let mut builder = Builder::load(&dir)?;

            let object_id = object_id.unwrap_or_else(ObjectID::random);
            let object = Object::with_id_owner_gas_for_testing(object_id, address, value);
            builder = builder.add_object(object);

            builder.save(dir)?;
        }

        CeremonyCommand::Finalize => {
            let builder = Builder::load(&dir)?;

            let genesis = builder.build();

            genesis.save(dir.join(HANEUL_GENESIS_FILENAME))?;
        }

        CeremonyCommand::Verify => {
            let loaded_genesis = Genesis::load(dir.join(HANEUL_GENESIS_FILENAME))?;

            let builder = Builder::load(&dir)?;

            let built_genesis = builder.build();

            if built_genesis != loaded_genesis {
                return Err(anyhow::anyhow!(
                    "loaded genesis does not match built genesis"
                ));
            }
        }
    }

    Ok(())
}
