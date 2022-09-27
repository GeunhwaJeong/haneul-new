// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This file contain response types used by the GatewayAPI, most of the types mirrors it's internal type counterparts.
/// These mirrored types allow us to optimise the JSON serde without impacting the internal types, which are optimise for storage.
///
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;
use std::fmt::{Display, Formatter};

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, StructTypeParameter, Visibility};
use move_binary_format::normalized::{
    Field as NormalizedField, Function as HaneulNormalizedFunction, Module as NormalizedModule,
    Struct as NormalizedStruct, Type as NormalizedType,
};
use move_bytecode_utils::module_cache::GetModule;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::value::{MoveStruct, MoveStructLayout, MoveValue};
use schemars::JsonSchema;
use serde::ser::Error;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_with::serde_as;
use haneul_types::{parse_haneul_struct_tag, parse_haneul_type_tag};
use tracing::warn;

use haneul_json::HaneulJsonValue;
use haneul_types::base_types::{
    ObjectDigest, ObjectID, ObjectInfo, ObjectRef, SequenceNumber, HaneulAddress, TransactionDigest,
    TransactionEffectsDigest,
};
use haneul_types::committee::EpochId;
use haneul_types::crypto::{AuthorityStrongQuorumSignInfo, SignableBytes, Signature};
use haneul_types::error::HaneulError;
use haneul_types::event::{Event, TransferType};
use haneul_types::event::{EventEnvelope, EventType};
use haneul_types::filter::{EventFilter, TransactionFilter};
use haneul_types::gas::GasCostSummary;
use haneul_types::gas_coin::GasCoin;
use haneul_types::messages::{
    CallArg, CertifiedTransaction, CertifiedTransactionEffects, ExecuteTransactionResponse,
    ExecutionStatus, InputObjectKind, MoveModulePublish, ObjectArg, Pay, SingleTransactionKind,
    TransactionData, TransactionEffects, TransactionKind,
};
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;
use haneul_types::move_package::{disassemble_modules, MovePackage};
use haneul_types::object::{
    Data, MoveObject, Object, ObjectFormatOptions, ObjectRead, Owner, PastObjectRead,
};
use haneul_types::haneul_serde::{Base64, Encoding};

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

