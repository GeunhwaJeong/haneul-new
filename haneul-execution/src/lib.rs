// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use haneul_protocol_config::ProtocolConfig;
use haneul_types::{error::HaneulResult, metrics::BytecodeVerifierMetrics};

pub use executor::Executor;
pub use verifier::Verifier;

pub mod executor;
pub mod verifier;

mod latest;

pub fn executor(
    protocol_config: &ProtocolConfig,
    paranoid_type_checks: bool,
    silent: bool,
) -> HaneulResult<Arc<dyn Executor + Send + Sync>> {
    Ok(Arc::new(latest::Executor::new(
        protocol_config,
        paranoid_type_checks,
        silent,
    )?))
}

pub fn verifier<'m>(
    protocol_config: &ProtocolConfig,
    is_metered: bool,
    metrics: &'m Arc<BytecodeVerifierMetrics>,
) -> Box<dyn Verifier + 'm> {
    Box::new(latest::Verifier::new(protocol_config, is_metered, metrics))
}
