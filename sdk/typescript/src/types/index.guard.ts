// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/* eslint-disable */

/*
 * Generated type guards for "index.ts".
 * WARNING: Do not manually change this file.
 */
import { TransactionDigest, HaneulAddress, ObjectOwner, HaneulObjectRef, HaneulObjectInfo, ObjectContentFields, MovePackageContent, HaneulData, HaneulMoveObject, HaneulMovePackage, HaneulMoveFunctionArgTypesResponse, HaneulMoveFunctionArgType, HaneulMoveFunctionArgTypes, HaneulMoveNormalizedModules, HaneulMoveNormalizedModule, HaneulMoveModuleId, HaneulMoveNormalizedStruct, HaneulMoveStructTypeParameter, HaneulMoveNormalizedField, HaneulMoveNormalizedFunction, HaneulMoveVisibility, HaneulMoveTypeParameterIndex, HaneulMoveAbilitySet, HaneulMoveNormalizedType, HaneulMoveNormalizedTypeParameterType, HaneulMoveNormalizedStructType, HaneulObject, ObjectStatus, ObjectType, GetOwnedObjectsResponse, GetObjectDataResponse, ObjectDigest, ObjectId, SequenceNumber, MoveEvent, PublishEvent, TransferObjectEvent, DeleteObjectEvent, NewObjectEvent, HaneulEvent, MoveEventField, EventType, HaneulEventFilter, HaneulEventEnvelope, SubscriptionId, SubscriptionEvent, TransferObject, HaneulTransferHaneul, HaneulChangeEpoch, ExecuteTransactionRequestType, TransactionKindName, HaneulTransactionKind, HaneulTransactionData, EpochId, AuthorityQuorumSignInfo, CertifiedTransaction, GasCostSummary, ExecutionStatusType, ExecutionStatus, OwnedObjectRef, TransactionEffects, HaneulTransactionResponse, HaneulCertifiedTransactionEffects, HaneulExecuteTransactionResponse, GatewayTxSeqNumber, GetTxnDigestsResponse, MoveCall, HaneulJsonValue, EmptySignInfo, AuthorityName, AuthoritySignature, TransactionBytes, HaneulParsedMergeCoinResponse, HaneulParsedSplitCoinResponse, HaneulParsedPublishResponse, HaneulPackage, HaneulParsedTransactionResponse, DelegationData, DelegationHaneulObject, TransferObjectTx, TransferHaneulTx, PublishTx, ObjectArg, CallArg, StructTag, TypeTag, MoveCallTx, Transaction, TransactionKind, TransactionData } from "./index";

export function isTransactionDigest(obj: any, _argumentName?: string): obj is TransactionDigest {
    return (
        typeof obj === "string"
    )
}

export function isHaneulAddress(obj: any, _argumentName?: string): obj is HaneulAddress {
    return (
        typeof obj === "string"
    )
}

export function isObjectOwner(obj: any, _argumentName?: string): obj is ObjectOwner {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isTransactionDigest(obj.AddressOwner) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.ObjectOwner) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.SingleOwner) as boolean ||
            obj === "Shared" ||
            obj === "Immutable")
    )
}

export function isHaneulObjectRef(obj: any, _argumentName?: string): obj is HaneulObjectRef {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.digest) as boolean &&
        isTransactionDigest(obj.objectId) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.version) as boolean
    )
}

export function isHaneulObjectInfo(obj: any, _argumentName?: string): obj is HaneulObjectInfo {
    return (
        isHaneulObjectRef(obj) as boolean &&
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.type) as boolean &&
        isObjectOwner(obj.owner) as boolean &&
        isTransactionDigest(obj.previousTransaction) as boolean
    )
}

export function isObjectContentFields(obj: any, _argumentName?: string): obj is ObjectContentFields {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Object.entries<any>(obj)
            .every(([key, _value]) => (isTransactionDigest(key) as boolean))
    )
}

export function isMovePackageContent(obj: any, _argumentName?: string): obj is MovePackageContent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Object.entries<any>(obj)
            .every(([key, value]) => (isTransactionDigest(value) as boolean &&
                isTransactionDigest(key) as boolean))
    )
}

export function isHaneulData(obj: any, _argumentName?: string): obj is HaneulData {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isObjectType(obj.dataType) as boolean &&
            isHaneulMoveObject(obj) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isObjectType(obj.dataType) as boolean &&
            isHaneulMovePackage(obj) as boolean)
    )
}