pub type GatewayTxSeqNumber = u64;
pub type HaneulMoveTypeParameterIndex = u16;

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
    U64,
    U128,
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
            NormalizedType::U64 => HaneulMoveNormalizedType::U64,
            NormalizedType::U128 => HaneulMoveNormalizedType::U128,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulTransactionResponse {
    pub certificate: HaneulCertifiedTransaction,
    pub effects: HaneulTransactionEffects,
    pub timestamp_ms: Option<u64>,
    pub parsed_data: Option<HaneulParsedTransactionResponse>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum HaneulParsedTransactionResponse {
    Publish(HaneulParsedPublishResponse),
    MergeCoin(HaneulParsedMergeCoinResponse),
    SplitCoin(HaneulParsedSplitCoinResponse),
}

impl HaneulParsedTransactionResponse {
    pub fn to_publish_response(self) -> Result<HaneulParsedPublishResponse, HaneulError> {
        match self {
            HaneulParsedTransactionResponse::Publish(resp) => Ok(resp),
            _ => Err(HaneulError::UnexpectedMessage),
        }
    }

    pub fn to_merge_coin_response(self) -> Result<HaneulParsedMergeCoinResponse, HaneulError> {
        match self {
            HaneulParsedTransactionResponse::MergeCoin(resp) => Ok(resp),
            _ => Err(HaneulError::UnexpectedMessage),
        }
    }

    pub fn to_split_coin_response(self) -> Result<HaneulParsedSplitCoinResponse, HaneulError> {
        match self {
            HaneulParsedTransactionResponse::SplitCoin(resp) => Ok(resp),
            _ => Err(HaneulError::UnexpectedMessage),
        }
    }
}

impl Display for HaneulParsedTransactionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HaneulParsedTransactionResponse::Publish(r) => r.fmt(f),
            HaneulParsedTransactionResponse::MergeCoin(r) => r.fmt(f),
            HaneulParsedTransactionResponse::SplitCoin(r) => r.fmt(f),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulExecuteTransactionResponse {
    ImmediateReturn {
        tx_digest: TransactionDigest,
    },
    TxCert {
        certificate: HaneulCertifiedTransaction,
    },
    // TODO: Change to CertifiedTransactionEffects eventually.
    EffectsCert {
        certificate: HaneulCertifiedTransaction,
        effects: HaneulCertifiedTransactionEffects,
    },
}

impl HaneulExecuteTransactionResponse {
    pub fn from_execute_transaction_response(
        resp: ExecuteTransactionResponse,
        tx_digest: TransactionDigest,
        resolver: &impl GetModule,
    ) -> Result<Self, anyhow::Error> {
        Ok(match resp {
            ExecuteTransactionResponse::ImmediateReturn => {
                HaneulExecuteTransactionResponse::ImmediateReturn { tx_digest }
            }
            ExecuteTransactionResponse::TxCert(certificate) => {
                HaneulExecuteTransactionResponse::TxCert {
                    certificate: (*certificate).try_into()?,
                }
            }
            ExecuteTransactionResponse::EffectsCert(cert) => {
                let (certificate, effects) = *cert;
                let certificate: HaneulCertifiedTransaction = certificate.try_into()?;
                let effects: HaneulCertifiedTransactionEffects =
                    HaneulCertifiedTransactionEffects::try_from(effects, resolver)?;
                HaneulExecuteTransactionResponse::EffectsCert {
                    certificate,
                    effects,
                }
            }
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulParsedSplitCoinResponse {
    /// The updated original coin object after split
    pub updated_coin: HaneulParsedObject,
    /// All the newly created coin objects generated from the split
    pub new_coins: Vec<HaneulParsedObject>,
    /// The updated gas payment object after deducting payment
    pub updated_gas: HaneulParsedObject,
}

impl Display for HaneulParsedSplitCoinResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "{}", "----- Split Coin Results ----".bold())?;

        let coin = GasCoin::try_from(&self.updated_coin).map_err(fmt::Error::custom)?;
        writeln!(writer, "Updated Coin : {}", coin)?;
        let mut new_coin_text = Vec::new();
        for coin in &self.new_coins {
            let coin = GasCoin::try_from(coin).map_err(fmt::Error::custom)?;
            new_coin_text.push(format!("{coin}"))
        }
        writeln!(
            writer,
            "New Coins : {}",
            new_coin_text.join(",\n            ")
        )?;
        let gas_coin = GasCoin::try_from(&self.updated_gas).map_err(fmt::Error::custom)?;
        writeln!(writer, "Updated Gas : {}", gas_coin)?;
        write!(f, "{}", writer)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulParsedMergeCoinResponse {
    /// The updated original coin object after merge
    pub updated_coin: HaneulParsedObject,
    /// The updated gas payment object after deducting payment
    pub updated_gas: HaneulParsedObject,
}

impl Display for HaneulParsedMergeCoinResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "{}", "----- Merge Coin Results ----".bold())?;

        let coin = GasCoin::try_from(&self.updated_coin).map_err(fmt::Error::custom)?;
        writeln!(writer, "Updated Coin : {}", coin)?;
        let gas_coin = GasCoin::try_from(&self.updated_gas).map_err(fmt::Error::custom)?;
        writeln!(writer, "Updated Gas : {}", gas_coin)?;
        write!(f, "{}", writer)
    }
}

pub type HaneulRawObject = HaneulObject<HaneulRawData>;
pub type HaneulParsedObject = HaneulObject<HaneulParsedData>;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct HaneulObject<T: HaneulData> {
    /// The meat of the object
    pub data: T,
    /// The owner that unlocks this object
    pub owner: Owner,
    /// The digest of the transaction that created or last mutated this object
    pub previous_transaction: TransactionDigest,
    /// The amount of HANEUL we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    pub storage_rebate: u64,
    pub reference: HaneulObjectRef,
}

impl TryInto<Object> for HaneulObject<HaneulRawData> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Object, Self::Error> {
        let data = match self.data {
            HaneulRawData::MoveObject(o) => {
                let struct_tag = parse_haneul_struct_tag(o.type_())?;
                Data::Move(unsafe {
                    MoveObject::new_from_execution(
                        struct_tag,
                        o.has_public_transfer,
                        o.version,
                        o.child_count,
                        o.bcs_bytes,
                    )
                })
            }
            HaneulRawData::Package(p) => Data::Package(MovePackage::new(p.id, &p.module_map)),
        };
        Ok(Object {
            data,
            owner: self.owner,
            previous_transaction: self.previous_transaction,
            storage_rebate: self.storage_rebate,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase", rename = "ObjectRef")]
pub struct HaneulObjectRef {
    /// Hex code as string representing the object id
    pub object_id: ObjectID,
    /// Object version.
    pub version: SequenceNumber,
    /// Base64 string representing the object digest
    pub digest: ObjectDigest,
}

impl HaneulObjectRef {
    pub fn to_object_ref(&self) -> ObjectRef {
        (self.object_id, self.version, self.digest)
    }
}

impl From<ObjectRef> for HaneulObjectRef {
    fn from(oref: ObjectRef) -> Self {
        Self {
            object_id: oref.0,
            version: oref.1,
            digest: oref.2,
        }
    }
}

impl Display for HaneulParsedObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let type_ = if self.data.type_().is_some() {
            "Move Object"
        } else {
            "Move Package"
        };
        let mut writer = String::new();
        writeln!(
            writer,
            "{}",
            format!(
                "----- {type_} ({}[{}]) -----",
                self.id(),
                self.version().value()
            )
            .bold()
        )?;
        writeln!(writer, "{}: {}", "Owner".bold().bright_black(), self.owner)?;
        writeln!(
            writer,
            "{}: {}",
            "Version".bold().bright_black(),
            self.version().value()
        )?;
        writeln!(
            writer,
            "{}: {}",
            "Storage Rebate".bold().bright_black(),
            self.storage_rebate
        )?;
        writeln!(
            writer,
            "{}: {:?}",
            "Previous Transaction".bold().bright_black(),
            self.previous_transaction
        )?;
        writeln!(writer, "{}", "----- Data -----".bold())?;
        write!(writer, "{}", &self.data)?;
        write!(f, "{}", writer)
    }
}

impl<T: HaneulData> HaneulObject<T> {
    pub fn id(&self) -> ObjectID {
        self.reference.object_id
    }
    pub fn version(&self) -> SequenceNumber {
        self.reference.version
    }

    pub fn try_from(o: Object, layout: Option<MoveStructLayout>) -> Result<Self, anyhow::Error> {
        let oref = o.compute_object_reference();
        let data = match o.data {
            Data::Move(m) => {
                let layout = layout.ok_or(HaneulError::ObjectSerializationError {
                    error: "Layout is required to convert Move object to json".to_owned(),
                })?;
                T::try_from_object(m, layout)?
            }
            Data::Package(p) => T::try_from_package(p)?,
        };
        Ok(Self {
            data,
            owner: o.owner,
            previous_transaction: o.previous_transaction,
            storage_rebate: o.storage_rebate,
            reference: oref.into(),
        })
    }
}

pub trait HaneulData: Sized {
    type ObjectType;
    type PackageType;
    fn try_from_object(object: MoveObject, layout: MoveStructLayout)
        -> Result<Self, anyhow::Error>;
    fn try_from_package(package: MovePackage) -> Result<Self, anyhow::Error>;
    fn try_as_move(&self) -> Option<&Self::ObjectType>;
    fn try_as_package(&self) -> Option<&Self::PackageType>;
    fn type_(&self) -> Option<&str>;
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(tag = "dataType", rename_all = "camelCase", rename = "Data")]
pub enum HaneulRawData {
    // Manually handle generic schema generation
    MoveObject(HaneulRawMoveObject),
    Package(HaneulRawMovePackage),
}

impl HaneulData for HaneulRawData {
    type ObjectType = HaneulRawMoveObject;
    type PackageType = HaneulRawMovePackage;

    fn try_from_object(object: MoveObject, _: MoveStructLayout) -> Result<Self, anyhow::Error> {
        Ok(Self::MoveObject(object.into()))
    }

    fn try_from_package(package: MovePackage) -> Result<Self, anyhow::Error> {
        Ok(Self::Package(package.into()))
    }

    fn try_as_move(&self) -> Option<&Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_as_package(&self) -> Option<&Self::PackageType> {
        match self {
            Self::MoveObject(_) => None,
            Self::Package(p) => Some(p),
        }
    }

    fn type_(&self) -> Option<&str> {
        match self {
            Self::MoveObject(o) => Some(o.type_.as_ref()),
            Self::Package(_) => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(tag = "dataType", rename_all = "camelCase", rename = "Data")]
pub enum HaneulParsedData {
    // Manually handle generic schema generation
    MoveObject(HaneulParsedMoveObject),
    Package(HaneulMovePackage),
}

impl HaneulData for HaneulParsedData {
    type ObjectType = HaneulParsedMoveObject;
    type PackageType = HaneulMovePackage;

    fn try_from_object(
        object: MoveObject,
        layout: MoveStructLayout,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self::MoveObject(HaneulParsedMoveObject::try_from_layout(
            object, layout,
        )?))
    }

    fn try_from_package(package: MovePackage) -> Result<Self, anyhow::Error> {
        Ok(Self::Package(HaneulMovePackage {
            disassembled: package.disassemble()?,
        }))
    }

    fn try_as_move(&self) -> Option<&Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_as_package(&self) -> Option<&Self::PackageType> {
        match self {
            Self::MoveObject(_) => None,
            Self::Package(p) => Some(p),
        }
    }

    fn type_(&self) -> Option<&str> {
        match self {
            Self::MoveObject(o) => Some(&o.type_),
            Self::Package(_) => None,
        }
    }
}

impl Display for HaneulParsedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulParsedData::MoveObject(o) => {
                writeln!(writer, "{}: {}", "type".bold().bright_black(), o.type_)?;
                write!(writer, "{}", &o.fields)?;
            }
            HaneulParsedData::Package(p) => {
                write!(
                    writer,
                    "{}: {:?}",
                    "Modules".bold().bright_black(),
                    p.disassembled.keys()
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

pub trait HaneulMoveObject: Sized {
    fn try_from_layout(object: MoveObject, layout: MoveStructLayout)
        -> Result<Self, anyhow::Error>;

    fn try_from(o: MoveObject, resolver: &impl GetModule) -> Result<Self, anyhow::Error> {
        let layout = o.get_layout(ObjectFormatOptions::default(), resolver)?;
        Self::try_from_layout(o, layout)
    }

    fn type_(&self) -> &str;
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "MoveObject")]
pub struct HaneulParsedMoveObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub has_public_transfer: bool,
    pub fields: HaneulMoveStruct,
}

impl HaneulMoveObject for HaneulParsedMoveObject {
    fn try_from_layout(
        object: MoveObject,
        layout: MoveStructLayout,
    ) -> Result<Self, anyhow::Error> {
        let move_struct = object.to_move_struct(&layout)?.into();

        Ok(
            if let HaneulMoveStruct::WithTypes { type_, fields } = move_struct {
                HaneulParsedMoveObject {
                    type_,
                    has_public_transfer: object.has_public_transfer(),
                    fields: HaneulMoveStruct::WithFields(fields),
                }
            } else {
                HaneulParsedMoveObject {
                    type_: object.type_.to_string(),
                    has_public_transfer: object.has_public_transfer(),
                    fields: move_struct,
                }
            },
        )
    }

    fn type_(&self) -> &str {
        &self.type_
    }
}

impl HaneulParsedMoveObject {
    fn try_type_and_fields_from_move_struct(
        type_: &StructTag,
        move_struct: MoveStruct,
    ) -> Result<(String, HaneulMoveStruct), anyhow::Error> {
        Ok(match move_struct.into() {
            HaneulMoveStruct::WithTypes { type_, fields } => {
                (type_, HaneulMoveStruct::WithFields(fields))
            }
            fields => (type_.to_string(), fields),
        })
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "RawMoveObject")]
pub struct HaneulRawMoveObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub has_public_transfer: bool,
    pub version: SequenceNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_count: Option<u32>,
    #[serde_as(as = "Base64")]
    #[schemars(with = "Base64")]
    pub bcs_bytes: Vec<u8>,
}

impl From<MoveObject> for HaneulRawMoveObject {
    fn from(o: MoveObject) -> Self {
        Self {
            type_: o.type_.to_string(),
            has_public_transfer: o.has_public_transfer(),
            version: o.version(),
            child_count: o.child_count(),
            bcs_bytes: o.into_contents(),
        }
    }
}

impl HaneulMoveObject for HaneulRawMoveObject {
    fn try_from_layout(
        object: MoveObject,
        _layout: MoveStructLayout,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            type_: object.type_.to_string(),
            has_public_transfer: object.has_public_transfer(),
            version: object.version(),
            child_count: object.child_count(),
            bcs_bytes: object.into_contents(),
        })
    }

    fn type_(&self) -> &str {
        &self.type_
    }
}

