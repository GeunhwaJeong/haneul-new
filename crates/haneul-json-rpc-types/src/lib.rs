// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use colored::Colorize;
use fastcrypto::encoding::{Base64, Encoding, Hex};
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, StructTypeParameter, Visibility};
use move_binary_format::normalized::{
    Field as NormalizedField, Function as HaneulNormalizedFunction, Module as NormalizedModule,
    Struct as NormalizedStruct, Type as NormalizedType,
};
use move_bytecode_utils::module_cache::GetModule;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::value::{MoveStruct, MoveValue};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::serde_as;
use haneul_json::HaneulJsonValue;
use haneul_types::base_types::{
    ObjectDigest, ObjectID, ObjectInfo, ObjectRef, SequenceNumber, HaneulAddress, TransactionDigest,
    TransactionEffectsDigest,
};
use haneul_types::coin::CoinMetadata;
use haneul_types::committee::EpochId;
use haneul_types::crypto::HaneulAuthorityStrongQuorumSignInfo;
use haneul_types::digests::TransactionEventsDigest;
use haneul_types::dynamic_field::DynamicFieldInfo;
use haneul_types::error::{ExecutionError, HaneulError};
use haneul_types::event::{BalanceChangeType, Event, EventID};
use haneul_types::event::{EventEnvelope, EventType};
use haneul_types::filter::EventFilter;
use haneul_types::gas::GasCostSummary;
use haneul_types::messages::{
    CallArg, EffectsFinalityInfo, ExecutionStatus, GenesisObject, InputObjectKind, ObjectArg, Pay,
    PayAllHaneul, PayHaneul, SenderSignedData, SingleTransactionKind, TransactionData,
    TransactionEffects, TransactionEvents, TransactionKind,
};
use haneul_types::messages_checkpoint::{
    CheckpointContents, CheckpointDigest, CheckpointSequenceNumber, CheckpointSummary,
    CheckpointTimestamp, EndOfEpochData,
};
use haneul_types::object::{Object, Owner};
use haneul_types::signature::GenericSignature;
use haneul_types::haneul_system_state::{HaneulSystemState, HaneulSystemStateInnerV1};
use haneul_types::{parse_haneul_struct_tag, parse_haneul_type_tag};
use tracing::warn;

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

mod haneul_object;
pub use haneul_object::*;

pub type HaneulMoveTypeParameterIndex = u16;
pub type TransactionsPage = Page<TransactionDigest, TransactionDigest>;
pub type EventPage = Page<HaneulEventEnvelope, EventID>;
pub type CoinPage = Page<Coin, ObjectID>;
pub type DynamicFieldPage = Page<DynamicFieldInfo, ObjectID>;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin_type: String,
    pub coin_object_count: usize,
    pub total_balance: u128,
    pub locked_balance: HashMap<EpochId, u128>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub coin_type: String,
    pub coin_object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub balance: u64,
    pub locked_until_epoch: Option<EpochId>,
    pub previous_transaction: TransactionDigest,
}