export function isHaneulMoveObject(obj: any, _argumentName?: string): obj is HaneulMoveObject {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.type) as boolean &&
        isObjectContentFields(obj.fields) as boolean &&
        (typeof obj.has_public_transfer === "undefined" ||
            obj.has_public_transfer === false ||
            obj.has_public_transfer === true)
    )
}

export function isHaneulMovePackage(obj: any, _argumentName?: string): obj is HaneulMovePackage {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isMovePackageContent(obj.disassembled) as boolean
    )
}

export function isHaneulMoveFunctionArgTypesResponse(obj: any, _argumentName?: string): obj is HaneulMoveFunctionArgTypesResponse {
    return (
        Array.isArray(obj) &&
        obj.every((e: any) =>
            isHaneulMoveFunctionArgType(e) as boolean
        )
    )
}

export function isHaneulMoveFunctionArgType(obj: any, _argumentName?: string): obj is HaneulMoveFunctionArgType {
    return (
        (isTransactionDigest(obj) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.Object) as boolean)
    )
}

export function isHaneulMoveFunctionArgTypes(obj: any, _argumentName?: string): obj is HaneulMoveFunctionArgTypes {
    return (
        Array.isArray(obj) &&
        obj.every((e: any) =>
            isHaneulMoveFunctionArgType(e) as boolean
        )
    )
}

export function isHaneulMoveNormalizedModules(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedModules {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Object.entries<any>(obj)
            .every(([key, value]) => (isHaneulMoveNormalizedModule(value) as boolean &&
                isTransactionDigest(key) as boolean))
    )
}

export function isHaneulMoveNormalizedModule(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedModule {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.file_format_version) as boolean &&
        isTransactionDigest(obj.address) as boolean &&
        isTransactionDigest(obj.name) as boolean &&
        Array.isArray(obj.friends) &&
        obj.friends.every((e: any) =>
            isHaneulMoveModuleId(e) as boolean
        ) &&
        (obj.structs !== null &&
            typeof obj.structs === "object" ||
            typeof obj.structs === "function") &&
        Object.entries<any>(obj.structs)
            .every(([key, value]) => (isHaneulMoveNormalizedStruct(value) as boolean &&
                isTransactionDigest(key) as boolean)) &&
        (obj.exposed_functions !== null &&
            typeof obj.exposed_functions === "object" ||
            typeof obj.exposed_functions === "function") &&
        Object.entries<any>(obj.exposed_functions)
            .every(([key, value]) => (isHaneulMoveNormalizedFunction(value) as boolean &&
                isTransactionDigest(key) as boolean))
    )
}

export function isHaneulMoveModuleId(obj: any, _argumentName?: string): obj is HaneulMoveModuleId {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.address) as boolean &&
        isTransactionDigest(obj.name) as boolean
    )
}

export function isHaneulMoveNormalizedStruct(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedStruct {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveAbilitySet(obj.abilities) as boolean &&
        Array.isArray(obj.type_parameters) &&
        obj.type_parameters.every((e: any) =>
            isHaneulMoveStructTypeParameter(e) as boolean
        ) &&
        Array.isArray(obj.fields) &&
        obj.fields.every((e: any) =>
            isHaneulMoveNormalizedField(e) as boolean
        )
    )
}

export function isHaneulMoveStructTypeParameter(obj: any, _argumentName?: string): obj is HaneulMoveStructTypeParameter {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveAbilitySet(obj.constraints) as boolean &&
        typeof obj.is_phantom === "boolean"
    )
}

export function isHaneulMoveNormalizedField(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedField {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.name) as boolean &&
        isHaneulMoveNormalizedType(obj.type_) as boolean
    )
}

export function isHaneulMoveNormalizedFunction(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedFunction {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveVisibility(obj.visibility) as boolean &&
        typeof obj.is_entry === "boolean" &&
        Array.isArray(obj.type_parameters) &&
        obj.type_parameters.every((e: any) =>
            isHaneulMoveAbilitySet(e) as boolean
        ) &&
        Array.isArray(obj.parameters) &&
        obj.parameters.every((e: any) =>
            isHaneulMoveNormalizedType(e) as boolean
        ) &&
        Array.isArray(obj.return_) &&
        obj.return_.every((e: any) =>
            isHaneulMoveNormalizedType(e) as boolean
        )
    )
}

export function isHaneulMoveVisibility(obj: any, _argumentName?: string): obj is HaneulMoveVisibility {
    return (
        (obj === "Private" ||
            obj === "Public" ||
            obj === "Friend")
    )
}

export function isHaneulMoveTypeParameterIndex(obj: any, _argumentName?: string): obj is HaneulMoveTypeParameterIndex {
    return (
        typeof obj === "number"
    )
}