impl HaneulRawMoveObject {
    pub fn deserialize<'a, T: Deserialize<'a>>(&'a self) -> Result<T, anyhow::Error> {
        Ok(bcs::from_bytes(self.bcs_bytes.as_slice())?)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "RawMovePackage")]
pub struct HaneulRawMovePackage {
    pub id: ObjectID,
    #[schemars(with = "BTreeMap<String, Base64>")]
    #[serde_as(as = "BTreeMap<_, Base64>")]
    pub module_map: BTreeMap<String, Vec<u8>>,
}

impl From<MovePackage> for HaneulRawMovePackage {
    fn from(p: MovePackage) -> Self {
        Self {
            id: p.id(),
            module_map: p.serialized_module_map().clone(),
        }
    }
}

impl TryFrom<&HaneulParsedObject> for GasCoin {
    type Error = HaneulError;
    fn try_from(object: &HaneulParsedObject) -> Result<Self, Self::Error> {
        match &object.data {
            HaneulParsedData::MoveObject(o) => {
                if GasCoin::type_().to_string() == o.type_ {
                    return GasCoin::try_from(&o.fields);
                }
            }
            HaneulParsedData::Package(_) => {}
        }

        Err(HaneulError::TypeError {
            error: format!(
                "Gas object type is not a gas coin: {:?}",
                object.data.type_()
            ),
        })
    }
}

