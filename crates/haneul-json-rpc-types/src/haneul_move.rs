// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, StructTypeParameter, Visibility};
use move_binary_format::normalized::{
    Field as NormalizedField, Function as HaneulNormalizedFunction, Module as NormalizedModule,
    Struct as NormalizedStruct, Type as NormalizedType,
};
use move_core_types::annotated_value::{MoveStruct, MoveValue};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use haneul_macros::EnumVariantOrder;
use tracing::warn;

use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::haneul_serde::HaneulStructTag;

pub type HaneulMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/haneul_move_tests.rs"]
mod haneul_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulMoveAbilitySet {
    pub abilities: Vec<HaneulMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulMoveStructTypeParameter {
    pub constraints: HaneulMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: HaneulMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulMoveNormalizedStruct {
    pub abilities: HaneulMoveAbilitySet,
    pub type_parameters: Vec<HaneulMoveStructTypeParameter>,
    pub fields: Vec<HaneulMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    #[serde(rename_all = "camelCase")]
    Struct {
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<HaneulMoveNormalizedType>,
    },
    Vector(Box<HaneulMoveNormalizedType>),
    TypeParameter(HaneulMoveTypeParameterIndex),
    Reference(Box<HaneulMoveNormalizedType>),
    MutableReference(Box<HaneulMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulMoveNormalizedFunction {
    pub visibility: HaneulMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<HaneulMoveAbilitySet>,
    pub parameters: Vec<HaneulMoveNormalizedType>,
    pub return_: Vec<HaneulMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<HaneulMoveModuleId>,
    pub structs: BTreeMap<String, HaneulMoveNormalizedStruct>,
    pub exposed_functions: BTreeMap<String, HaneulMoveNormalizedFunction>,
}

impl PartialEq for HaneulMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for HaneulMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| HaneulMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<HaneulMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), HaneulMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, HaneulMoveNormalizedStruct>>(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), HaneulMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, HaneulMoveNormalizedFunction>>(),
        }
    }
}

impl From<HaneulNormalizedFunction> for HaneulMoveNormalizedFunction {
    fn from(function: HaneulNormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => HaneulMoveVisibility::Private,
                Visibility::Public => HaneulMoveVisibility::Public,
                Visibility::Friend => HaneulMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<HaneulMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(HaneulMoveNormalizedType::from)
                .collect::<Vec<HaneulMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(HaneulMoveNormalizedType::from)
                .collect::<Vec<HaneulMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for HaneulMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(HaneulMoveStructTypeParameter::from)
                .collect::<Vec<HaneulMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(HaneulMoveNormalizedField::from)
                .collect::<Vec<HaneulMoveNormalizedField>>(),
        }
    }
}

impl From<StructTypeParameter> for HaneulMoveStructTypeParameter {
    fn from(type_parameter: StructTypeParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for HaneulMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: HaneulMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for HaneulMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => HaneulMoveNormalizedType::Bool,
            NormalizedType::U8 => HaneulMoveNormalizedType::U8,
            NormalizedType::U16 => HaneulMoveNormalizedType::U16,
            NormalizedType::U32 => HaneulMoveNormalizedType::U32,
            NormalizedType::U64 => HaneulMoveNormalizedType::U64,
            NormalizedType::U128 => HaneulMoveNormalizedType::U128,
            NormalizedType::U256 => HaneulMoveNormalizedType::U256,
            NormalizedType::Address => HaneulMoveNormalizedType::Address,
            NormalizedType::Signer => HaneulMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => HaneulMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(HaneulMoveNormalizedType::from)
                    .collect::<Vec<HaneulMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                HaneulMoveNormalizedType::Vector(Box::new(HaneulMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => HaneulMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                HaneulMoveNormalizedType::Reference(Box::new(HaneulMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                HaneulMoveNormalizedType::MutableReference(Box::new(HaneulMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for HaneulMoveAbilitySet {
    fn from(set: AbilitySet) -> HaneulMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => HaneulMoveAbility::Copy,
                    Ability::Drop => HaneulMoveAbility::Drop,
                    Ability::Key => HaneulMoveAbility::Key,
                    Ability::Store => HaneulMoveAbility::Store,
                })
                .collect::<Vec<HaneulMoveAbility>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum HaneulMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(HaneulAddress),
    Vector(Vec<HaneulMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(HaneulMoveStruct),
    Option(Box<Option<HaneulMoveValue>>),
}

impl HaneulMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            HaneulMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            HaneulMoveValue::Vector(values) => HaneulMoveStruct::Runtime(values).to_json_value(),
            HaneulMoveValue::Number(v) => json!(v),
            HaneulMoveValue::Bool(v) => json!(v),
            HaneulMoveValue::Address(v) => json!(v),
            HaneulMoveValue::String(v) => json!(v),
            HaneulMoveValue::UID { id } => json!({ "id": id }),
            HaneulMoveValue::Option(v) => json!(v),
        }
    }
}

impl Display for HaneulMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulMoveValue::Number(value) => write!(writer, "{}", value)?,
            HaneulMoveValue::Bool(value) => write!(writer, "{}", value)?,
            HaneulMoveValue::Address(value) => write!(writer, "{}", value)?,
            HaneulMoveValue::String(value) => write!(writer, "{}", value)?,
            HaneulMoveValue::UID { id } => write!(writer, "{id}")?,
            HaneulMoveValue::Struct(value) => write!(writer, "{}", value)?,
            HaneulMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            HaneulMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for HaneulMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => HaneulMoveValue::Number(value.into()),
            MoveValue::U16(value) => HaneulMoveValue::Number(value.into()),
            MoveValue::U32(value) => HaneulMoveValue::Number(value),
            MoveValue::U64(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => HaneulMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                HaneulMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Haneul core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                HaneulMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                HaneulMoveValue::Address(HaneulAddress::from(ObjectID::from(value)))
            }
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum HaneulMoveStruct {
    Runtime(Vec<HaneulMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "HaneulStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, HaneulMoveValue>,
    },
    WithFields(BTreeMap<String, HaneulMoveValue>),
}

impl HaneulMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            HaneulMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            HaneulMoveStruct::WithTypes { type_: _, fields } | HaneulMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn read_dynamic_field_value(&self, field_name: &str) -> Option<HaneulMoveValue> {
        match self {
            HaneulMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            HaneulMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for HaneulMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulMoveStruct::Runtime(_) => {}
            HaneulMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            HaneulMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<HaneulMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(HaneulMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(HaneulMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(HaneulMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(HaneulMoveValue::from);
            if let Some(HaneulMoveValue::Address(address)) = id {
                return Some(HaneulMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(HaneulMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(HaneulMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(HaneulMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to HaneulMoveValue"
    );
    None
}

impl From<MoveStruct> for HaneulMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        HaneulMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