export function isHaneulMoveAbilitySet(obj: any, _argumentName?: string): obj is HaneulMoveAbilitySet {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Array.isArray(obj.abilities) &&
        obj.abilities.every((e: any) =>
            isTransactionDigest(e) as boolean
        )
    )
}

export function isHaneulMoveNormalizedType(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedType {
    return (
        (isTransactionDigest(obj) as boolean ||
            isHaneulMoveNormalizedTypeParameterType(obj) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulMoveNormalizedStructType(obj.Reference) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulMoveNormalizedStructType(obj.MutableReference) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulMoveNormalizedType(obj.Vector) as boolean ||
            isHaneulMoveNormalizedStructType(obj) as boolean)
    )
}

export function isHaneulMoveNormalizedTypeParameterType(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedTypeParameterType {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.TypeParameter) as boolean
    )
}

export function isHaneulMoveNormalizedStructType(obj: any, _argumentName?: string): obj is HaneulMoveNormalizedStructType {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (obj.Struct !== null &&
            typeof obj.Struct === "object" ||
            typeof obj.Struct === "function") &&
        isTransactionDigest(obj.Struct.address) as boolean &&
        isTransactionDigest(obj.Struct.module) as boolean &&
        isTransactionDigest(obj.Struct.name) as boolean &&
        Array.isArray(obj.Struct.type_arguments) &&
        obj.Struct.type_arguments.every((e: any) =>
            isHaneulMoveNormalizedTypeParameterType(e) as boolean
        )
    )
}

export function isHaneulObject(obj: any, _argumentName?: string): obj is HaneulObject {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulData(obj.data) as boolean &&
        isObjectOwner(obj.owner) as boolean &&
        isTransactionDigest(obj.previousTransaction) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.storageRebate) as boolean &&
        isHaneulObjectRef(obj.reference) as boolean
    )
}

export function isObjectStatus(obj: any, _argumentName?: string): obj is ObjectStatus {
    return (
        (obj === "Exists" ||
            obj === "NotExists" ||
            obj === "Deleted")
    )
}

export function isObjectType(obj: any, _argumentName?: string): obj is ObjectType {
    return (
        (obj === "moveObject" ||
            obj === "package")
    )
}

export function isGetOwnedObjectsResponse(obj: any, _argumentName?: string): obj is GetOwnedObjectsResponse {
    return (
        Array.isArray(obj) &&
        obj.every((e: any) =>
            isHaneulObjectInfo(e) as boolean
        )
    )
}

export function isGetObjectDataResponse(obj: any, _argumentName?: string): obj is GetObjectDataResponse {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isObjectStatus(obj.status) as boolean &&
        (isTransactionDigest(obj.details) as boolean ||
            isHaneulObjectRef(obj.details) as boolean ||
            isHaneulObject(obj.details) as boolean)
    )
}

export function isObjectDigest(obj: any, _argumentName?: string): obj is ObjectDigest {
    return (
        typeof obj === "string"
    )
}

export function isObjectId(obj: any, _argumentName?: string): obj is ObjectId {
    return (
        typeof obj === "string"
    )
}

export function isSequenceNumber(obj: any, _argumentName?: string): obj is SequenceNumber {
    return (
        typeof obj === "number"
    )
}

export function isMoveEvent(obj: any, _argumentName?: string): obj is MoveEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.packageId) as boolean &&
        isTransactionDigest(obj.transactionModule) as boolean &&
        isTransactionDigest(obj.sender) as boolean &&
        isTransactionDigest(obj.type) as boolean &&
        (obj.fields !== null &&
            typeof obj.fields === "object" ||
            typeof obj.fields === "function") &&
        isTransactionDigest(obj.bcs) as boolean
    )
}

export function isPublishEvent(obj: any, _argumentName?: string): obj is PublishEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.sender) as boolean &&
        isTransactionDigest(obj.packageId) as boolean
    )
}

export function isTransferObjectEvent(obj: any, _argumentName?: string): obj is TransferObjectEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.packageId) as boolean &&
        isTransactionDigest(obj.transactionModule) as boolean &&
        isTransactionDigest(obj.sender) as boolean &&
        isObjectOwner(obj.recipient) as boolean &&
        isTransactionDigest(obj.objectId) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.version) as boolean &&
        isTransactionDigest(obj.type) as boolean &&
        (obj.amount === null ||
            isHaneulMoveTypeParameterIndex(obj.amount) as boolean)
    )
}

