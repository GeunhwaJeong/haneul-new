// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::displays::Pretty;
use std::fmt::{Display, Formatter};
use haneul_types::gas::HaneulGasStatus;
use haneul_types::gas_model::gas_v2::HaneulGasStatus as GasStatusV2;
#[allow(unused)]
use tabled::{
    builder::Builder as TableBuilder,
    settings::{style::HorizontalLine, Style as TableStyle},
};

impl<'a> Display for Pretty<'a, HaneulGasStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(haneul_gas_status) = self;
        Ok(match haneul_gas_status {
            HaneulGasStatus::V2(s) => {
                display_info(f, &s)?;
            }
        })
    }
}

#[allow(unused)]
fn per_object_storage_table(f: &mut Formatter, _haneul_gas_status: &GasStatusV2) -> String {
    todo!();
}

fn display_info(f: &mut Formatter<'_>, haneul_gas_status: &GasStatusV2) -> std::fmt::Result {
    write!(
        f,
        "\nReference Gas Price: {}\n",
        haneul_gas_status.reference_gas_price()
    )?;
    write!(f, "Gas Price: {}\n", haneul_gas_status.gas_status.gas_price())?;
    write!(
        f,
        "Max Gas Stack Height: {}\n",
        haneul_gas_status.gas_status.stack_height_high_water_mark()
    )?;
    write!(
        f,
        "Number of Bytecode Instructions Executed: {}",
        haneul_gas_status.gas_status.instructions_executed()
    )
}
