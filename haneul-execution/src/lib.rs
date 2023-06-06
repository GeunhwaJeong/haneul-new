// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

pub use executor::Executor;
use haneul_protocol_config::ProtocolConfig;
use haneul_types::error::HaneulError;

pub mod executor;
pub mod latest;

pub fn executor(
    protocol_config: &ProtocolConfig,
    paranoid_type_checks: bool,
    silent: bool,
) -> Result<Arc<dyn Executor + Send + Sync>, HaneulError> {
    Ok(Arc::new(latest::VM::new(
        protocol_config,
        paranoid_type_checks,
        silent,
    )?))
}