export function isDeleteObjectEvent(obj: any, _argumentName?: string): obj is DeleteObjectEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.packageId) as boolean &&
        isTransactionDigest(obj.transactionModule) as boolean &&
        isTransactionDigest(obj.sender) as boolean &&
        isTransactionDigest(obj.objectId) as boolean
    )
}

export function isNewObjectEvent(obj: any, _argumentName?: string): obj is NewObjectEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.packageId) as boolean &&
        isTransactionDigest(obj.transactionModule) as boolean &&
        isTransactionDigest(obj.sender) as boolean &&
        isObjectOwner(obj.recipient) as boolean &&
        isTransactionDigest(obj.objectId) as boolean
    )
}

export function isHaneulEvent(obj: any, _argumentName?: string): obj is HaneulEvent {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isMoveEvent(obj.moveEvent) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isPublishEvent(obj.publish) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransferObjectEvent(obj.transferObject) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isDeleteObjectEvent(obj.deleteObject) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isNewObjectEvent(obj.newObject) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            typeof obj.epochChange === "bigint" ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            typeof obj.checkpoint === "bigint")
    )
}

export function isMoveEventField(obj: any, _argumentName?: string): obj is MoveEventField {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.path) as boolean &&
        isHaneulJsonValue(obj.value) as boolean
    )
}

export function isEventType(obj: any, _argumentName?: string): obj is EventType {
    return (
        (obj === "MoveEvent" ||
            obj === "Publish" ||
            obj === "TransferObject" ||
            obj === "DeleteObject" ||
            obj === "NewObject" ||
            obj === "EpochChange" ||
            obj === "Checkpoint")
    )
}

export function isHaneulEventFilter(obj: any, _argumentName?: string): obj is HaneulEventFilter {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isTransactionDigest(obj.Package) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.Module) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.MoveEventType) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isMoveEventField(obj.MoveEventField) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.SenderAddress) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isEventType(obj.EventType) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            Array.isArray(obj.All) &&
            obj.All.every((e: any) =>
                isHaneulEventFilter(e) as boolean
            ) ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            Array.isArray(obj.Any) &&
            obj.Any.every((e: any) =>
                isHaneulEventFilter(e) as boolean
            ) ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            Array.isArray(obj.And) &&
            isHaneulEventFilter(obj.And[0]) as boolean &&
            isHaneulEventFilter(obj.And[1]) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            Array.isArray(obj.Or) &&
            isHaneulEventFilter(obj.Or[0]) as boolean &&
            isHaneulEventFilter(obj.Or[1]) as boolean)
    )
}

export function isHaneulEventEnvelope(obj: any, _argumentName?: string): obj is HaneulEventEnvelope {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.timestamp) as boolean &&
        isTransactionDigest(obj.txDigest) as boolean &&
        isHaneulEvent(obj.event) as boolean
    )
}

export function isSubscriptionId(obj: any, _argumentName?: string): obj is SubscriptionId {
    return (
        typeof obj === "number"
    )
}

export function isSubscriptionEvent(obj: any, _argumentName?: string): obj is SubscriptionEvent {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.subscription) as boolean &&
        isHaneulEventEnvelope(obj.result) as boolean
    )
}

export function isTransferObject(obj: any, _argumentName?: string): obj is TransferObject {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.recipient) as boolean &&
        isHaneulObjectRef(obj.objectRef) as boolean
    )
}

export function isHaneulTransferHaneul(obj: any, _argumentName?: string): obj is HaneulTransferHaneul {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.recipient) as boolean &&
        (obj.amount === null ||
            isHaneulMoveTypeParameterIndex(obj.amount) as boolean)
    )
}

export function isHaneulChangeEpoch(obj: any, _argumentName?: string): obj is HaneulChangeEpoch {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.epoch) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.storage_charge) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.computation_charge) as boolean
    )
}

export function isExecuteTransactionRequestType(obj: any, _argumentName?: string): obj is ExecuteTransactionRequestType {
    return (
        (obj === "ImmediateReturn" ||
            obj === "WaitForTxCert" ||
            obj === "WaitForEffectsCert")
    )
}

export function isTransactionKindName(obj: any, _argumentName?: string): obj is TransactionKindName {
    return (
        (obj === "Publish" ||
            obj === "TransferObject" ||
            obj === "Call" ||
            obj === "TransferHaneul" ||
            obj === "ChangeEpoch")
    )
}

export function isHaneulTransactionKind(obj: any, _argumentName?: string): obj is HaneulTransactionKind {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isTransferObject(obj.TransferObject) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulMovePackage(obj.Publish) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isMoveCall(obj.Call) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulTransferHaneul(obj.TransferHaneul) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulChangeEpoch(obj.ChangeEpoch) as boolean)
    )
}

