// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, ensure};
use clap;
use move_command_line_common::parser::{parse_u256, parse_u64};
use move_command_line_common::values::{ParsableValue, ParsedValue};
use move_command_line_common::{parser::Parser as MoveCLParser, values::ValueToken};
use move_core_types::identifier::Identifier;
use move_core_types::u256::U256;
use move_core_types::value::{MoveStruct, MoveValue};
use haneul_types::base_types::HaneulAddress;
use haneul_types::messages::{Argument, CallArg, ObjectArg};
use haneul_types::object::Owner;
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;

use crate::test_adapter::{FakeID, HaneulTestAdapter};

pub const HANEUL_ARGS_LONG: &str = "haneul-args";

#[derive(Debug, clap::Parser)]
pub struct HaneulRunArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "view-events")]
    pub view_events: bool,
    #[clap(long = "view-gas-used")]
    pub view_gas_used: bool,
}

#[derive(Debug, clap::Parser)]
pub struct HaneulPublishArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "upgradeable", action = clap::ArgAction::SetTrue)]
    pub upgradeable: bool,
    #[clap(long = "view-gas-used")]
    pub view_gas_used: bool,
    #[clap(
        long = "dependencies",
        multiple_values(true),
        multiple_occurrences(false)
    )]
    pub dependencies: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct HaneulInitArgs {
    #[clap(long = "accounts", multiple_values(true), multiple_occurrences(false))]
    pub accounts: Option<Vec<String>>,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    #[clap(parse(try_from_str = parse_fake_id))]
    pub id: FakeID,
}

#[derive(Debug, clap::Parser)]
pub struct TransferObjectCommand {
    #[clap(parse(try_from_str = parse_fake_id))]
    pub id: FakeID,
    #[clap(long = "recipient")]
    pub recipient: String,
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "view-gas-used")]
    pub view_gas_used: bool,
}

#[derive(Debug, clap::Parser)]
pub struct ConsensusCommitPrologueCommand {
    #[clap(long = "timestamp-ms")]
    pub timestamp_ms: u64,
}

#[derive(Debug, clap::Parser)]
pub struct ProgrammableTransactionCommand {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "view-events")]
    pub view_events: bool,
    #[clap(long = "view-gas-used")]
    pub view_gas_used: bool,
    #[clap(
        long = "inputs",
        parse(try_from_str = ParsedValue::parse),
        takes_value(true),
        multiple_values(true),
        multiple_occurrences(true)
    )]
    pub inputs: Vec<ParsedValue<HaneulExtraValueArgs>>,
}

#[derive(Debug, clap::Parser)]
pub enum HaneulSubcommand {
    #[clap(name = "view-object")]
    ViewObject(ViewObjectCommand),
    #[clap(name = "transfer-object")]
    TransferObject(TransferObjectCommand),
    #[clap(name = "consensus-commit-prologue")]
    ConsensusCommitPrologue(ConsensusCommitPrologueCommand),
    #[clap(name = "programmable")]
    ProgrammableTransaction(ProgrammableTransactionCommand),
}

#[derive(Debug)]
pub enum HaneulExtraValueArgs {
    Object(FakeID),
}

pub enum HaneulValue {
    MoveValue(MoveValue),
    Object(FakeID),
    ObjVec(Vec<FakeID>),
}

impl HaneulExtraValueArgs {
    fn parse_value_impl<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "object");
        parser.advance(ValueToken::LParen)?;
        let i_str = parser.advance(ValueToken::Number)?;
        let (i, _) = parse_u256(i_str)?;
        let fake_id = if let Some(ValueToken::Comma) = parser.peek_tok() {
            parser.advance(ValueToken::Comma)?;
            let j_str = parser.advance(ValueToken::Number)?;
            let (j, _) = parse_u64(j_str)?;
            if i > U256::from(u64::MAX) {
                bail!("Object ID too large")
            }
            FakeID::Enumerated(i.unchecked_as_u64(), j)
        } else {
            let mut u256_bytes = i.to_le_bytes().to_vec();
            u256_bytes.reverse();
            let address: HaneulAddress = HaneulAddress::from_bytes(&u256_bytes).unwrap();
            FakeID::Known(address.into())
        };
        parser.advance(ValueToken::RParen)?;
        Ok(HaneulExtraValueArgs::Object(fake_id))
    }
}