impl TryFrom<&HaneulMoveStruct> for GasCoin {
    type Error = HaneulError;
    fn try_from(move_struct: &HaneulMoveStruct) -> Result<Self, Self::Error> {
        match move_struct {
            HaneulMoveStruct::WithFields(fields) | HaneulMoveStruct::WithTypes { type_: _, fields } => {
                if let Some(HaneulMoveValue::Number(balance)) = fields.get("balance") {
                    if let Some(HaneulMoveValue::UID { id }) = fields.get("id") {
                        return Ok(GasCoin::new(*id, *balance));
                    }
                }
            }
            _ => {}
        }
        Err(HaneulError::TypeError {
            error: format!("Struct is not a gas coin: {move_struct:?}"),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HaneulParsedPublishResponse {
    /// The newly published package object reference.
    pub package: HaneulObjectRef,
    /// List of Move objects created as part of running the module initializers in the package
    pub created_objects: Vec<HaneulParsedObject>,
    /// The updated gas payment object after deducting payment
    pub updated_gas: HaneulParsedObject,
}

impl Display for HaneulParsedPublishResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "{}", "----- Publish Results ----".bold())?;
        writeln!(
            writer,
            "{}",
            format!(
                "The newly published package object ID: {:?}\n",
                self.package.object_id
            )
            .bold()
        )?;
        if !self.created_objects.is_empty() {
            writeln!(
                writer,
                "List of objects created by running module initializers:"
            )?;
            for obj in &self.created_objects {
                writeln!(writer, "{}\n", obj)?;
            }
        }
        let gas_coin = GasCoin::try_from(&self.updated_gas).map_err(fmt::Error::custom)?;
        writeln!(writer, "Updated Gas : {}", gas_coin)?;
        write!(f, "{}", writer)
    }
}

pub type GetObjectDataResponse = HaneulObjectRead<HaneulParsedData>;
pub type GetRawObjectDataResponse = HaneulObjectRead<HaneulRawData>;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "status", content = "details", rename = "ObjectRead")]
pub enum HaneulObjectRead<T: HaneulData> {
    Exists(HaneulObject<T>),
    NotExists(ObjectID),
    Deleted(HaneulObjectRef),
}