export function isHaneulTransactionData(obj: any, _argumentName?: string): obj is HaneulTransactionData {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Array.isArray(obj.transactions) &&
        obj.transactions.every((e: any) =>
            isHaneulTransactionKind(e) as boolean
        ) &&
        isTransactionDigest(obj.sender) as boolean &&
        isHaneulObjectRef(obj.gasPayment) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.gasBudget) as boolean
    )
}

export function isEpochId(obj: any, _argumentName?: string): obj is EpochId {
    return (
        typeof obj === "number"
    )
}

export function isAuthorityQuorumSignInfo(obj: any, _argumentName?: string): obj is AuthorityQuorumSignInfo {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.epoch) as boolean &&
        Array.isArray(obj.signature) &&
        obj.signature.every((e: any) =>
            isTransactionDigest(e) as boolean
        )
    )
}

export function isCertifiedTransaction(obj: any, _argumentName?: string): obj is CertifiedTransaction {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.transactionDigest) as boolean &&
        isHaneulTransactionData(obj.data) as boolean &&
        isTransactionDigest(obj.txSignature) as boolean &&
        isAuthorityQuorumSignInfo(obj.authSignInfo) as boolean
    )
}

export function isGasCostSummary(obj: any, _argumentName?: string): obj is GasCostSummary {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulMoveTypeParameterIndex(obj.computationCost) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.storageCost) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.storageRebate) as boolean
    )
}

export function isExecutionStatusType(obj: any, _argumentName?: string): obj is ExecutionStatusType {
    return (
        (obj === "success" ||
            obj === "failure")
    )
}

export function isExecutionStatus(obj: any, _argumentName?: string): obj is ExecutionStatus {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isExecutionStatusType(obj.status) as boolean &&
        (typeof obj.error === "undefined" ||
            isTransactionDigest(obj.error) as boolean)
    )
}

export function isOwnedObjectRef(obj: any, _argumentName?: string): obj is OwnedObjectRef {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isObjectOwner(obj.owner) as boolean &&
        isHaneulObjectRef(obj.reference) as boolean
    )
}

export function isTransactionEffects(obj: any, _argumentName?: string): obj is TransactionEffects {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isExecutionStatus(obj.status) as boolean &&
        isGasCostSummary(obj.gasUsed) as boolean &&
        (typeof obj.sharedObjects === "undefined" ||
            Array.isArray(obj.sharedObjects) &&
            obj.sharedObjects.every((e: any) =>
                isHaneulObjectRef(e) as boolean
            )) &&
        isTransactionDigest(obj.transactionDigest) as boolean &&
        (typeof obj.created === "undefined" ||
            Array.isArray(obj.created) &&
            obj.created.every((e: any) =>
                isOwnedObjectRef(e) as boolean
            )) &&
        (typeof obj.mutated === "undefined" ||
            Array.isArray(obj.mutated) &&
            obj.mutated.every((e: any) =>
                isOwnedObjectRef(e) as boolean
            )) &&
        (typeof obj.unwrapped === "undefined" ||
            Array.isArray(obj.unwrapped) &&
            obj.unwrapped.every((e: any) =>
                isOwnedObjectRef(e) as boolean
            )) &&
        (typeof obj.deleted === "undefined" ||
            Array.isArray(obj.deleted) &&
            obj.deleted.every((e: any) =>
                isHaneulObjectRef(e) as boolean
            )) &&
        (typeof obj.wrapped === "undefined" ||
            Array.isArray(obj.wrapped) &&
            obj.wrapped.every((e: any) =>
                isHaneulObjectRef(e) as boolean
            )) &&
        isOwnedObjectRef(obj.gasObject) as boolean &&
        (typeof obj.events === "undefined" ||
            Array.isArray(obj.events)) &&
        (typeof obj.dependencies === "undefined" ||
            Array.isArray(obj.dependencies) &&
            obj.dependencies.every((e: any) =>
                isTransactionDigest(e) as boolean
            ))
    )
}