impl HaneulValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            HaneulValue::MoveValue(v) => v,
            HaneulValue::Object(_) => panic!("unexpected nested Haneul object in args"),
            HaneulValue::ObjVec(_) => panic!("unexpected nested Haneul object vector in args"),
        }
    }

    fn assert_object(self) -> FakeID {
        match self {
            HaneulValue::MoveValue(_) => panic!("unexpected nested non-object value in args"),
            HaneulValue::Object(v) => v,
            HaneulValue::ObjVec(_) => panic!("unexpected nested Haneul object vector in args"),
        }
    }

    fn object_arg(fake_id: FakeID, test_adapter: &HaneulTestAdapter) -> anyhow::Result<ObjectArg> {
        let id = match test_adapter.fake_to_real_object_id(fake_id) {
            Some(id) => id,
            None => bail!("INVALID TEST. Unknown object, object({})", fake_id),
        };
        let obj = match test_adapter.storage.get_object(&id) {
            Some(obj) => obj,
            None => bail!("INVALID TEST. Could not load object argument {}", id),
        };
        match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutable: true,
            }),
            Owner::AddressOwner(_) | Owner::ObjectOwner(_) | Owner::Immutable => {
                let obj_ref = obj.compute_object_reference();
                Ok(ObjectArg::ImmOrOwnedObject(obj_ref))
            }
        }
    }

    pub(crate) fn into_call_arg(self, test_adapter: &HaneulTestAdapter) -> anyhow::Result<CallArg> {
        Ok(match self {
            HaneulValue::Object(fake_id) => CallArg::Object(Self::object_arg(fake_id, test_adapter)?),
            HaneulValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
            HaneulValue::ObjVec(_) => bail!("obj vec is not supported as an input"),
        })
    }

    pub(crate) fn into_argument(
        self,
        builder: &mut ProgrammableTransactionBuilder,
        test_adapter: &HaneulTestAdapter,
    ) -> anyhow::Result<Argument> {
        Ok(match self {
            HaneulValue::Object(fake_id) => builder.obj(Self::object_arg(fake_id, test_adapter)?)?,
            HaneulValue::ObjVec(vec) => builder.make_obj_vec(
                vec.iter()
                    .map(|fake_id| Self::object_arg(*fake_id, test_adapter))
                    .collect::<Result<Vec<ObjectArg>, _>>()?,
            )?,
            HaneulValue::MoveValue(v) => {
                builder.input(CallArg::Pure(v.simple_serialize().unwrap()))?
            }
        })
    }
}

impl ParsableValue for HaneulExtraValueArgs {
    type ConcreteValue = HaneulValue;

    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
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
        if !elems.is_empty() && matches!(elems[0], HaneulValue::Object(_)) {
            Ok(HaneulValue::ObjVec(
                elems.into_iter().map(HaneulValue::assert_object).collect(),
            ))
        } else {
            Ok(HaneulValue::MoveValue(MoveValue::Vector(
                elems.into_iter().map(HaneulValue::assert_move_value).collect(),
            )))
        }
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

fn parse_fake_id(s: &str) -> anyhow::Result<FakeID> {
    Ok(if let Some((s1, s2)) = s.split_once(',') {
        let (i, _) = parse_u64(s1)?;
        let (j, _) = parse_u64(s2)?;
        FakeID::Enumerated(i, j)
    } else {
        let (i, _) = parse_u256(s)?;
        let mut u256_bytes = i.to_le_bytes().to_vec();
        u256_bytes.reverse();
        let address: HaneulAddress = HaneulAddress::from_bytes(&u256_bytes).unwrap();
        FakeID::Known(address.into())
    })
}