impl<T: HaneulData> HaneulObjectRead<T> {
    /// Returns a reference to the object if there is any, otherwise an Err if
    /// the object does not exist or is deleted.
    pub fn object(&self) -> Result<&HaneulObject<T>, HaneulError> {
        match &self {
            Self::Deleted(oref) => Err(HaneulError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::NotExists(id) => Err(HaneulError::ObjectNotFound { object_id: *id }),
            Self::Exists(o) => Ok(o),
        }
    }

    /// Returns the object value if there is any, otherwise an Err if
    /// the object does not exist or is deleted.
    pub fn into_object(self) -> Result<HaneulObject<T>, HaneulError> {
        match self {
            Self::Deleted(oref) => Err(HaneulError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::NotExists(id) => Err(HaneulError::ObjectNotFound { object_id: id }),
            Self::Exists(o) => Ok(o),
        }
    }
}

impl<T: HaneulData> TryFrom<ObjectRead> for HaneulObjectRead<T> {
    type Error = anyhow::Error;

    fn try_from(value: ObjectRead) -> Result<Self, Self::Error> {
        match value {
            ObjectRead::NotExists(id) => Ok(HaneulObjectRead::NotExists(id)),
            ObjectRead::Exists(_, o, layout) => {
                Ok(HaneulObjectRead::Exists(HaneulObject::try_from(o, layout)?))
            }
            ObjectRead::Deleted(oref) => Ok(HaneulObjectRead::Deleted(oref.into())),
        }
    }
}

pub type GetPastObjectDataResponse = HaneulPastObjectRead<HaneulParsedData>;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(tag = "status", content = "details", rename = "ObjectRead")]
pub enum HaneulPastObjectRead<T: HaneulData> {
    /// The object exists and is found with this version
    VersionFound(HaneulObject<T>),
    /// The object does not exist
    ObjectNotExists(ObjectID),
    /// The object is found to be deleted with this version
    ObjectDeleted(HaneulObjectRef),
    /// The object exists but not found with this version
    VersionNotFound(ObjectID, SequenceNumber),
    /// The asked object version is higher than the latest
    VersionTooHigh {
        object_id: ObjectID,
        asked_version: SequenceNumber,
        latest_version: SequenceNumber,
    },
}

impl<T: HaneulData> HaneulPastObjectRead<T> {
    /// Returns a reference to the object if there is any, otherwise an Err
    pub fn object(&self) -> Result<&HaneulObject<T>, HaneulError> {
        match &self {
            Self::ObjectDeleted(oref) => Err(HaneulError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::ObjectNotExists(id) => Err(HaneulError::ObjectNotFound { object_id: *id }),
            Self::VersionFound(o) => Ok(o),
            Self::VersionNotFound(id, seq_num) => Err(HaneulError::ObjectVersionNotFound {
                object_id: *id,
                version: *seq_num,
            }),
            Self::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            } => Err(HaneulError::ObjectSequenceNumberTooHigh {
                object_id: *object_id,
                asked_version: *asked_version,
                latest_version: *latest_version,
            }),
        }
    }

    /// Returns the object value if there is any, otherwise an Err
    pub fn into_object(self) -> Result<HaneulObject<T>, HaneulError> {
        match self {
            Self::ObjectDeleted(oref) => Err(HaneulError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::ObjectNotExists(id) => Err(HaneulError::ObjectNotFound { object_id: id }),
            Self::VersionFound(o) => Ok(o),
            Self::VersionNotFound(object_id, version) => {
                Err(HaneulError::ObjectVersionNotFound { object_id, version })
            }
            Self::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            } => Err(HaneulError::ObjectSequenceNumberTooHigh {
                object_id,
                asked_version,
                latest_version,
            }),
        }
    }
}

impl<T: HaneulData> TryFrom<PastObjectRead> for HaneulPastObjectRead<T> {
    type Error = anyhow::Error;