export function isHaneulTransactionResponse(obj: any, _argumentName?: string): obj is HaneulTransactionResponse {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isCertifiedTransaction(obj.certificate) as boolean &&
        isTransactionEffects(obj.effects) as boolean &&
        (obj.timestamp_ms === null ||
            isHaneulMoveTypeParameterIndex(obj.timestamp_ms) as boolean) &&
        (obj.parsed_data === null ||
            (obj.parsed_data !== null &&
                typeof obj.parsed_data === "object" ||
                typeof obj.parsed_data === "function") &&
            isHaneulParsedSplitCoinResponse(obj.parsed_data.SplitCoin) as boolean ||
            (obj.parsed_data !== null &&
                typeof obj.parsed_data === "object" ||
                typeof obj.parsed_data === "function") &&
            isHaneulParsedMergeCoinResponse(obj.parsed_data.MergeCoin) as boolean ||
            (obj.parsed_data !== null &&
                typeof obj.parsed_data === "object" ||
                typeof obj.parsed_data === "function") &&
            isHaneulParsedPublishResponse(obj.parsed_data.Publish) as boolean)
    )
}

export function isHaneulCertifiedTransactionEffects(obj: any, _argumentName?: string): obj is HaneulCertifiedTransactionEffects {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionEffects(obj.effects) as boolean
    )
}

export function isHaneulExecuteTransactionResponse(obj: any, _argumentName?: string): obj is HaneulExecuteTransactionResponse {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            (obj.ImmediateReturn !== null &&
                typeof obj.ImmediateReturn === "object" ||
                typeof obj.ImmediateReturn === "function") &&
            isTransactionDigest(obj.ImmediateReturn.tx_digest) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            (obj.TxCert !== null &&
                typeof obj.TxCert === "object" ||
                typeof obj.TxCert === "function") &&
            isCertifiedTransaction(obj.TxCert.certificate) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            (obj.EffectsCert !== null &&
                typeof obj.EffectsCert === "object" ||
                typeof obj.EffectsCert === "function") &&
            isCertifiedTransaction(obj.EffectsCert.certificate) as boolean &&
            isHaneulCertifiedTransactionEffects(obj.EffectsCert.effects) as boolean)
    )
}

export function isGatewayTxSeqNumber(obj: any, _argumentName?: string): obj is GatewayTxSeqNumber {
    return (
        typeof obj === "number"
    )
}

export function isGetTxnDigestsResponse(obj: any, _argumentName?: string): obj is GetTxnDigestsResponse {
    return (
        Array.isArray(obj) &&
        obj.every((e: any) =>
            Array.isArray(e) &&
            isHaneulMoveTypeParameterIndex(e[0]) as boolean &&
            isTransactionDigest(e[1]) as boolean
        )
    )
}

export function isMoveCall(obj: any, _argumentName?: string): obj is MoveCall {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulObjectRef(obj.package) as boolean &&
        isTransactionDigest(obj.module) as boolean &&
        isTransactionDigest(obj.function) as boolean &&
        (typeof obj.typeArguments === "undefined" ||
            Array.isArray(obj.typeArguments) &&
            obj.typeArguments.every((e: any) =>
                isTransactionDigest(e) as boolean
            )) &&
        (typeof obj.arguments === "undefined" ||
            Array.isArray(obj.arguments) &&
            obj.arguments.every((e: any) =>
                isHaneulJsonValue(e) as boolean
            ))
    )
}

export function isHaneulJsonValue(obj: any, _argumentName?: string): obj is HaneulJsonValue {
    return (
        (isTransactionDigest(obj) as boolean ||
            isHaneulMoveTypeParameterIndex(obj) as boolean ||
            obj === false ||
            obj === true ||
            Array.isArray(obj) &&
            obj.every((e: any) =>
                isHaneulJsonValue(e) as boolean
            ))
    )
}

export function isEmptySignInfo(obj: any, _argumentName?: string): obj is EmptySignInfo {
    return (
        typeof obj === "object"
    )
}

export function isAuthorityName(obj: any, _argumentName?: string): obj is AuthorityName {
    return (
        typeof obj === "string"
    )
}

export function isAuthoritySignature(obj: any, _argumentName?: string): obj is AuthoritySignature {
    return (
        typeof obj === "string"
    )
}

export function isTransactionBytes(obj: any, _argumentName?: string): obj is TransactionBytes {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.txBytes) as boolean &&
        isHaneulObjectRef(obj.gas) as boolean
    )
}

export function isHaneulParsedMergeCoinResponse(obj: any, _argumentName?: string): obj is HaneulParsedMergeCoinResponse {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulObject(obj.updatedCoin) as boolean &&
        isHaneulObject(obj.updatedGas) as boolean
    )
}

export function isHaneulParsedSplitCoinResponse(obj: any, _argumentName?: string): obj is HaneulParsedSplitCoinResponse {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isHaneulObject(obj.updatedCoin) as boolean &&
        Array.isArray(obj.newCoins) &&
        obj.newCoins.every((e: any) =>
            isHaneulObject(e) as boolean
        ) &&
        isHaneulObject(obj.updatedGas) as boolean
    )
}