impl Coin {
    pub fn object_ref(&self) -> ObjectRef {
        (self.coin_object_id, self.version, self.digest)
    }
}

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
pub struct HaneulMoveStructTypeParameter {
    pub constraints: HaneulMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulMoveNormalizedField {
    pub name: String,
    pub type_: HaneulMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
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
pub struct HaneulMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<HaneulMoveModuleId>,
    pub structs: BTreeMap<String, HaneulMoveNormalizedStruct>,
    pub exposed_functions: BTreeMap<String, HaneulMoveNormalizedFunction>,
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
                .exposed_functions
                .into_iter()
                .map(|(name, function)| {
                    (name.to_string(), HaneulMoveNormalizedFunction::from(function))
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HaneulTransactionResponse {
    pub transaction: HaneulTransaction,
    pub effects: HaneulTransactionEffects,
    pub events: HaneulTransactionEvents,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_local_execution: Option<bool>,
    /// The checkpoint number when this transaction was included and hence finalized.
    /// This is only returned in the read api, not in the transaction execution api.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<CheckpointSequenceNumber>,
}

/// We are specifically ignoring events for now until events become more stable.
impl PartialEq for HaneulTransactionResponse {
    fn eq(&self, other: &Self) -> bool {
        self.transaction == other.transaction
            && self.effects == other.effects
            && self.timestamp_ms == other.timestamp_ms
            && self.confirmed_local_execution == other.confirmed_local_execution
            && self.checkpoint == other.checkpoint
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulTBlsSignObjectCommitmentType {
    /// Check that the object is committed by the consensus.
    ConsensusCommitted,
    /// Check that the object is committed using the effects certificate.
    FastPathCommitted(HaneulFinalizedEffects),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulTBlsSignRandomnessObjectResponse {
    pub signature: fastcrypto_tbls::types::RawSignature,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HaneulCoinMetadata {
    /// Number of decimal places the coin uses.
    pub decimals: u8,
    /// Name for the token
    pub name: String,
    /// Symbol for the token
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// URL for the token logo
    pub icon_url: Option<String>,
    /// Object id for the CoinMetadata object
    pub id: Option<ObjectID>,
}

impl TryFrom<Object> for HaneulCoinMetadata {
    type Error = HaneulError;
    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let metadata: CoinMetadata = object.try_into()?;
        let CoinMetadata {
            decimals,
            name,
            symbol,
            description,
            icon_url,
            id,
        } = metadata;
        Ok(Self {
            id: Some(*id.object_id()),
            decimals,
            name,
            symbol,
            description,
            icon_url,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveValue")]
pub enum HaneulMoveValue {
    Number(u64),
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
            MoveValue::U32(value) => HaneulMoveValue::Number(value.into()),
            MoveValue::U64(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => HaneulMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                HaneulMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Haneul core type conversion
                if let MoveStruct::WithTypes { type_, fields } = &value {
                    if let Some(value) = try_convert_type(type_, fields) {
                        return value;
                    }
                };
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveStruct")]
pub enum HaneulMoveStruct {
    Runtime(Vec<HaneulMoveValue>),
    WithTypes {
        #[serde(rename = "type")]
        type_: String,
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
        match move_struct {
            MoveStruct::Runtime(value) => {
                HaneulMoveStruct::Runtime(value.into_iter().map(|value| value.into()).collect())
            }
            MoveStruct::WithFields(value) => HaneulMoveStruct::WithFields(
                value
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect(),
            ),
            MoveStruct::WithTypes { type_, fields } => HaneulMoveStruct::WithTypes {
                type_: type_.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect(),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "Pay")]
pub struct HaneulPay {
    /// The coins to be used for payment
    pub coins: Vec<HaneulObjectRef>,
    /// The addresses that will receive payment
    pub recipients: Vec<HaneulAddress>,
    /// The amounts each recipient will receive.
    /// Must be the same length as amounts
    pub amounts: Vec<u64>,
}

impl From<Pay> for HaneulPay {
    fn from(p: Pay) -> Self {
        let coins = p.coins.into_iter().map(|c| c.into()).collect();
        HaneulPay {
            coins,
            recipients: p.recipients,
            amounts: p.amounts,
        }
    }
}

/// Send HANEUL coins to a list of addresses, following a list of amounts.
/// only for HANEUL coin and does not require a separate gas coin object.
/// Specifically, what pay_haneul does are:
/// 1. debit each input_coin to create new coin following the order of
/// amounts and assign it to the corresponding recipient.
/// 2. accumulate all residual HANEUL from input coins left and deposit all HANEUL to the first
/// input coin, then use the first input coin as the gas coin object.
/// 3. the balance of the first input coin after tx is sum(input_coins) - sum(amounts) - actual_gas_cost
/// 4. all other input coints other than the first one are deleted.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "PayHaneul")]
pub struct HaneulPayHaneul {
    /// The coins to be used for payment
    pub coins: Vec<HaneulObjectRef>,
    /// The addresses that will receive payment
    pub recipients: Vec<HaneulAddress>,
    /// The amounts each recipient will receive.
    /// Must be the same length as amounts
    pub amounts: Vec<u64>,
}

impl From<PayHaneul> for HaneulPayHaneul {
    fn from(p: PayHaneul) -> Self {
        let coins = p.coins.into_iter().map(|c| c.into()).collect();
        HaneulPayHaneul {
            coins,
            recipients: p.recipients,
            amounts: p.amounts,
        }
    }
}

/// Send all HANEUL coins to one recipient.
/// only for HANEUL coin and does not require a separate gas coin object either.
/// Specifically, what pay_all_haneul does are:
/// 1. accumulate all HANEUL from input coins and deposit all HANEUL to the first input coin
/// 2. transfer the updated first coin to the recipient and also use this first coin as
/// gas coin object.
/// 3. the balance of the first input coin after tx is sum(input_coins) - actual_gas_cost.
/// 4. all other input coins other than the first are deleted.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "PayAllHaneul")]
pub struct HaneulPayAllHaneul {
    /// The coins to be used for payment
    pub coins: Vec<HaneulObjectRef>,
    /// The addresses that will receive payment
    pub recipient: HaneulAddress,
}

impl From<PayAllHaneul> for HaneulPayAllHaneul {
    fn from(p: PayAllHaneul) -> Self {
        let coins = p.coins.into_iter().map(|c| c.into()).collect();
        HaneulPayAllHaneul {
            coins,
            recipient: p.recipient,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename = "GasData", rename_all = "camelCase")]
pub struct HaneulGasData {
    pub payment: Vec<HaneulObjectRef>,
    pub owner: HaneulAddress,
    pub price: u64,
    pub budget: u64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename = "TransactionData", rename_all = "camelCase")]
pub struct HaneulTransactionData {
    pub transactions: Vec<HaneulTransactionKind>,
    pub sender: HaneulAddress,
    pub gas_data: HaneulGasData,
}

impl HaneulTransactionData {
    pub fn move_calls(&self) -> Vec<&HaneulMoveCall> {
        self.transactions
            .iter()
            .filter_map(|tx| match tx {
                HaneulTransactionKind::Call(call) => Some(call),
                _ => None,
            })
            .collect()
    }
}

impl Display for HaneulTransactionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        if self.transactions.len() == 1 {
            writeln!(writer, "{}", self.transactions.first().unwrap())?;
        } else {
            writeln!(writer, "Transaction Kind : Batch")?;
            writeln!(writer, "List of transactions in the batch:")?;
            for kind in &self.transactions {
                writeln!(writer, "{}", kind)?;
            }
        }
        writeln!(writer, "Sender: {}", self.sender)?;
        write!(writer, "Gas Payment: ")?;
        for payment in &self.gas_data.payment {
            write!(writer, "{} ", payment)?;
        }
        writeln!(writer)?;
        writeln!(writer, "Gas Owner: {}", self.gas_data.owner)?;
        writeln!(writer, "Gas Price: {}", self.gas_data.price)?;
        writeln!(writer, "Gas Budget: {}", self.gas_data.budget)?;
        write!(f, "{}", writer)
    }
}

impl TryFrom<TransactionData> for HaneulTransactionData {
    type Error = anyhow::Error;

    fn try_from(data: TransactionData) -> Result<Self, Self::Error> {
        let transactions = match data.kind.clone() {
            TransactionKind::Single(tx) => {
                vec![tx.try_into()?]
            }
            TransactionKind::Batch(txs) => txs
                .into_iter()
                .map(HaneulTransactionKind::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        };
        Ok(Self {
            transactions,
            sender: data.sender(),
            gas_data: HaneulGasData {
                payment: data
                    .gas()
                    .iter()
                    .map(|obj_ref| HaneulObjectRef::from(*obj_ref))
                    .collect(),
                owner: data.gas_owner(),
                price: data.gas_price(),
                budget: data.gas_budget(),
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename = "Transaction", rename_all = "camelCase")]
pub struct HaneulTransaction {
    pub data: HaneulTransactionData,
    pub tx_signatures: Vec<GenericSignature>,
}

impl TryFrom<SenderSignedData> for HaneulTransaction {
    type Error = anyhow::Error;

    fn try_from(data: SenderSignedData) -> Result<Self, Self::Error> {
        Ok(Self {
            data: data.intent_message.value.try_into()?,
            tx_signatures: data.tx_signatures,
        })
    }
}

impl Display for HaneulTransaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Transaction Signature: {:?}", self.tx_signatures)?;
        write!(writer, "{}", &self.data)?;
        write!(f, "{}", writer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HaneulGenesisTransaction {
    pub objects: Vec<ObjectID>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HaneulConsensusCommitPrologue {
    pub epoch: u64,
    pub round: u64,
    pub commit_timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename = "TransactionKind")]
pub enum HaneulTransactionKind {
    /// Initiate an object transfer between addresses
    TransferObject(HaneulTransferObject),
    /// Pay one or more recipients from a set of input coins
    Pay(HaneulPay),
    /// Pay one or more recipients from a set of Haneul coins, the input coins
    /// are also used to for gas payments.
    PayHaneul(HaneulPayHaneul),
    /// Pay one or more recipients from a set of Haneul coins, the input coins
    /// are also used to for gas payments.
    PayAllHaneul(HaneulPayAllHaneul),
    /// Publish a new Move module
    Publish(HaneulMovePackage),
    /// Call a function in a published Move module
    Call(HaneulMoveCall),
    /// Initiate a HANEUL coin transfer between addresses
    TransferHaneul(HaneulTransferHaneul),
    /// A system transaction that will update epoch information on-chain.
    ChangeEpoch(HaneulChangeEpoch),
    /// A system transaction used for initializing the initial state of the chain.
    Genesis(HaneulGenesisTransaction),
    /// A system transaction marking the start of a series of transactions scheduled as part of a
    /// checkpoint
    ConsensusCommitPrologue(HaneulConsensusCommitPrologue),
    // .. more transaction types go here
}

impl Display for HaneulTransactionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match &self {
            Self::TransferObject(t) => {
                writeln!(writer, "Transaction Kind : Transfer Object")?;
                writeln!(writer, "Recipient : {}", t.recipient)?;
                writeln!(writer, "Object ID : {}", t.object_ref.object_id)?;
                writeln!(writer, "Version : {:?}", t.object_ref.version)?;
                write!(
                    writer,
                    "Object Digest : {}",
                    Base64::encode(t.object_ref.digest)
                )?;
            }
            Self::TransferHaneul(t) => {
                writeln!(writer, "Transaction Kind : Transfer HANEUL")?;
                writeln!(writer, "Recipient : {}", t.recipient)?;
                if let Some(amount) = t.amount {
                    writeln!(writer, "Amount: {}", amount)?;
                } else {
                    writeln!(writer, "Amount: Full Balance")?;
                }
            }
            Self::Pay(p) => {
                writeln!(writer, "Transaction Kind : Pay")?;
                writeln!(writer, "Coins:")?;
                for obj_ref in &p.coins {
                    writeln!(writer, "Object ID : {}", obj_ref.object_id)?;
                }
                writeln!(writer, "Recipients:")?;
                for recipient in &p.recipients {
                    writeln!(writer, "{}", recipient)?;
                }
                writeln!(writer, "Amounts:")?;
                for amount in &p.amounts {
                    writeln!(writer, "{}", amount)?
                }
            }
            Self::PayHaneul(p) => {
                writeln!(writer, "Transaction Kind : Pay HANEUL")?;
                writeln!(writer, "Coins:")?;
                for obj_ref in &p.coins {
                    writeln!(writer, "Object ID : {}", obj_ref.object_id)?;
                }
                writeln!(writer, "Recipients:")?;
                for recipient in &p.recipients {
                    writeln!(writer, "{}", recipient)?;
                }
                writeln!(writer, "Amounts:")?;
                for amount in &p.amounts {
                    writeln!(writer, "{}", amount)?
                }
            }
            Self::PayAllHaneul(p) => {
                writeln!(writer, "Transaction Kind : Pay HANEUL")?;
                writeln!(writer, "Coins:")?;
                for obj_ref in &p.coins {
                    writeln!(writer, "Object ID : {}", obj_ref.object_id)?;
                }
                writeln!(writer, "Recipient:")?;
                writeln!(writer, "{}", &p.recipient)?;
            }
            Self::Publish(_p) => {
                write!(writer, "Transaction Kind : Publish")?;
            }
            Self::Call(c) => {
                writeln!(writer, "Transaction Kind : Call")?;
                writeln!(writer, "Package ID : {}", c.package.to_hex_literal())?;
                writeln!(writer, "Module : {}", c.module)?;
                writeln!(writer, "Function : {}", c.function)?;
                writeln!(writer, "Arguments : {:?}", c.arguments)?;
                write!(writer, "Type Arguments : {:?}", c.type_arguments)?;
            }
            Self::ChangeEpoch(e) => {
                writeln!(writer, "Transaction Kind : Epoch Change")?;
                writeln!(writer, "New epoch ID : {}", e.epoch)?;
                writeln!(writer, "Storage gas reward : {}", e.storage_charge)?;
                writeln!(writer, "Computation gas reward : {}", e.computation_charge)?;
                writeln!(writer, "Storage rebate : {}", e.storage_rebate)?;
                writeln!(writer, "Timestamp : {}", e.epoch_start_timestamp_ms)?;
            }
            Self::Genesis(_) => {
                writeln!(writer, "Transaction Kind : Genesis Transaction")?;
            }
            Self::ConsensusCommitPrologue(p) => {
                writeln!(writer, "Transaction Kind : Consensus Commit Prologue")?;
                writeln!(
                    writer,
                    "Epoch: {}, Round: {}, Timestamp : {}",
                    p.epoch, p.round, p.commit_timestamp_ms
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

impl TryFrom<SingleTransactionKind> for HaneulTransactionKind {
    type Error = anyhow::Error;

    fn try_from(tx: SingleTransactionKind) -> Result<Self, Self::Error> {
        Ok(match tx {
            SingleTransactionKind::TransferObject(t) => Self::TransferObject(HaneulTransferObject {
                recipient: t.recipient,
                object_ref: t.object_ref.into(),
            }),
            SingleTransactionKind::TransferHaneul(t) => Self::TransferHaneul(HaneulTransferHaneul {
                recipient: t.recipient,
                amount: t.amount,
            }),
            SingleTransactionKind::Pay(p) => Self::Pay(p.into()),
            SingleTransactionKind::PayHaneul(p) => Self::PayHaneul(p.into()),
            SingleTransactionKind::PayAllHaneul(p) => Self::PayAllHaneul(p.into()),
            SingleTransactionKind::Publish(p) => Self::Publish(p.into()),
            SingleTransactionKind::Call(c) => Self::Call(HaneulMoveCall {
                package: c.package,
                module: c.module.to_string(),
                function: c.function.to_string(),
                type_arguments: c.type_arguments.iter().map(|ty| ty.to_string()).collect(),
                arguments: c
                    .arguments
                    .into_iter()
                    .map(|arg| match arg {
                        CallArg::Pure(p) => HaneulJsonValue::from_bcs_bytes(&p),
                        CallArg::Object(ObjectArg::ImmOrOwnedObject((id, _, _)))
                        | CallArg::Object(ObjectArg::SharedObject { id, .. }) => {
                            HaneulJsonValue::new(Value::String(Hex::encode(id)))
                        }
                        CallArg::ObjVec(vec) => HaneulJsonValue::new(Value::Array(
                            vec.iter()
                                .map(|obj_arg| match obj_arg {
                                    ObjectArg::ImmOrOwnedObject((id, _, _))
                                    | ObjectArg::SharedObject { id, .. } => {
                                        Value::String(Hex::encode(id))
                                    }
                                })
                                .collect(),
                        )),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            SingleTransactionKind::ChangeEpoch(e) => Self::ChangeEpoch(HaneulChangeEpoch {
                epoch: e.epoch,
                storage_charge: e.storage_charge,
                computation_charge: e.computation_charge,
                storage_rebate: e.storage_rebate,
                epoch_start_timestamp_ms: e.epoch_start_timestamp_ms,
            }),
            SingleTransactionKind::Genesis(g) => Self::Genesis(HaneulGenesisTransaction {
                objects: g.objects.iter().map(GenesisObject::id).collect(),
            }),
            SingleTransactionKind::ConsensusCommitPrologue(p) => {
                Self::ConsensusCommitPrologue(HaneulConsensusCommitPrologue {
                    epoch: p.epoch,
                    round: p.round,
                    commit_timestamp_ms: p.commit_timestamp_ms,
                })
            }
            SingleTransactionKind::ProgrammableTransaction(_) => {
                anyhow::bail!("programmable transactions are not yet supported")
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename = "MoveCall", rename_all = "camelCase")]
pub struct HaneulMoveCall {
    pub package: ObjectID,
    pub module: String,
    pub function: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_arguments: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<HaneulJsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HaneulChangeEpoch {
    pub epoch: EpochId,
    pub storage_charge: u64,
    pub computation_charge: u64,
    pub storage_rebate: u64,
    pub epoch_start_timestamp_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "EffectsFinalityInfo", rename_all = "camelCase")]
pub enum HaneulEffectsFinalityInfo {
    Certified(HaneulAuthorityStrongQuorumSignInfo),
    Checkpointed(EpochId, CheckpointSequenceNumber),
}

impl From<EffectsFinalityInfo> for HaneulEffectsFinalityInfo {
    fn from(info: EffectsFinalityInfo) -> Self {
        match info {
            EffectsFinalityInfo::Certified(cert) => {
                Self::Certified(HaneulAuthorityStrongQuorumSignInfo::from(&cert))
            }
            EffectsFinalityInfo::Checkpointed(epoch, checkpoint) => {
                Self::Checkpointed(epoch, checkpoint)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "FinalizedEffects", rename_all = "camelCase")]
pub struct HaneulFinalizedEffects {
    pub transaction_effects_digest: TransactionEffectsDigest,
    pub effects: HaneulTransactionEffects,
    pub finality_info: HaneulEffectsFinalityInfo,
}

impl Display for HaneulFinalizedEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        writeln!(
            writer,
            "Transaction Effects Digest: {:?}",
            self.transaction_effects_digest
        )?;
        writeln!(writer, "Transaction Effects: {:?}", self.effects)?;
        match &self.finality_info {
            HaneulEffectsFinalityInfo::Certified(cert) => {
                writeln!(writer, "Signed Authorities Bitmap: {:?}", cert.signers_map)?;
            }
            HaneulEffectsFinalityInfo::Checkpointed(epoch, checkpoint) => {
                writeln!(
                    writer,
                    "Finalized at epoch {:?}, checkpoint {:?}",
                    epoch, checkpoint
                )?;
            }
        }

        write!(f, "{}", writer)
    }
}

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionEffects", rename_all = "camelCase")]
pub struct HaneulTransactionEffects {
    /// The status of the execution
    pub status: HaneulExecutionStatus,
    /// The epoch when this transaction was executed.
    pub executed_epoch: EpochId,
    pub gas_used: HaneulGasCostSummary,
    /// The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_objects: Vec<HaneulObjectRef>,
    /// The transaction digest
    pub transaction_digest: TransactionDigest,
    /// ObjectRef and owner of new objects created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of mutated objects, including gas object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutated: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in the past,
    /// and just got extracted out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped: Vec<OwnedObjectRef>,
    /// Object Refs of objects now deleted (the old refs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<HaneulObjectRef>,
    /// Object refs of objects previously wrapped in other objects but now deleted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped_then_deleted: Vec<HaneulObjectRef>,
    /// Object refs of objects now wrapped in other objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wrapped: Vec<HaneulObjectRef>,
    /// The updated gas object reference. Have a dedicated field for convenient access.
    /// It's also included in mutated.
    pub gas_object: OwnedObjectRef,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    pub events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<TransactionDigest>,
}

impl HaneulTransactionEffects {
    /// Return an iterator of mutated objects, but excluding the gas object.
    pub fn mutated_excluding_gas(&self) -> impl Iterator<Item = &OwnedObjectRef> {
        self.mutated.iter().filter(|o| *o != &self.gas_object)
    }
}

impl From<TransactionEffects> for HaneulTransactionEffects {
    fn from(effect: TransactionEffects) -> Self {
        Self {
            status: effect.status.into(),
            executed_epoch: effect.executed_epoch,
            gas_used: effect.gas_used.into(),
            shared_objects: to_haneul_object_ref(effect.shared_objects),
            transaction_digest: effect.transaction_digest,
            created: to_owned_ref(effect.created),
            mutated: to_owned_ref(effect.mutated),
            unwrapped: to_owned_ref(effect.unwrapped),
            deleted: to_haneul_object_ref(effect.deleted),
            unwrapped_then_deleted: to_haneul_object_ref(effect.unwrapped_then_deleted),
            wrapped: to_haneul_object_ref(effect.wrapped),
            gas_object: OwnedObjectRef {
                owner: effect.gas_object.1,
                reference: effect.gas_object.0.into(),
            },
            events_digest: effect.events_digest,
            dependencies: effect.dependencies,
        }
    }
}

impl Display for HaneulTransactionEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Status : {:?}", self.status)?;
        if !self.created.is_empty() {
            writeln!(writer, "Created Objects:")?;
            for oref in &self.created {
                writeln!(
                    writer,
                    "  - ID: {} , Owner: {}",
                    oref.reference.object_id, oref.owner
                )?;
            }
        }
        if !self.mutated.is_empty() {
            writeln!(writer, "Mutated Objects:")?;
            for oref in &self.mutated {
                writeln!(
                    writer,
                    "  - ID: {} , Owner: {}",
                    oref.reference.object_id, oref.owner
                )?;
            }
        }
        if !self.deleted.is_empty() {
            writeln!(writer, "Deleted Objects:")?;
            for oref in &self.deleted {
                writeln!(writer, "  - ID: {}", oref.object_id)?;
            }
        }
        if !self.wrapped.is_empty() {
            writeln!(writer, "Wrapped Objects:")?;
            for oref in &self.wrapped {
                writeln!(writer, "  - ID: {}", oref.object_id)?;
            }
        }
        if !self.unwrapped.is_empty() {
            writeln!(writer, "Unwrapped Objects:")?;
            for oref in &self.unwrapped {
                writeln!(
                    writer,
                    "  - ID: {} , Owner: {}",
                    oref.reference.object_id, oref.owner
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DryRunTransactionResponse {
    pub effects: HaneulTransactionEffects,
    pub events: HaneulTransactionEvents,
}

#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionEffects", transparent)]
pub struct HaneulTransactionEvents {
    pub data: Vec<HaneulEvent>,
}

impl HaneulTransactionEvents {
    pub fn try_from(
        events: TransactionEvents,
        resolver: &impl GetModule,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            data: events
                .data
                .into_iter()
                .map(|event| HaneulEvent::try_from(event, resolver))
                .collect::<Result<_, _>>()?,
        })
    }
}

/// The response from processing a dev inspect transaction
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "DevInspectResults", rename_all = "camelCase")]
pub struct DevInspectResults {
    /// Summary of effects that likely would be generated if the transaction is actually run.
    /// Note however, that not all dev-inspect transactions are actually usable as transactions so
    /// it might not be possible actually generate these effects from a normal transaction.
    pub effects: HaneulTransactionEffects,
    /// Events that likely would be generated if the transaction is actually run.
    pub events: HaneulTransactionEvents,
    /// Execution results (including return values) from executing the transactions
    /// Currently contains only return values from Move calls
    pub results: Result<Vec<(usize, HaneulExecutionResult)>, String>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "HaneulExecutionResult", rename_all = "camelCase")]
pub struct HaneulExecutionResult {
    /// The value of any arguments that were mutably borrowed.
    /// Non-mut borrowed values are not included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutable_reference_outputs: Vec<(/* local index */ u8, Vec<u8>, HaneulTypeTag)>,
    /// The return values from the function
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub return_values: Vec<(Vec<u8>, HaneulTypeTag)>,
}

type ExecutionResult = (
    /*  mutable_reference_outputs */ Vec<(u8, Vec<u8>, TypeTag)>,
    /*  return_values */ Vec<(Vec<u8>, TypeTag)>,
);

impl DevInspectResults {
    pub fn new(
        effects: TransactionEffects,
        events: TransactionEvents,
        return_values: Result<Vec<(usize, ExecutionResult)>, ExecutionError>,
        resolver: &impl GetModule,
    ) -> Result<Self, anyhow::Error> {
        let results = match return_values {
            Err(e) => Err(format!("{}", e)),
            Ok(srvs) => Ok(srvs
                .into_iter()
                .map(|(idx, srv)| {
                    let (mutable_reference_outputs, return_values) = srv;
                    let mutable_reference_outputs = mutable_reference_outputs
                        .into_iter()
                        .map(|(i, bytes, tag)| (i, bytes, HaneulTypeTag::from(tag)))
                        .collect();
                    let return_values = return_values
                        .into_iter()
                        .map(|(bytes, tag)| (bytes, HaneulTypeTag::from(tag)))
                        .collect();
                    let res = HaneulExecutionResult {
                        mutable_reference_outputs,
                        return_values,
                    };
                    (idx, res)
                })
                .collect()),
        };
        Ok(Self {
            effects: effects.into(),
            events: HaneulTransactionEvents::try_from(events, resolver)?,
            results,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum HaneulTransactionBuilderMode {
    /// Regular Haneul Transactions that are committed on chain
    Commit,
    /// Simulated transaction that allows calling any Move function with
    /// arbitrary values.
    DevInspect,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "ExecutionStatus", rename_all = "camelCase", tag = "status")]
pub enum HaneulExecutionStatus {
    // Gas used in the success case.
    Success,
    // Gas used in the failed case, and the error.
    Failure { error: String },
}

impl HaneulExecutionStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, HaneulExecutionStatus::Success { .. })
    }
    pub fn is_err(&self) -> bool {
        matches!(self, HaneulExecutionStatus::Failure { .. })
    }
}

impl From<ExecutionStatus> for HaneulExecutionStatus {
    fn from(status: ExecutionStatus) -> Self {
        match status {
            ExecutionStatus::Success => Self::Success,
            ExecutionStatus::Failure {
                error,
                command: None,
            } => Self::Failure {
                error: format!("{error:?}"),
            },
            ExecutionStatus::Failure {
                error,
                command: Some(idx),
            } => Self::Failure {
                error: format!("{error:?} in command {idx}"),
            },
        }
    }
}

fn to_haneul_object_ref(refs: Vec<ObjectRef>) -> Vec<HaneulObjectRef> {
    refs.into_iter().map(HaneulObjectRef::from).collect()
}

fn to_owned_ref(owned_refs: Vec<(ObjectRef, Owner)>) -> Vec<OwnedObjectRef> {
    owned_refs
        .into_iter()
        .map(|(oref, owner)| OwnedObjectRef {
            owner,
            reference: oref.into(),
        })
        .collect()
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "GasCostSummary", rename_all = "camelCase")]
pub struct HaneulGasCostSummary {
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
}

impl From<GasCostSummary> for HaneulGasCostSummary {
    fn from(s: GasCostSummary) -> Self {
        Self {
            computation_cost: s.computation_cost,
            storage_cost: s.storage_cost,
            storage_rebate: s.storage_rebate,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "OwnedObjectRef")]
pub struct OwnedObjectRef {
    pub owner: Owner,
    pub reference: HaneulObjectRef,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "EventEnvelope", rename_all = "camelCase")]
pub struct HaneulEventEnvelope {
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    pub timestamp: u64,
    /// Transaction digest of associated transaction
    pub tx_digest: TransactionDigest,
    /// Sequential event ID, ie (transaction seq number, event seq number).
    /// 1) Serves as a unique event ID for each fullnode
    /// 2) Also serves to sequence events for the purposes of pagination and querying.
    ///    A higher id is an event seen later by that fullnode.
    /// This ID is the "cursor" for event querying.
    pub id: EventID,
    /// Specific event type
    pub event: HaneulEvent,
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "Event", rename_all = "camelCase")]
pub enum HaneulEvent {
    /// Move-specific event
    #[serde(rename_all = "camelCase")]
    MoveEvent {
        // TODO: What's the best way to serialize this using `AccountAddress::short_str_lossless` ??
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        type_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        fields: Option<HaneulMoveStruct>,
        #[serde_as(as = "Base64")]
        #[schemars(with = "Base64")]
        bcs: Vec<u8>,
    },
    /// Module published
    #[serde(rename_all = "camelCase")]
    Publish {
        sender: HaneulAddress,
        package_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
    /// Coin balance changing event
    #[serde(rename_all = "camelCase")]
    CoinBalanceChange {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        change_type: BalanceChangeType,
        owner: Owner,
        coin_type: String,
        coin_object_id: ObjectID,
        version: SequenceNumber,
        amount: i128,
    },
    /// Epoch change
    EpochChange(EpochId),
    /// New checkpoint
    Checkpoint(CheckpointSequenceNumber),
    /// Transfer objects to new address / wrap in another object / coin
    #[serde(rename_all = "camelCase")]
    TransferObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        recipient: Owner,
        object_type: String,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// Object mutated.
    #[serde(rename_all = "camelCase")]
    MutateObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        object_type: String,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// Delete object
    #[serde(rename_all = "camelCase")]
    DeleteObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// New object creation
    #[serde(rename_all = "camelCase")]
    NewObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        recipient: Owner,
        object_type: String,
        object_id: ObjectID,
        version: SequenceNumber,
    },
}

impl TryFrom<HaneulEvent> for Event {
    type Error = anyhow::Error;
    fn try_from(event: HaneulEvent) -> Result<Self, Self::Error> {
        Ok(match event {
            HaneulEvent::MoveEvent {
                package_id,
                transaction_module,
                sender,
                type_,
                fields: _,
                bcs,
            } => Event::MoveEvent {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                type_: parse_haneul_struct_tag(&type_)?,
                contents: bcs,
            },
            HaneulEvent::Publish {
                sender,
                package_id,
                version,
                digest,
            } => Event::Publish {
                sender,
                package_id,
                version,
                digest,
            },
            HaneulEvent::TransferObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            } => Event::TransferObject {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            },
            HaneulEvent::DeleteObject {
                package_id,
                transaction_module,
                sender,
                object_id,
                version,
            } => Event::DeleteObject {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                object_id,
                version,
            },
            HaneulEvent::NewObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            } => Event::NewObject {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            },
            HaneulEvent::EpochChange(id) => Event::EpochChange(id),
            HaneulEvent::Checkpoint(seq) => Event::Checkpoint(seq),
            HaneulEvent::CoinBalanceChange {
                package_id,
                transaction_module,
                sender,
                change_type,
                owner,
                coin_object_id: coin_id,
                version,
                coin_type,
                amount,
            } => Event::CoinBalanceChange {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                change_type,
                owner,
                coin_type,
                coin_object_id: coin_id,
                version,
                amount,
            },
            HaneulEvent::MutateObject {
                package_id,
                transaction_module,
                sender,
                object_type,
                object_id,
                version,
            } => Event::MutateObject {
                package_id,
                transaction_module: Identifier::from_str(&transaction_module)?,
                sender,
                object_type,
                object_id,
                version,
            },
        })
    }
}

impl HaneulEvent {
    pub fn try_from(event: Event, resolver: &impl GetModule) -> Result<Self, anyhow::Error> {
        Ok(match event {
            Event::MoveEvent {
                package_id,
                transaction_module,
                sender,
                type_,
                contents,
            } => {
                let bcs = contents.to_vec();

                let (type_, fields) = if let Ok(move_struct) =
                    Event::move_event_to_move_struct(&type_, &contents, resolver)
                {
                    let (type_, field) = type_and_fields_from_move_struct(&type_, move_struct);
                    (type_, Some(field))
                } else {
                    (type_.to_string(), None)
                };

                HaneulEvent::MoveEvent {
                    package_id,
                    transaction_module: transaction_module.to_string(),
                    sender,
                    type_,
                    fields,
                    bcs,
                }
            }
            Event::Publish {
                sender,
                package_id,
                version,
                digest,
            } => HaneulEvent::Publish {
                sender,
                package_id,
                version,
                digest,
            },
            Event::TransferObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            } => HaneulEvent::TransferObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                recipient,
                object_type,
                object_id,
                version,
            },
            Event::DeleteObject {
                package_id,
                transaction_module,
                sender,
                object_id,
                version,
            } => HaneulEvent::DeleteObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                object_id,
                version,
            },
            Event::NewObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_type,
                object_id,
                version,
            } => HaneulEvent::NewObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                recipient,
                object_type,
                object_id,
                version,
            },
            Event::EpochChange(id) => HaneulEvent::EpochChange(id),
            Event::Checkpoint(seq) => HaneulEvent::Checkpoint(seq),
            Event::CoinBalanceChange {
                package_id,
                transaction_module,
                sender,
                change_type,
                owner,
                coin_object_id: coin_id,
                version,
                coin_type,
                amount,
            } => HaneulEvent::CoinBalanceChange {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                change_type,
                owner,
                coin_object_id: coin_id,
                version,
                coin_type,
                amount,
            },
            Event::MutateObject {
                package_id,
                transaction_module,
                sender,
                object_type,
                object_id,
                version,
            } => HaneulEvent::MutateObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                object_type,
                object_id,
                version,
            },
        })
    }

    pub fn get_event_type(&self) -> String {
        match self {
            HaneulEvent::MoveEvent { .. } => "MoveEvent".to_string(),
            HaneulEvent::Publish { .. } => "Publish".to_string(),
            HaneulEvent::TransferObject { .. } => "TransferObject".to_string(),
            HaneulEvent::DeleteObject { .. } => "DeleteObject".to_string(),
            HaneulEvent::NewObject { .. } => "NewObject".to_string(),
            HaneulEvent::EpochChange(..) => "EpochChange".to_string(),
            HaneulEvent::Checkpoint(..) => "CheckPoint".to_string(),
            HaneulEvent::CoinBalanceChange { .. } => "CoinBalanceChange".to_string(),
            HaneulEvent::MutateObject { .. } => "MutateObject".to_string(),
        }
    }
}

impl PartialEq<HaneulEventEnvelope> for EventEnvelope {
    fn eq(&self, other: &HaneulEventEnvelope) -> bool {
        self.timestamp == other.timestamp
            && self.tx_digest == other.tx_digest
            && self.event == other.event
    }
}

impl PartialEq<HaneulEvent> for Event {
    fn eq(&self, other: &HaneulEvent) -> bool {
        match self {
            Event::MoveEvent {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                type_: self_type,
                contents: self_contents,
            } => {
                if let HaneulEvent::MoveEvent {
                    package_id,
                    transaction_module,
                    sender,
                    type_,
                    fields: _fields,
                    bcs,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && &self_type.to_string() == type_
                        && self_contents == bcs
                } else {
                    false
                }
            }
            Event::Publish {
                sender: self_sender,
                package_id: self_package_id,
                version: self_version,
                digest: self_digest,
            } => {
                if let HaneulEvent::Publish {
                    package_id,
                    sender,
                    version,
                    digest,
                } = other
                {
                    package_id == self_package_id
                        && self_sender == sender
                        && self_version == version
                        && self_digest == digest
                } else {
                    false
                }
            }
            Event::TransferObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                recipient: self_recipient,
                object_type: self_object_type,
                object_id: self_object_id,
                version: self_version,
            } => {
                if let HaneulEvent::TransferObject {
                    package_id,
                    transaction_module,
                    sender,
                    recipient,
                    object_type,
                    object_id,
                    version,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_recipient == recipient
                        && self_object_id == object_id
                        && self_version == version
                        && self_object_type == object_type
                } else {
                    false
                }
            }
            Event::DeleteObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                object_id: self_object_id,
                version: self_version,
            } => {
                if let HaneulEvent::DeleteObject {
                    package_id,
                    transaction_module,
                    sender,
                    object_id,
                    version,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_object_id == object_id
                        && self_version == version
                } else {
                    false
                }
            }
            Event::NewObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                recipient: self_recipient,
                object_type: self_object_type,
                object_id: self_object_id,
                version: self_version,
            } => {
                if let HaneulEvent::NewObject {
                    package_id,
                    transaction_module,
                    sender,
                    recipient,
                    object_type,
                    object_id,
                    version,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_recipient == recipient
                        && self_object_id == object_id
                        && self_object_type == object_type
                        && self_version == version
                } else {
                    false
                }
            }
            Event::EpochChange(self_id) => {
                if let HaneulEvent::EpochChange(id) = other {
                    self_id == id
                } else {
                    false
                }
            }
            Event::Checkpoint(self_id) => {
                if let HaneulEvent::Checkpoint(id) = other {
                    self_id == id
                } else {
                    false
                }
            }
            Event::CoinBalanceChange {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                change_type: self_change_type,
                owner: self_owner,
                coin_object_id: self_coin_id,
                version: self_version,
                coin_type: self_coin_type,
                amount: self_amount,
            } => {
                if let HaneulEvent::CoinBalanceChange {
                    package_id,
                    transaction_module,
                    sender,
                    change_type,
                    owner,
                    coin_object_id,
                    version,
                    coin_type,
                    amount,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_owner == owner
                        && self_coin_id == coin_object_id
                        && self_version == version
                        && &self_coin_type.to_string() == coin_type
                        && self_amount == amount
                        && self_sender == sender
                        && self_change_type == change_type
                } else {
                    false
                }
            }
            Event::MutateObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                object_type: self_object_type,
                object_id: self_object_id,
                version: self_version,
            } => {
                if let HaneulEvent::MutateObject {
                    package_id,
                    transaction_module,
                    sender,
                    object_type,
                    object_id,
                    version,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_object_type == object_type
                        && self_object_id == object_id
                        && self_version == version
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransferObject", rename_all = "camelCase")]
pub struct HaneulTransferObject {
    pub recipient: HaneulAddress,
    pub object_ref: HaneulObjectRef,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransferHaneul", rename_all = "camelCase")]
pub struct HaneulTransferHaneul {
    pub recipient: HaneulAddress,
    pub amount: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "InputObjectKind")]
pub enum HaneulInputObjectKind {
    // A Move package, must be immutable.
    MovePackage(ObjectID),
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedMoveObject(HaneulObjectRef),
    // A Move object that's shared and mutable.
    SharedMoveObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        #[serde(default = "default_shared_object_mutability")]
        mutable: bool,
    },
}

const fn default_shared_object_mutability() -> bool {
    true
}

impl From<InputObjectKind> for HaneulInputObjectKind {
    fn from(input: InputObjectKind) -> Self {
        match input {
            InputObjectKind::MovePackage(id) => Self::MovePackage(id),
            InputObjectKind::ImmOrOwnedMoveObject(oref) => Self::ImmOrOwnedMoveObject(oref.into()),
            InputObjectKind::SharedMoveObject {
                id,
                initial_shared_version,
                mutable,
            } => Self::SharedMoveObject {
                id,
                initial_shared_version,
                mutable,
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[serde(rename = "ObjectInfo", rename_all = "camelCase")]
pub struct HaneulObjectInfo {
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    #[serde(rename = "type")]
    pub type_: String,
    pub owner: Owner,
    pub previous_transaction: TransactionDigest,
}

impl HaneulObjectInfo {
    pub fn to_object_ref(&self) -> ObjectRef {
        (self.object_id, self.version, self.digest)
    }
}

impl From<ObjectInfo> for HaneulObjectInfo {
    fn from(info: ObjectInfo) -> Self {
        Self {
            object_id: info.object_id,
            version: info.version,
            digest: info.digest,
            type_: format!("{}", info.type_),
            owner: info.owner,
            previous_transaction: info.previous_transaction,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectExistsResponse {
    object_ref: HaneulObjectRef,
    owner: Owner,
    previous_transaction: TransactionDigest,
    data: HaneulParsedData,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectNotExistsResponse {
    object_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename = "TypeTag", rename_all = "camelCase")]
pub struct HaneulTypeTag(String);

impl TryInto<TypeTag> for HaneulTypeTag {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<TypeTag, Self::Error> {
        parse_haneul_type_tag(&self.0)
    }
}

impl From<TypeTag> for HaneulTypeTag {
    fn from(tag: TypeTag) -> Self {
        Self(format!("{}", tag))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RPCTransactionRequestParams {
    TransferObjectRequestParams(TransferObjectParams),
    MoveCallRequestParams(MoveCallParams),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransferObjectParams {
    pub recipient: HaneulAddress,
    pub object_id: ObjectID,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoveCallParams {
    pub package_object_id: ObjectID,
    pub module: String,
    pub function: String,
    #[serde(default)]
    pub type_arguments: Vec<HaneulTypeTag>,
    pub arguments: Vec<HaneulJsonValue>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename = "EventFilter")]
pub enum HaneulEventFilter {
    Package(ObjectID),
    Module(String),
    /// Move StructTag string value of the event type e.g. `0x2::devnet_nft::MintNFTEvent`
    MoveEventType(String),
    MoveEventField {
        path: String,
        value: Value,
    },
    SenderAddress(HaneulAddress),
    EventType(EventType),
    ObjectId(ObjectID),
    All(Vec<HaneulEventFilter>),
    Any(Vec<HaneulEventFilter>),
    And(Box<HaneulEventFilter>, Box<HaneulEventFilter>),
    Or(Box<HaneulEventFilter>, Box<HaneulEventFilter>),
}

impl TryInto<EventFilter> for HaneulEventFilter {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<EventFilter, anyhow::Error> {
        use HaneulEventFilter::*;
        Ok(match self {
            Package(id) => EventFilter::Package(id),
            Module(module) => EventFilter::Module(Identifier::new(module)?),
            MoveEventType(event_type) => {
                // parse_haneul_struct_tag converts StructTag string e.g. `0x2::devnet_nft::MintNFTEvent` to StructTag object
                EventFilter::MoveEventType(parse_haneul_struct_tag(&event_type)?)
            }
            MoveEventField { path, value } => EventFilter::MoveEventField { path, value },
            SenderAddress(address) => EventFilter::SenderAddress(address),
            ObjectId(id) => EventFilter::ObjectId(id),
            All(filters) => EventFilter::MatchAll(
                filters
                    .into_iter()
                    .map(HaneulEventFilter::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            Any(filters) => EventFilter::MatchAny(
                filters
                    .into_iter()
                    .map(HaneulEventFilter::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            And(filter_a, filter_b) => All(vec![*filter_a, *filter_b]).try_into()?,
            Or(filter_a, filter_b) => Any(vec![*filter_a, *filter_b]).try_into()?,
            EventType(type_) => EventFilter::EventType(type_),
        })
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBytes {
    /// BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
    pub tx_bytes: Base64,
    /// the gas objects to be used
    pub gas: Vec<HaneulObjectRef>,
    /// objects to be used in this transaction
    pub input_objects: Vec<HaneulInputObjectKind>,
}

impl TransactionBytes {
    pub fn from_data(data: TransactionData) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tx_bytes: Base64::from_bytes(bcs::to_bytes(&data)?.as_slice()),
            gas: data
                .gas()
                .iter()
                .map(|obj_ref| HaneulObjectRef::from(*obj_ref))
                .collect(),
            input_objects: data
                .input_objects()?
                .into_iter()
                .map(HaneulInputObjectKind::from)
                .collect(),
        })
    }

    pub fn to_data(self) -> Result<TransactionData, anyhow::Error> {
        bcs::from_bytes::<TransactionData>(&self.tx_bytes.to_vec().map_err(|e| anyhow::anyhow!(e))?)
            .map_err(|e| anyhow::anyhow!(e))
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    /// Checkpoint's epoch ID
    pub epoch: EpochId,
    /// Checkpoint sequence number
    pub sequence_number: CheckpointSequenceNumber,
    /// Checkpoint digest
    pub digest: CheckpointDigest,
    /// Total number of transactions committed since genesis, including those in this
    /// checkpoint.
    pub network_total_transactions: u64,
    /// Digest of the previous checkpoint
    pub previous_digest: Option<CheckpointDigest>,
    /// The running total gas costs of all transactions included in the current epoch so far
    /// until this checkpoint.
    pub epoch_rolling_gas_cost_summary: GasCostSummary,
    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: CheckpointTimestamp,
    /// Present only on the final checkpoint of the epoch.
    pub end_of_epoch_data: Option<EndOfEpochData>,
    /// Transaction digests
    pub transactions: Vec<TransactionDigest>,
}

impl From<(CheckpointSummary, CheckpointContents)> for Checkpoint {
    fn from((summary, contents): (CheckpointSummary, CheckpointContents)) -> Self {
        let digest = summary.digest();
        let CheckpointSummary {
            epoch,
            sequence_number,
            network_total_transactions,
            previous_digest,
            epoch_rolling_gas_cost_summary,
            timestamp_ms,
            end_of_epoch_data,
            ..
        } = summary;

        Checkpoint {
            epoch,
            sequence_number,
            digest,
            network_total_transactions,
            previous_digest,
            epoch_rolling_gas_cost_summary,
            timestamp_ms,
            end_of_epoch_data,
            transactions: contents.iter().map(|digest| digest.transaction).collect(),
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckpointId {
    SequenceNumber(CheckpointSequenceNumber),
    Digest(CheckpointDigest),
}

impl From<CheckpointSequenceNumber> for CheckpointId {
    fn from(seq: CheckpointSequenceNumber) -> Self {
        Self::SequenceNumber(seq)
    }
}

impl From<CheckpointDigest> for CheckpointId {
    fn from(digest: CheckpointDigest) -> Self {
        Self::Digest(digest)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
#[serde(untagged, rename = "HaneulSystemState")]
pub enum HaneulSystemStateRpc {
    V1(HaneulSystemStateInnerV1),
}

impl From<HaneulSystemState> for HaneulSystemStateRpc {
    fn from(state: HaneulSystemState) -> Self {
        match state {
            HaneulSystemState::V1(state) => Self::V1(state),
        }
    }
}

impl From<HaneulSystemStateRpc> for HaneulSystemState {
    fn from(state: HaneulSystemStateRpc) -> Self {
        match state {
            HaneulSystemStateRpc::V1(state) => Self::V1(state),
        }
    }
}
