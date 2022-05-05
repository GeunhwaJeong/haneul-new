// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[path = "generated/haneul.validator.Validator.rs"]
#[rustfmt::skip]
mod validator;

pub use validator::{
    validator_client::ValidatorClient,
    validator_server::{Validator, ValidatorServer},
};