export function isHaneulParsedPublishResponse(obj: any, _argumentName?: string): obj is HaneulParsedPublishResponse {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        Array.isArray(obj.createdObjects) &&
        obj.createdObjects.every((e: any) =>
            isHaneulObject(e) as boolean
        ) &&
        isHaneulPackage(obj.package) as boolean &&
        isHaneulObject(obj.updatedGas) as boolean
    )
}

export function isHaneulPackage(obj: any, _argumentName?: string): obj is HaneulPackage {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.digest) as boolean &&
        isTransactionDigest(obj.objectId) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.version) as boolean
    )
}

export function isHaneulParsedTransactionResponse(obj: any, _argumentName?: string): obj is HaneulParsedTransactionResponse {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isHaneulParsedSplitCoinResponse(obj.SplitCoin) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulParsedMergeCoinResponse(obj.MergeCoin) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isHaneulParsedPublishResponse(obj.Publish) as boolean)
    )
}

export function isDelegationData(obj: any, _argumentName?: string): obj is DelegationData {
    return (
        isHaneulMoveObject(obj) as boolean &&
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isObjectType(obj.dataType) as boolean &&
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        obj.type === "0x2::delegation::Delegation" &&
        (obj.fields !== null &&
            typeof obj.fields === "object" ||
            typeof obj.fields === "function") &&
        (isHaneulMoveTypeParameterIndex(obj.fields.active_delegation) as boolean ||
            (obj.fields.active_delegation !== null &&
                typeof obj.fields.active_delegation === "object" ||
                typeof obj.fields.active_delegation === "function") &&
            (obj.fields.active_delegation.fields !== null &&
                typeof obj.fields.active_delegation.fields === "object" ||
                typeof obj.fields.active_delegation.fields === "function") &&
            obj.fields.active_delegation.fields.vec === "" &&
            isTransactionDigest(obj.fields.active_delegation.type) as boolean) &&
        isHaneulMoveTypeParameterIndex(obj.fields.delegate_amount) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.fields.next_reward_unclaimed_epoch) as boolean &&
        isTransactionDigest(obj.fields.validator_address) as boolean &&
        (obj.fields.info !== null &&
            typeof obj.fields.info === "object" ||
            typeof obj.fields.info === "function") &&
        isTransactionDigest(obj.fields.info.id) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.fields.info.version) as boolean &&
        (isHaneulMoveObject(obj.fields.coin_locked_until_epoch) as boolean ||
            (obj.fields.coin_locked_until_epoch !== null &&
                typeof obj.fields.coin_locked_until_epoch === "object" ||
                typeof obj.fields.coin_locked_until_epoch === "function") &&
            (obj.fields.coin_locked_until_epoch.fields !== null &&
                typeof obj.fields.coin_locked_until_epoch.fields === "object" ||
                typeof obj.fields.coin_locked_until_epoch.fields === "function") &&
            obj.fields.coin_locked_until_epoch.fields.vec === "" &&
            isTransactionDigest(obj.fields.coin_locked_until_epoch.type) as boolean) &&
        (isHaneulMoveTypeParameterIndex(obj.fields.ending_epoch) as boolean ||
            (obj.fields.ending_epoch !== null &&
                typeof obj.fields.ending_epoch === "object" ||
                typeof obj.fields.ending_epoch === "function") &&
            (obj.fields.ending_epoch.fields !== null &&
                typeof obj.fields.ending_epoch.fields === "object" ||
                typeof obj.fields.ending_epoch.fields === "function") &&
            obj.fields.ending_epoch.fields.vec === "" &&
            isTransactionDigest(obj.fields.ending_epoch.type) as boolean)
    )
}

export function isDelegationHaneulObject(obj: any, _argumentName?: string): obj is DelegationHaneulObject {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isObjectOwner(obj.owner) as boolean &&
        isTransactionDigest(obj.previousTransaction) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.storageRebate) as boolean &&
        isHaneulObjectRef(obj.reference) as boolean &&
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isDelegationData(obj.data) as boolean
    )
}

export function isTransferObjectTx(obj: any, _argumentName?: string): obj is TransferObjectTx {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (obj.TransferObject !== null &&
            typeof obj.TransferObject === "object" ||
            typeof obj.TransferObject === "function") &&
        isTransactionDigest(obj.TransferObject.recipient) as boolean &&
        isHaneulObjectRef(obj.TransferObject.object_ref) as boolean
    )
}

