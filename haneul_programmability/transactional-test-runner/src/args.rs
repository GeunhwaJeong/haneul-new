// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::test_adapter::HaneulTestAdapter;
use anyhow::{anyhow, bail, ensure};
use clap::*;
use move_compiler::shared::{parse_address, parse_u128, parse_u64, parse_u8, NumericalAddress};
use move_core_types::{account_address::AccountAddress, value::MoveValue};

use haneul_types::{base_types::HANEUL_ADDRESS_LENGTH, messages::CallArg};

pub const HANEUL_ARGS_LONG: &str = "haneul-args";

#[derive(Debug, Parser)]
pub struct HaneulRunArgs {
    #[clap(
        long = HANEUL_ARGS_LONG,
        parse(try_from_str = parse_argument),
        takes_value(true),
        multiple_values(true),
        multiple_occurrences(false),
    )]
    pub args: Vec<HaneulTransactionArg>,

    #[structopt(long = "sender")]
    pub sender: Option<String>,
}

#[derive(Debug, Parser)]
pub struct HaneulPublishArgs {
    #[structopt(long = "sender")]
    pub sender: Option<String>,
}

#[derive(Debug, Parser)]
pub struct HaneulInitArgs {
    #[structopt(long = "accounts", multiple_values(true), multiple_occurrences(false))]
    pub accounts: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum HaneulTransactionArg {
    NamedAddress(String),
    NumericalAddress(NumericalAddress),
    InferredNum(u128),
    U8(u8),
    U64(u64),
    U128(u128),
    Bool(bool),
    Vector(Vec<HaneulTransactionArg>),

    Object([u8; HANEUL_ADDRESS_LENGTH]),
}

fn parse_argument(s: &str) -> anyhow::Result<HaneulTransactionArg> {
    let (arg, s) = parse_argument_rec(s)?;
    ensure!(s.is_empty());
    Ok(arg)
}

fn parse_argument_rec(s: &str) -> anyhow::Result<(HaneulTransactionArg, &str)> {
    use HaneulTransactionArg as A;
    Ok(if let Some(s) = s.strip_prefix('@') {
        check_not_empty("@", s)?;
        if s.chars().next().unwrap().is_numeric() {
            let (addr, s) = split_alpha_numeric(s);
            let addr = NumericalAddress::parse_str(addr).map_err(|msg| anyhow!(msg))?;
            (A::NumericalAddress(addr), s)
        } else {
            ensure!(s.is_ascii());
            let (n, s) = split_alpha_numeric(s);
            (A::NamedAddress(n.to_string()), s)
        }
    } else if let Some(mut s) = s.strip_prefix("vector[") {
        check_not_empty("vector[", s)?;
        let mut args = vec![];
        loop {
            let (arg, after) = parse_argument_rec(s)?;
            args.push(arg);
            match after.strip_prefix(',') {
                None => {
                    s = after;
                    break;
                }
                Some(next) => s = next,
            }
        }
        let s = eat(s, ']')?;
        (A::Vector(args), s)
    } else if let Some(s) = s.strip_prefix("object(") {
        check_not_empty("object(", s)?;
        let (id, s) = split_alpha_numeric(s);
        let (id, _) = parse_address(id)
            .ok_or_else(|| anyhow!("Expected address after 'object(', got \"{}\"", s))?;
        let s = eat(s, ')')?;
        (A::Object(id), s)
    } else if let Some(_s) = s.strip_prefix("x\"") {
        todo!("hex strings not yet supported")
    } else if let Some(_s) = s.strip_prefix("b\"") {
        todo!("byte strings not yet supported")
    } else if let Some(s) = s.strip_prefix("true") {
        (A::Bool(true), s)
    } else if let Some(s) = s.strip_prefix("false") {
        (A::Bool(false), s)
    } else if let Some(s) = s.strip_suffix("u8") {
        let (u, s) = split_alpha_numeric(s);
        let (u, _) = parse_u8(u)?;
        (A::U8(u), s)
    } else if let Some(s) = s.strip_suffix("u64") {
        let (u, s) = split_alpha_numeric(s);
        let (u, _) = parse_u64(u)?;
        (A::U64(u), s)
    } else if let Some(s) = s.strip_suffix("u128") {
        let (u, s) = split_alpha_numeric(s);
        let (u, _) = parse_u128(u)?;
        (A::U128(u), s)
    } else {
        let (u, s) = split_alpha_numeric(s);
        let (u, _) = parse_u128(u)?;
        (A::InferredNum(u), s)
    })
}

fn check_not_empty(prefix: &str, s: &str) -> anyhow::Result<()> {
    if s.is_empty() {
        bail!("Unexpected end of string after prefix: '{}'", prefix)
    }
    Ok(())
}

fn split_alpha_numeric(s: &str) -> (&str, &str) {
    fn is_alpha_numeric(c: char) -> bool {
        matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_')
    }
    match s.split_once(|c| !is_alpha_numeric(c)) {
        None => (s, ""),
        Some((s1, s2)) => (s1, s2),
    }
}

fn eat(s: &str, c: char) -> anyhow::Result<&str> {
    s.strip_prefix(c).ok_or_else(|| {
        anyhow!(
            "Expected character: '{}' at beginning of string \"{}\"",
            c,
            s
        )
    })
}

impl HaneulTransactionArg {
    pub(crate) fn into_call_args(self, test_adapter: &HaneulTestAdapter) -> CallArg {
        match self {
            HaneulTransactionArg::Object(fake_id) => {
                let id = match test_adapter.fake_to_real_object_id(fake_id) {
                    Some(id) => id,
                    None => panic!(
                        "Unknown object, Object({})",
                        AccountAddress::new(fake_id).short_str_lossless()
                    ),
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
            a => {
                let v: MoveValue = a.into_move_value(test_adapter);
                CallArg::Pure(v.simple_serialize().unwrap())
            }
        }
    }

    fn into_move_value(self, test_adapter: &HaneulTestAdapter) -> MoveValue {
        match self {
            HaneulTransactionArg::NamedAddress(n) => {
                MoveValue::Address(test_adapter.compiled_state.resolve_named_address(&n))
            }
            HaneulTransactionArg::NumericalAddress(a) => MoveValue::Address(a.into_inner()),
            HaneulTransactionArg::U8(u) => MoveValue::U8(u),
            HaneulTransactionArg::U64(u) => MoveValue::U64(u),
            HaneulTransactionArg::U128(u) | HaneulTransactionArg::InferredNum(u) => MoveValue::U128(u),
            HaneulTransactionArg::Bool(b) => MoveValue::Bool(b),
            HaneulTransactionArg::Vector(v) => MoveValue::Vector(
                v.into_iter()
                    .map(|a| a.into_move_value(test_adapter))
                    .collect(),
            ),
            HaneulTransactionArg::Object(_) => panic!("Nested object arguments are not supported"),
        }
    }
}
