// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, ensure};
use clap;
use move_command_line_common::values::ParsableValue;
use move_command_line_common::{parser::Parser as MoveCLParser, values::ValueToken};
use move_compiler::shared::parse_u128;
use move_core_types::identifier::Identifier;
use move_core_types::value::{MoveStruct, MoveValue};
use haneul_types::messages::CallArg;

use crate::test_adapter::HaneulTestAdapter;

pub const HANEUL_ARGS_LONG: &str = "haneul-args";

#[derive(Debug, clap::Parser)]
pub struct HaneulRunArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "view-events")]
    pub view_events: bool,
}

#[derive(Debug, clap::Parser)]
pub struct HaneulPublishArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub struct HaneulInitArgs {
    #[clap(long = "accounts", multiple_values(true), multiple_occurrences(false))]
    pub accounts: Option<Vec<String>>,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    pub id: u64,
}

#[derive(Debug, clap::Parser)]
pub enum HaneulSubcommand {
    #[clap(name = "view-object")]
    ViewObject(ViewObjectCommand),
}

#[derive(Debug)]
pub enum HaneulExtraValueArgs {
    Object(u64),
}

pub enum HaneulValue {
    MoveValue(MoveValue),
    Object(u64),
}

impl HaneulExtraValueArgs {
    fn parse_value_impl<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, move_command_line_common::values::ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "object");
        parser.advance(ValueToken::LParen)?;
        let u_str = parser.advance(ValueToken::Number)?;
        let (fake_id, _) = parse_u128(u_str)?;
        if fake_id > (u64::MAX as u128) {
            bail!("Object id too large")
        }
        parser.advance(ValueToken::RParen)?;
        Ok(HaneulExtraValueArgs::Object(fake_id as u64))
    }
}

impl HaneulValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            HaneulValue::MoveValue(v) => v,
            HaneulValue::Object(_) => panic!("nested haneul objects are not yet supported in args"),
        }
    }

    pub(crate) fn into_call_args(self, test_adapter: &HaneulTestAdapter) -> CallArg {
        match self {
            HaneulValue::Object(fake_id) => {
                let id = match test_adapter.fake_to_real_object_id(fake_id) {
                    Some(id) => id,
                    None => panic!("Unknown object, object({})", fake_id),
                };
                let obj = match test_adapter.storage.get_object(&id) {
                    Some(obj) => obj,
                    None => panic!("Could not load object argument {}", id),
                };
                if obj.is_shared() {
                    CallArg::SharedObject(id)
                } else {
                    let obj_ref = obj.compute_object_reference();
                    CallArg::ImmOrOwnedObject(obj_ref)
                }
            }
            HaneulValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
        }
    }
}

impl ParsableValue for HaneulExtraValueArgs {
    type ConcreteValue = HaneulValue;

    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, move_command_line_common::values::ValueToken, I>,
    ) -> Option<anyhow::Result<Self>> {
        match parser.peek()? {
            (ValueToken::Ident, "object") => Some(Self::parse_value_impl(parser)),
            _ => None,
        }
    }

    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue> {
        Ok(HaneulValue::MoveValue(v))
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(HaneulValue::MoveValue(MoveValue::Vector(
            elems.into_iter().map(HaneulValue::assert_move_value).collect(),
        )))
    }

    fn concrete_struct(
        _addr: move_core_types::account_address::AccountAddress,
        _module: String,
        _name: String,
        values: std::collections::BTreeMap<String, Self::ConcreteValue>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        Ok(HaneulValue::MoveValue(MoveValue::Struct(
            MoveStruct::WithFields(
                values
                    .into_iter()
                    .map(|(f, v)| Ok((Identifier::new(f)?, v.assert_move_value())))
                    .collect::<anyhow::Result<_>>()?,
            ),
        )))
    }

    fn into_concrete_value(
        self,
        _mapping: &impl Fn(&str) -> Option<move_core_types::account_address::AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        match self {
            HaneulExtraValueArgs::Object(id) => Ok(HaneulValue::Object(id)),
        }
    }
}