export function isTransferHaneulTx(obj: any, _argumentName?: string): obj is TransferHaneulTx {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (obj.TransferHaneul !== null &&
            typeof obj.TransferHaneul === "object" ||
            typeof obj.TransferHaneul === "function") &&
        isTransactionDigest(obj.TransferHaneul.recipient) as boolean &&
        ((obj.TransferHaneul.amount !== null &&
            typeof obj.TransferHaneul.amount === "object" ||
            typeof obj.TransferHaneul.amount === "function") &&
            isHaneulMoveTypeParameterIndex(obj.TransferHaneul.amount.Some) as boolean ||
            (obj.TransferHaneul.amount !== null &&
                typeof obj.TransferHaneul.amount === "object" ||
                typeof obj.TransferHaneul.amount === "function") &&
            obj.TransferHaneul.amount.None === null)
    )
}

export function isPublishTx(obj: any, _argumentName?: string): obj is PublishTx {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (obj.Publish !== null &&
            typeof obj.Publish === "object" ||
            typeof obj.Publish === "function") &&
        (obj.Publish.modules !== null &&
            typeof obj.Publish.modules === "object" ||
            typeof obj.Publish.modules === "function") &&
        isHaneulMoveTypeParameterIndex(obj.Publish.modules.length) as boolean
    )
}

export function isObjectArg(obj: any, _argumentName?: string): obj is ObjectArg {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isHaneulObjectRef(obj.ImmOrOwned) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTransactionDigest(obj.Shared) as boolean)
    )
}

export function isCallArg(obj: any, _argumentName?: string): obj is CallArg {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            (obj.Pure !== null &&
                typeof obj.Pure === "object" ||
                typeof obj.Pure === "function") &&
            isHaneulMoveTypeParameterIndex(obj.Pure.length) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isObjectArg(obj.Object) as boolean)
    )
}

export function isStructTag(obj: any, _argumentName?: string): obj is StructTag {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        isTransactionDigest(obj.address) as boolean &&
        isTransactionDigest(obj.module) as boolean &&
        isTransactionDigest(obj.name) as boolean &&
        Array.isArray(obj.typeParams) &&
        obj.typeParams.every((e: any) =>
            isTypeTag(e) as boolean
        )
    )
}

export function isTypeTag(obj: any, _argumentName?: string): obj is TypeTag {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            obj.bool === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            obj.u8 === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            obj.u64 === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            obj.u128 === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            obj.address === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            obj.signer === null ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isTypeTag(obj.vector) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            isStructTag(obj.struct) as boolean)
    )
}

export function isMoveCallTx(obj: any, _argumentName?: string): obj is MoveCallTx {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (obj.Call !== null &&
            typeof obj.Call === "object" ||
            typeof obj.Call === "function") &&
        isHaneulObjectRef(obj.Call.package) as boolean &&
        isTransactionDigest(obj.Call.module) as boolean &&
        isTransactionDigest(obj.Call.function) as boolean &&
        Array.isArray(obj.Call.typeArguments) &&
        obj.Call.typeArguments.every((e: any) =>
            isTypeTag(e) as boolean
        ) &&
        Array.isArray(obj.Call.arguments) &&
        obj.Call.arguments.every((e: any) =>
            isCallArg(e) as boolean
        )
    )
}

export function isTransaction(obj: any, _argumentName?: string): obj is Transaction {
    return (
        (isTransferObjectTx(obj) as boolean ||
            isTransferHaneulTx(obj) as boolean ||
            isPublishTx(obj) as boolean ||
            isMoveCallTx(obj) as boolean)
    )
}

export function isTransactionKind(obj: any, _argumentName?: string): obj is TransactionKind {
    return (
        ((obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
            isTransaction(obj.Single) as boolean ||
            (obj !== null &&
                typeof obj === "object" ||
                typeof obj === "function") &&
            Array.isArray(obj.Batch) &&
            obj.Batch.every((e: any) =>
                isTransaction(e) as boolean
            ))
    )
}

export function isTransactionData(obj: any, _argumentName?: string): obj is TransactionData {
    return (
        (obj !== null &&
            typeof obj === "object" ||
            typeof obj === "function") &&
        (typeof obj.sender === "undefined" ||
            isTransactionDigest(obj.sender) as boolean) &&
        isHaneulMoveTypeParameterIndex(obj.gasBudget) as boolean &&
        isHaneulMoveTypeParameterIndex(obj.gasPrice) as boolean &&
        isTransactionKind(obj.kind) as boolean &&
        isHaneulObjectRef(obj.gasPayment) as boolean
    )
}