    fn try_from(value: PastObjectRead) -> Result<Self, Self::Error> {
        match value {
            PastObjectRead::ObjectNotExists(id) => Ok(HaneulPastObjectRead::ObjectNotExists(id)),
            PastObjectRead::VersionFound(_, o, layout) => Ok(HaneulPastObjectRead::VersionFound(
                HaneulObject::try_from(o, layout)?,
            )),
            PastObjectRead::ObjectDeleted(oref) => {
                Ok(HaneulPastObjectRead::ObjectDeleted(oref.into()))
            }
            PastObjectRead::VersionNotFound(id, seq_num) => {
                Ok(HaneulPastObjectRead::VersionNotFound(id, seq_num))
            }
            PastObjectRead::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            } => Ok(HaneulPastObjectRead::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveValue")]
pub enum HaneulMoveValue {
    Number(u64),
    Bool(bool),
    Address(HaneulAddress),
    Vector(Vec<HaneulMoveValue>),
    Bytearray(Base64),
    String(String),
    UID { id: ObjectID },
    Struct(HaneulMoveStruct),
    Option(Box<Option<HaneulMoveValue>>),
}

impl Display for HaneulMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulMoveValue::Number(value) => {
                write!(writer, "{}", value)?;
            }
            HaneulMoveValue::Bool(value) => {
                write!(writer, "{}", value)?;
            }
            HaneulMoveValue::Address(value) => {
                write!(writer, "{}", value)?;
            }
            HaneulMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            HaneulMoveValue::String(value) => {
                write!(writer, "{}", value)?;
            }
            HaneulMoveValue::UID { id } => {
                write!(writer, "{id}")?;
            }
            HaneulMoveValue::Struct(value) => {
                write!(writer, "{}", value)?;
            }
            HaneulMoveValue::Option(value) => {
                write!(writer, "{:?}", value)?;
            }
            HaneulMoveValue::Bytearray(value) => {
                write!(
                    writer,
                    "{:?}",
                    value.clone().to_vec().map_err(fmt::Error::custom)?
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
            MoveValue::U64(value) => HaneulMoveValue::Number(value),
            MoveValue::U128(value) => HaneulMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => HaneulMoveValue::Bool(value),
            MoveValue::Vector(value) => {
                // Try convert bytearray
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
                    return HaneulMoveValue::Bytearray(Base64::from_bytes(&bytearray));
                }
                HaneulMoveValue::Vector(value.into_iter().map(|value| value.into()).collect())
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
    pub fn to_json_value(self) -> Result<Value, serde_json::Error> {
        // Unwrap MoveStructs
        let unwrapped = match self {
            HaneulMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| match value {
                        HaneulMoveValue::Struct(move_struct) => move_struct.to_json_value(),
                        HaneulMoveValue::Vector(values) => {
                            HaneulMoveStruct::Runtime(values).to_json_value()
                        }
                        _ => serde_json::to_value(&value),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                serde_json::to_value(&values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            HaneulMoveStruct::WithTypes { type_: _, fields } | HaneulMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| {
                        let value = match value {
                            HaneulMoveValue::Struct(move_struct) => move_struct.to_json_value(),
                            HaneulMoveValue::Vector(values) => {
                                HaneulMoveStruct::Runtime(values).to_json_value()
                            }
                            _ => serde_json::to_value(&value),
                        };
                        value.map(|value| (key, value))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()?;
                serde_json::to_value(&fields)
            }
        }?;
        serde_json::to_value(&unwrapped)
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

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<HaneulMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let fields = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value.clone().into()))
        .collect::<BTreeMap<_, HaneulMoveValue>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(HaneulMoveValue::Bytearray(bytes)) = fields.get("bytes") {
                if let Ok(bytes) = bytes.to_vec() {
                    if let Ok(s) = String::from_utf8(bytes) {
                        return Some(HaneulMoveValue::String(s));
                    }
                }
            }
        }
        "0x2::url::Url" => {
            if let Some(url) = fields.get("url") {
                return Some(url.clone());
            }
        }
        "0x2::object::ID" => {
            if let Some(HaneulMoveValue::Address(id)) = fields.get("bytes") {
                return Some(HaneulMoveValue::Address(*id));
            }
        }
        "0x2::object::UID" => {
            if let Some(HaneulMoveValue::Address(address)) = fields.get("id") {
                return Some(HaneulMoveValue::UID {
                    id: ObjectID::from(*address),
                });
            }
        }
        "0x2::balance::Balance" => {
            if let Some(HaneulMoveValue::Number(value)) = fields.get("value") {
                return Some(HaneulMoveValue::Number(*value));
            }
        }
        "0x1::option::Option" => {
            if let Some(HaneulMoveValue::Vector(values)) = fields.get("vec") {
                return Some(HaneulMoveValue::Option(Box::new(values.first().cloned())));
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
#[serde(rename = "MovePackage")]
pub struct HaneulMovePackage {
    pub disassembled: BTreeMap<String, Value>,
}

impl TryFrom<MoveModulePublish> for HaneulMovePackage {
    type Error = anyhow::Error;

    fn try_from(m: MoveModulePublish) -> Result<Self, Self::Error> {
        Ok(Self {
            disassembled: disassemble_modules(m.modules.iter())?,
        })
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename = "TransactionData", rename_all = "camelCase")]
pub struct HaneulTransactionData {
    pub transactions: Vec<HaneulTransactionKind>,
    pub sender: HaneulAddress,
    pub gas_payment: HaneulObjectRef,
    pub gas_budget: u64,
}

impl Display for HaneulTransactionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            sender: data.signer(),
            gas_payment: data.gas().into(),
            gas_budget: data.gas_budget,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionKind")]
pub enum HaneulTransactionKind {
    /// Initiate an object transfer between addresses
    TransferObject(HaneulTransferObject),
    /// Pay one or more recipients from a set of input coins
    Pay(HaneulPay),
    /// Publish a new Move module
    Publish(HaneulMovePackage),
    /// Call a function in a published Move module
    Call(HaneulMoveCall),
    /// Initiate a HANEUL coin transfer between addresses
    TransferHaneul(HaneulTransferHaneul),
    /// A system transaction that will update epoch information on-chain.
    ChangeEpoch(HaneulChangeEpoch),
    // .. more transaction types go here
}

impl Display for HaneulTransactionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            Self::Publish(_p) => {
                write!(writer, "Transaction Kind : Publish")?;
            }
            Self::Call(c) => {
                writeln!(writer, "Transaction Kind : Call")?;
                writeln!(
                    writer,
                    "Package ID : {}",
                    c.package.object_id.to_hex_literal()
                )?;
                writeln!(writer, "Module : {}", c.module)?;
                writeln!(writer, "Function : {}", c.function)?;
                writeln!(writer, "Arguments : {:?}", c.arguments)?;
                write!(writer, "Type Arguments : {:?}", c.type_arguments)?;
            }
            Self::ChangeEpoch(e) => {
                writeln!(writer, "Transaction Kind: Epoch Change")?;
                writeln!(writer, "New epoch ID: {}", e.epoch)?;
                writeln!(writer, "Storage gas reward: {}", e.storage_charge)?;
                writeln!(writer, "Computation gas reward: {}", e.computation_charge)?;
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
            SingleTransactionKind::Publish(p) => Self::Publish(p.try_into()?),
            SingleTransactionKind::Call(c) => Self::Call(HaneulMoveCall {
                package: c.package.into(),
                module: c.module.to_string(),
                function: c.function.to_string(),
                type_arguments: c.type_arguments.iter().map(|ty| ty.to_string()).collect(),
                arguments: c
                    .arguments
                    .into_iter()
                    .map(|arg| match arg {
                        CallArg::Pure(p) => HaneulJsonValue::from_bcs_bytes(&p),
                        CallArg::Object(ObjectArg::ImmOrOwnedObject((id, _, _)))
                        | CallArg::Object(ObjectArg::SharedObject(id)) => {
                            HaneulJsonValue::new(Value::String(id.to_hex_literal()))
                        }
                        CallArg::ObjVec(vec) => HaneulJsonValue::new(Value::Array(
                            vec.iter()
                                .map(|obj_arg| match obj_arg {
                                    ObjectArg::ImmOrOwnedObject((id, _, _))
                                    | ObjectArg::SharedObject(id) => {
                                        Value::String(id.to_hex_literal())
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
            }),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "MoveCall", rename_all = "camelCase")]
pub struct HaneulMoveCall {
    pub package: HaneulObjectRef,
    pub module: String,
    pub function: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_arguments: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<HaneulJsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HaneulChangeEpoch {
    pub epoch: EpochId,
    pub storage_charge: u64,
    pub computation_charge: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "CertifiedTransaction", rename_all = "camelCase")]
pub struct HaneulCertifiedTransaction {
    pub transaction_digest: TransactionDigest,
    pub data: HaneulTransactionData,
    /// tx_signature is signed by the transaction sender, applied on `data`.
    pub tx_signature: Signature,
    /// authority signature information, if available, is signed by an authority, applied on `data`.
    pub auth_sign_info: AuthorityStrongQuorumSignInfo,
}

impl Display for HaneulCertifiedTransaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Transaction Hash: {:?}", self.transaction_digest)?;
        writeln!(writer, "Transaction Signature: {:?}", self.tx_signature)?;
        writeln!(
            writer,
            "Signed Authorities Bitmap: {:?}",
            self.auth_sign_info.signers_map
        )?;
        write!(writer, "{}", &self.data)?;
        write!(f, "{}", writer)
    }
}

impl TryFrom<CertifiedTransaction> for HaneulCertifiedTransaction {
    type Error = anyhow::Error;

    fn try_from(cert: CertifiedTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_digest: *cert.digest(),
            data: cert.signed_data.data.try_into()?,
            tx_signature: cert.signed_data.tx_signature,
            auth_sign_info: cert.auth_sign_info,
        })
    }
}

/// The certified Transaction Effects which has signatures from >= 2/3 of validators
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "CertifiedTransactionEffects", rename_all = "camelCase")]
pub struct HaneulCertifiedTransactionEffects {
    pub transaction_effects_digest: TransactionEffectsDigest,
    pub effects: HaneulTransactionEffects,
    /// authority signature information signed by the quorum of the validators.
    pub auth_sign_info: AuthorityStrongQuorumSignInfo,
}

impl Display for HaneulCertifiedTransactionEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(
            writer,
            "Transaction Effects Digest: {:?}",
            self.transaction_effects_digest
        )?;
        writeln!(writer, "Transaction Effects: {:?}", self.effects)?;
        writeln!(
            writer,
            "Signed Authorities Bitmap: {:?}",
            self.auth_sign_info.signers_map
        )?;
        write!(f, "{}", writer)
    }
}

impl HaneulCertifiedTransactionEffects {
    fn try_from(
        cert: CertifiedTransactionEffects,
        resolver: &impl GetModule,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            transaction_effects_digest: *cert.digest(),
            effects: HaneulTransactionEffects::try_from(cert.effects, resolver)?,
            auth_sign_info: cert.auth_signature,
        })
    }
}

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionEffects", rename_all = "camelCase")]
pub struct HaneulTransactionEffects {
    // The status of the execution
    pub status: HaneulExecutionStatus,
    pub gas_used: HaneulGasCostSummary,
    // The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_objects: Vec<HaneulObjectRef>,
    // The transaction digest
    pub transaction_digest: TransactionDigest,
    // ObjectRef and owner of new objects created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<OwnedObjectRef>,
    // ObjectRef and owner of mutated objects, including gas object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutated: Vec<OwnedObjectRef>,
    // ObjectRef and owner of objects that are unwrapped in this transaction.
    // Unwrapped objects are objects that were wrapped into other objects in the past,
    // and just got extracted out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped: Vec<OwnedObjectRef>,
    // Object Refs of objects now deleted (the old refs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<HaneulObjectRef>,
    // Object refs of objects now wrapped in other objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wrapped: Vec<HaneulObjectRef>,
    // The updated gas object reference. Have a dedicated field for convenient access.
    // It's also included in mutated.
    pub gas_object: OwnedObjectRef,
    /// The events emitted during execution. Note that only successful transactions emit events
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<HaneulEvent>,
    /// The set of transaction digests this transaction depends on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<TransactionDigest>,
}

impl HaneulTransactionEffects {
    /// Return an iterator of mutated objects, but excluding the gas object.
    pub fn mutated_excluding_gas(&self) -> impl Iterator<Item = &OwnedObjectRef> {
        self.mutated.iter().filter(|o| *o != &self.gas_object)
    }

    pub fn try_from(
        effect: TransactionEffects,
        resolver: &impl GetModule,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            status: effect.status.into(),
            gas_used: effect.gas_used.into(),
            shared_objects: to_haneul_object_ref(effect.shared_objects),
            transaction_digest: effect.transaction_digest,
            created: to_owned_ref(effect.created),
            mutated: to_owned_ref(effect.mutated),
            unwrapped: to_owned_ref(effect.unwrapped),
            deleted: to_haneul_object_ref(effect.deleted),
            wrapped: to_haneul_object_ref(effect.wrapped),
            gas_object: OwnedObjectRef {
                owner: effect.gas_object.1,
                reference: effect.gas_object.0.into(),
            },
            events: effect
                .events
                .into_iter()
                .map(|event| HaneulEvent::try_from(event, resolver))
                .collect::<Result<_, _>>()?,
            dependencies: effect.dependencies,
        })
    }
}

impl Display for HaneulTransactionEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            ExecutionStatus::Failure { error } => Self::Failure {
                error: format!("{:?}", error),
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
#[serde(rename = "ObjectRef")]
pub struct OwnedObjectRef {
    pub owner: Owner,
    pub reference: HaneulObjectRef,
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "EventEnvelope", rename_all = "camelCase")]
pub struct HaneulEventEnvelope {
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    pub timestamp: u64,
    /// Transaction digest of associated transaction, if any
    pub tx_digest: Option<TransactionDigest>,
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
    },
    /// Transfer objects to new address / wrap in another object / coin
    #[serde(rename_all = "camelCase")]
    TransferObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        recipient: Owner,
        object_id: ObjectID,
        version: SequenceNumber,
        type_: TransferType,
        amount: Option<u64>,
    },
    /// Delete object
    #[serde(rename_all = "camelCase")]
    DeleteObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        object_id: ObjectID,
    },
    /// New object creation
    #[serde(rename_all = "camelCase")]
    NewObject {
        package_id: ObjectID,
        transaction_module: String,
        sender: HaneulAddress,
        recipient: Owner,
        object_id: ObjectID,
    },
    /// Epoch change
    EpochChange(EpochId),
    /// New checkpoint
    Checkpoint(CheckpointSequenceNumber),
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

                // Resolver is not guaranteed to have knowledge of the event struct layout in the gateway server.
                let (type_, fields) = if let Ok(move_struct) =
                    Event::move_event_to_move_struct(&type_, &contents, resolver)
                {
                    let (type_, field) = HaneulParsedMoveObject::try_type_and_fields_from_move_struct(
                        &type_,
                        move_struct,
                    )?;
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
            Event::Publish { sender, package_id } => HaneulEvent::Publish { sender, package_id },
            Event::TransferObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_id,
                version,
                type_,
                amount,
            } => HaneulEvent::TransferObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                recipient,
                object_id,
                version,
                type_,
                amount,
            },
            Event::DeleteObject {
                package_id,
                transaction_module,
                sender,
                object_id,
            } => HaneulEvent::DeleteObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                object_id,
            },
            Event::NewObject {
                package_id,
                transaction_module,
                sender,
                recipient,
                object_id,
            } => HaneulEvent::NewObject {
                package_id,
                transaction_module: transaction_module.to_string(),
                sender,
                recipient,
                object_id,
            },
            Event::EpochChange(id) => HaneulEvent::EpochChange(id),
            Event::Checkpoint(seq) => HaneulEvent::Checkpoint(seq),
        })
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
            } => {
                if let HaneulEvent::Publish { package_id, sender } = other {
                    package_id == self_package_id && self_sender == sender
                } else {
                    false
                }
            }
            Event::TransferObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                recipient: self_recipient,
                type_: self_type,
                object_id: self_object_id,
                version: self_version,
                amount: self_amount,
            } => {
                if let HaneulEvent::TransferObject {
                    package_id,
                    transaction_module,
                    sender,
                    recipient,
                    object_id,
                    version,
                    type_,
                    amount,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_recipient == recipient
                        && self_object_id == object_id
                        && self_version == version
                        && self_type == type_
                        && self_amount == amount
                } else {
                    false
                }
            }
            Event::DeleteObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                object_id: self_object_id,
            } => {
                if let HaneulEvent::DeleteObject {
                    package_id,
                    transaction_module,
                    sender,
                    object_id,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_object_id == object_id
                } else {
                    false
                }
            }
            Event::NewObject {
                package_id: self_package_id,
                transaction_module: self_transaction_module,
                sender: self_sender,
                recipient: self_recipient,
                object_id: self_object_id,
            } => {
                if let HaneulEvent::NewObject {
                    package_id,
                    transaction_module,
                    sender,
                    recipient,
                    object_id,
                } = other
                {
                    package_id == self_package_id
                        && &self_transaction_module.to_string() == transaction_module
                        && self_sender == sender
                        && self_recipient == recipient
                        && self_object_id == object_id
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
    SharedMoveObject(ObjectID),
}

impl From<InputObjectKind> for HaneulInputObjectKind {
    fn from(input: InputObjectKind) -> Self {
        match input {
            InputObjectKind::MovePackage(id) => Self::MovePackage(id),
            InputObjectKind::ImmOrOwnedMoveObject(oref) => Self::ImmOrOwnedMoveObject(oref.into()),
            InputObjectKind::SharedMoveObject(id) => Self::SharedMoveObject(id),
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
            type_: info.type_,
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
#[serde(rename = "HaneulTransactionFilter")]
pub enum HaneulTransactionFilter {
    Any,
}

impl From<HaneulTransactionFilter> for TransactionFilter {
    fn from(filter: HaneulTransactionFilter) -> Self {
        use HaneulTransactionFilter::*;
        match filter {
            Any => TransactionFilter::Any,
        }
    }
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
    /// transaction data bytes, as base-64 encoded string
    pub tx_bytes: Base64,
    /// the gas object to be used
    pub gas: HaneulObjectRef,
    /// objects to be used in this transaction
    pub input_objects: Vec<HaneulInputObjectKind>,
}

impl TransactionBytes {
    pub fn from_data(data: TransactionData) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tx_bytes: Base64::from_bytes(&data.to_bytes()),
            gas: data.gas().into(),
            input_objects: data
                .input_objects()?
                .into_iter()
                .map(HaneulInputObjectKind::from)
                .collect(),
        })
    }

    pub fn to_data(self) -> Result<TransactionData, anyhow::Error> {
        TransactionData::from_signable_bytes(&self.tx_bytes.to_vec()?)
    }
}
