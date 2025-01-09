# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [haneul.node.v2.proto](#haneul-node-v2-proto)
    - [BalanceChange](#haneul-node-v2-BalanceChange)
    - [BalanceChanges](#haneul-node-v2-BalanceChanges)
    - [EffectsFinality](#haneul-node-v2-EffectsFinality)
    - [ExecuteTransactionOptions](#haneul-node-v2-ExecuteTransactionOptions)
    - [ExecuteTransactionRequest](#haneul-node-v2-ExecuteTransactionRequest)
    - [ExecuteTransactionResponse](#haneul-node-v2-ExecuteTransactionResponse)
    - [FullCheckpointObject](#haneul-node-v2-FullCheckpointObject)
    - [FullCheckpointObjects](#haneul-node-v2-FullCheckpointObjects)
    - [FullCheckpointTransaction](#haneul-node-v2-FullCheckpointTransaction)
    - [GetCheckpointOptions](#haneul-node-v2-GetCheckpointOptions)
    - [GetCheckpointRequest](#haneul-node-v2-GetCheckpointRequest)
    - [GetCheckpointResponse](#haneul-node-v2-GetCheckpointResponse)
    - [GetCommitteeRequest](#haneul-node-v2-GetCommitteeRequest)
    - [GetCommitteeResponse](#haneul-node-v2-GetCommitteeResponse)
    - [GetFullCheckpointOptions](#haneul-node-v2-GetFullCheckpointOptions)
    - [GetFullCheckpointRequest](#haneul-node-v2-GetFullCheckpointRequest)
    - [GetFullCheckpointResponse](#haneul-node-v2-GetFullCheckpointResponse)
    - [GetNodeInfoRequest](#haneul-node-v2-GetNodeInfoRequest)
    - [GetNodeInfoResponse](#haneul-node-v2-GetNodeInfoResponse)
    - [GetObjectOptions](#haneul-node-v2-GetObjectOptions)
    - [GetObjectRequest](#haneul-node-v2-GetObjectRequest)
    - [GetObjectResponse](#haneul-node-v2-GetObjectResponse)
    - [GetTransactionOptions](#haneul-node-v2-GetTransactionOptions)
    - [GetTransactionRequest](#haneul-node-v2-GetTransactionRequest)
    - [GetTransactionResponse](#haneul-node-v2-GetTransactionResponse)
    - [UserSignatures](#haneul-node-v2-UserSignatures)
    - [UserSignaturesBytes](#haneul-node-v2-UserSignaturesBytes)
  
    - [NodeService](#haneul-node-v2-NodeService)
  
- [haneul.types.proto](#haneul-types-proto)
    - [ActiveJwk](#haneul-types-ActiveJwk)
    - [Address](#haneul-types-Address)
    - [AddressDeniedForCoinError](#haneul-types-AddressDeniedForCoinError)
    - [Argument](#haneul-types-Argument)
    - [AuthenticatorStateExpire](#haneul-types-AuthenticatorStateExpire)
    - [AuthenticatorStateUpdate](#haneul-types-AuthenticatorStateUpdate)
    - [Bcs](#haneul-types-Bcs)
    - [Bn254FieldElement](#haneul-types-Bn254FieldElement)
    - [CancelledTransaction](#haneul-types-CancelledTransaction)
    - [CancelledTransactions](#haneul-types-CancelledTransactions)
    - [ChangeEpoch](#haneul-types-ChangeEpoch)
    - [ChangedObject](#haneul-types-ChangedObject)
    - [CheckpointCommitment](#haneul-types-CheckpointCommitment)
    - [CheckpointContents](#haneul-types-CheckpointContents)
    - [CheckpointContents.V1](#haneul-types-CheckpointContents-V1)
    - [CheckpointSummary](#haneul-types-CheckpointSummary)
    - [CheckpointedTransactionInfo](#haneul-types-CheckpointedTransactionInfo)
    - [CircomG1](#haneul-types-CircomG1)
    - [CircomG2](#haneul-types-CircomG2)
    - [Command](#haneul-types-Command)
    - [CommandArgumentError](#haneul-types-CommandArgumentError)
    - [CongestedObjectsError](#haneul-types-CongestedObjectsError)
    - [ConsensusCommitPrologue](#haneul-types-ConsensusCommitPrologue)
    - [ConsensusDeterminedVersionAssignments](#haneul-types-ConsensusDeterminedVersionAssignments)
    - [Digest](#haneul-types-Digest)
    - [EndOfEpochData](#haneul-types-EndOfEpochData)
    - [EndOfEpochTransaction](#haneul-types-EndOfEpochTransaction)
    - [EndOfEpochTransactionKind](#haneul-types-EndOfEpochTransactionKind)
    - [Event](#haneul-types-Event)
    - [ExecutionStatus](#haneul-types-ExecutionStatus)
    - [FailureStatus](#haneul-types-FailureStatus)
    - [GasCostSummary](#haneul-types-GasCostSummary)
    - [GasPayment](#haneul-types-GasPayment)
    - [GenesisObject](#haneul-types-GenesisObject)
    - [GenesisTransaction](#haneul-types-GenesisTransaction)
    - [I128](#haneul-types-I128)
    - [Identifier](#haneul-types-Identifier)
    - [Input](#haneul-types-Input)
    - [Jwk](#haneul-types-Jwk)
    - [JwkId](#haneul-types-JwkId)
    - [MakeMoveVector](#haneul-types-MakeMoveVector)
    - [MergeCoins](#haneul-types-MergeCoins)
    - [ModifiedAtVersion](#haneul-types-ModifiedAtVersion)
    - [MoveCall](#haneul-types-MoveCall)
    - [MoveError](#haneul-types-MoveError)
    - [MoveField](#haneul-types-MoveField)
    - [MoveLocation](#haneul-types-MoveLocation)
    - [MoveModule](#haneul-types-MoveModule)
    - [MovePackage](#haneul-types-MovePackage)
    - [MoveStruct](#haneul-types-MoveStruct)
    - [MoveStructValue](#haneul-types-MoveStructValue)
    - [MoveValue](#haneul-types-MoveValue)
    - [MoveVariant](#haneul-types-MoveVariant)
    - [MoveVector](#haneul-types-MoveVector)
    - [MultisigAggregatedSignature](#haneul-types-MultisigAggregatedSignature)
    - [MultisigCommittee](#haneul-types-MultisigCommittee)
    - [MultisigMember](#haneul-types-MultisigMember)
    - [MultisigMemberPublicKey](#haneul-types-MultisigMemberPublicKey)
    - [MultisigMemberSignature](#haneul-types-MultisigMemberSignature)
    - [NestedResult](#haneul-types-NestedResult)
    - [Object](#haneul-types-Object)
    - [ObjectData](#haneul-types-ObjectData)
    - [ObjectExist](#haneul-types-ObjectExist)
    - [ObjectId](#haneul-types-ObjectId)
    - [ObjectReference](#haneul-types-ObjectReference)
    - [ObjectReferenceWithOwner](#haneul-types-ObjectReferenceWithOwner)
    - [ObjectWrite](#haneul-types-ObjectWrite)
    - [Owner](#haneul-types-Owner)
    - [PackageIdDoesNotMatch](#haneul-types-PackageIdDoesNotMatch)
    - [PackageUpgradeError](#haneul-types-PackageUpgradeError)
    - [PackageWrite](#haneul-types-PackageWrite)
    - [PasskeyAuthenticator](#haneul-types-PasskeyAuthenticator)
    - [ProgrammableTransaction](#haneul-types-ProgrammableTransaction)
    - [Publish](#haneul-types-Publish)
    - [RandomnessStateUpdate](#haneul-types-RandomnessStateUpdate)
    - [ReadOnlyRoot](#haneul-types-ReadOnlyRoot)
    - [RoaringBitmap](#haneul-types-RoaringBitmap)
    - [SharedObjectInput](#haneul-types-SharedObjectInput)
    - [SimpleSignature](#haneul-types-SimpleSignature)
    - [SizeError](#haneul-types-SizeError)
    - [SplitCoins](#haneul-types-SplitCoins)
    - [StructTag](#haneul-types-StructTag)
    - [SystemPackage](#haneul-types-SystemPackage)
    - [Transaction](#haneul-types-Transaction)
    - [Transaction.TransactionV1](#haneul-types-Transaction-TransactionV1)
    - [TransactionEffects](#haneul-types-TransactionEffects)
    - [TransactionEffectsV1](#haneul-types-TransactionEffectsV1)
    - [TransactionEffectsV2](#haneul-types-TransactionEffectsV2)
    - [TransactionEvents](#haneul-types-TransactionEvents)
    - [TransactionExpiration](#haneul-types-TransactionExpiration)
    - [TransactionKind](#haneul-types-TransactionKind)
    - [TransferObjects](#haneul-types-TransferObjects)
    - [TypeArgumentError](#haneul-types-TypeArgumentError)
    - [TypeOrigin](#haneul-types-TypeOrigin)
    - [TypeTag](#haneul-types-TypeTag)
    - [U128](#haneul-types-U128)
    - [U256](#haneul-types-U256)
    - [UnchangedSharedObject](#haneul-types-UnchangedSharedObject)
    - [Upgrade](#haneul-types-Upgrade)
    - [UpgradeInfo](#haneul-types-UpgradeInfo)
    - [UserSignature](#haneul-types-UserSignature)
    - [ValidatorAggregatedSignature](#haneul-types-ValidatorAggregatedSignature)
    - [ValidatorCommittee](#haneul-types-ValidatorCommittee)
    - [ValidatorCommitteeMember](#haneul-types-ValidatorCommitteeMember)
    - [VersionAssignment](#haneul-types-VersionAssignment)
    - [ZkLoginAuthenticator](#haneul-types-ZkLoginAuthenticator)
    - [ZkLoginClaim](#haneul-types-ZkLoginClaim)
    - [ZkLoginInputs](#haneul-types-ZkLoginInputs)
    - [ZkLoginProof](#haneul-types-ZkLoginProof)
    - [ZkLoginPublicIdentifier](#haneul-types-ZkLoginPublicIdentifier)
  
    - [SignatureScheme](#haneul-types-SignatureScheme)
  
- [google/protobuf/empty.proto](#google_protobuf_empty-proto)
    - [Empty](#google-protobuf-Empty)
  
- [google/protobuf/timestamp.proto](#google_protobuf_timestamp-proto)
    - [Timestamp](#google-protobuf-Timestamp)
  
- [Scalar Value Types](#scalar-value-types)



<a name="haneul-node-v2-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## haneul.node.v2.proto



<a name="haneul-node-v2-BalanceChange"></a>

### BalanceChange



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [haneul.types.Address](#haneul-types-Address) | optional |  |
| coin_type | [haneul.types.TypeTag](#haneul-types-TypeTag) | optional |  |
| amount | [haneul.types.I128](#haneul-types-I128) | optional |  |






<a name="haneul-node-v2-BalanceChanges"></a>

### BalanceChanges



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| balance_changes | [BalanceChange](#haneul-node-v2-BalanceChange) | repeated |  |






<a name="haneul-node-v2-EffectsFinality"></a>

### EffectsFinality



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| certified | [haneul.types.ValidatorAggregatedSignature](#haneul-types-ValidatorAggregatedSignature) |  |  |
| checkpointed | [uint64](#uint64) |  |  |
| quorum_executed | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="haneul-node-v2-ExecuteTransactionOptions"></a>

### ExecuteTransactionOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| effects | [bool](#bool) | optional | Include the haneul.types.TransactionEffects message in the response.

Defaults to `false` if not included |
| effects_bcs | [bool](#bool) | optional | Include the TransactionEffects formatted as BCS in the response.

Defaults to `false` if not included |
| events | [bool](#bool) | optional | Include the haneul.types.TransactionEvents message in the response.

Defaults to `false` if not included |
| events_bcs | [bool](#bool) | optional | Include the TransactionEvents formatted as BCS in the response.

Defaults to `false` if not included |
| balance_changes | [bool](#bool) | optional | Include the BalanceChanges in the response.

Defaults to `false` if not included |






<a name="haneul-node-v2-ExecuteTransactionRequest"></a>

### ExecuteTransactionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [haneul.types.Transaction](#haneul-types-Transaction) | optional |  |
| transaction_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| signatures | [UserSignatures](#haneul-node-v2-UserSignatures) | optional |  |
| signatures_bytes | [UserSignaturesBytes](#haneul-node-v2-UserSignaturesBytes) | optional |  |
| options | [ExecuteTransactionOptions](#haneul-node-v2-ExecuteTransactionOptions) | optional |  |






<a name="haneul-node-v2-ExecuteTransactionResponse"></a>

### ExecuteTransactionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| finality | [EffectsFinality](#haneul-node-v2-EffectsFinality) | optional |  |
| effects | [haneul.types.TransactionEffects](#haneul-types-TransactionEffects) | optional |  |
| effects_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| events | [haneul.types.TransactionEvents](#haneul-types-TransactionEvents) | optional |  |
| events_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| balance_changes | [BalanceChanges](#haneul-node-v2-BalanceChanges) | optional |  |






<a name="haneul-node-v2-FullCheckpointObject"></a>

### FullCheckpointObject



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [haneul.types.ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this object |
| object | [haneul.types.Object](#haneul-types-Object) | optional |  |
| object_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |






<a name="haneul-node-v2-FullCheckpointObjects"></a>

### FullCheckpointObjects



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [FullCheckpointObject](#haneul-node-v2-FullCheckpointObject) | repeated |  |






<a name="haneul-node-v2-FullCheckpointTransaction"></a>

### FullCheckpointTransaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this transaction |
| transaction | [haneul.types.Transaction](#haneul-types-Transaction) | optional |  |
| transaction_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| effects | [haneul.types.TransactionEffects](#haneul-types-TransactionEffects) | optional |  |
| effects_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| events | [haneul.types.TransactionEvents](#haneul-types-TransactionEvents) | optional |  |
| events_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| input_objects | [FullCheckpointObjects](#haneul-node-v2-FullCheckpointObjects) | optional |  |
| output_objects | [FullCheckpointObjects](#haneul-node-v2-FullCheckpointObjects) | optional |  |






<a name="haneul-node-v2-GetCheckpointOptions"></a>

### GetCheckpointOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| summary | [bool](#bool) | optional | Include the haneul.types.CheckpointSummary in the response.

Defaults to `false` if not included |
| summary_bcs | [bool](#bool) | optional | Include the CheckpointSummary formatted as BCS in the response.

Defaults to `false` if not included |
| signature | [bool](#bool) | optional | Include the haneul.types.ValidatorAggregatedSignature in the response.

Defaults to `false` if not included |
| contents | [bool](#bool) | optional | Include the haneul.types.CheckpointContents message in the response.

Defaults to `false` if not included |
| contents_bcs | [bool](#bool) | optional | Include the CheckpointContents formatted as BCS in the response.

Defaults to `false` if not included |






<a name="haneul-node-v2-GetCheckpointRequest"></a>

### GetCheckpointRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional |  |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional |  |
| options | [GetCheckpointOptions](#haneul-node-v2-GetCheckpointOptions) | optional |  |






<a name="haneul-node-v2-GetCheckpointResponse"></a>

### GetCheckpointResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | The sequence number of this Checkpoint |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this Checkpoint&#39;s CheckpointSummary |
| summary | [haneul.types.CheckpointSummary](#haneul-types-CheckpointSummary) | optional |  |
| summary_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| signature | [haneul.types.ValidatorAggregatedSignature](#haneul-types-ValidatorAggregatedSignature) | optional |  |
| contents | [haneul.types.CheckpointContents](#haneul-types-CheckpointContents) | optional |  |
| contents_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |






<a name="haneul-node-v2-GetCommitteeRequest"></a>

### GetCommitteeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |






<a name="haneul-node-v2-GetCommitteeResponse"></a>

### GetCommitteeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| committee | [haneul.types.ValidatorCommittee](#haneul-types-ValidatorCommittee) | optional |  |






<a name="haneul-node-v2-GetFullCheckpointOptions"></a>

### GetFullCheckpointOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| summary | [bool](#bool) | optional | Include the haneul.types.CheckpointSummary in the response.

Defaults to `false` if not included |
| summary_bcs | [bool](#bool) | optional | Include the CheckpointSummary formatted as BCS in the response.

Defaults to `false` if not included |
| signature | [bool](#bool) | optional | Include the haneul.types.ValidatorAggregatedSignature in the response.

Defaults to `false` if not included |
| contents | [bool](#bool) | optional | Include the haneul.types.CheckpointContents message in the response.

Defaults to `false` if not included |
| contents_bcs | [bool](#bool) | optional | Include the CheckpointContents formatted as BCS in the response.

Defaults to `false` if not included |
| transaction | [bool](#bool) | optional | Include the haneul.types.Transaction message in the response.

Defaults to `false` if not included |
| transaction_bcs | [bool](#bool) | optional | Include the Transaction formatted as BCS in the response.

Defaults to `false` if not included |
| effects | [bool](#bool) | optional | Include the haneul.types.TransactionEffects message in the response.

Defaults to `false` if not included |
| effects_bcs | [bool](#bool) | optional | Include the TransactionEffects formatted as BCS in the response.

Defaults to `false` if not included |
| events | [bool](#bool) | optional | Include the haneul.types.TransactionEvents message in the response.

Defaults to `false` if not included |
| events_bcs | [bool](#bool) | optional | Include the TransactionEvents formatted as BCS in the response.

Defaults to `false` if not included |
| input_objects | [bool](#bool) | optional | Include the input objects for transactions in the response.

Defaults to `false` if not included |
| output_objects | [bool](#bool) | optional | Include the output objects for transactions in the response.

Defaults to `false` if not included |
| object | [bool](#bool) | optional | Include the haneul.types.Object message in the response.

Defaults to `false` if not included |
| object_bcs | [bool](#bool) | optional | Include the Object formatted as BCS in the response.

Defaults to `false` if not included |






<a name="haneul-node-v2-GetFullCheckpointRequest"></a>

### GetFullCheckpointRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional |  |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional |  |
| options | [GetFullCheckpointOptions](#haneul-node-v2-GetFullCheckpointOptions) | optional |  |






<a name="haneul-node-v2-GetFullCheckpointResponse"></a>

### GetFullCheckpointResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | The sequence number of this Checkpoint |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this Checkpoint&#39;s CheckpointSummary |
| summary | [haneul.types.CheckpointSummary](#haneul-types-CheckpointSummary) | optional |  |
| summary_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| signature | [haneul.types.ValidatorAggregatedSignature](#haneul-types-ValidatorAggregatedSignature) | optional |  |
| contents | [haneul.types.CheckpointContents](#haneul-types-CheckpointContents) | optional |  |
| contents_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| transactions | [FullCheckpointTransaction](#haneul-node-v2-FullCheckpointTransaction) | repeated |  |






<a name="haneul-node-v2-GetNodeInfoRequest"></a>

### GetNodeInfoRequest







<a name="haneul-node-v2-GetNodeInfoResponse"></a>

### GetNodeInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| chain_id | [haneul.types.Digest](#haneul-types-Digest) | optional | The chain identifier of the chain that this Node is on |
| chain | [string](#string) | optional | Human readable name of the chain that this Node is on |
| epoch | [uint64](#uint64) | optional | Current epoch of the Node based on its highest executed checkpoint |
| checkpoint_height | [uint64](#uint64) | optional | Checkpoint height of the most recently executed checkpoint |
| timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | Unix timestamp of the most recently executed checkpoint |
| lowest_available_checkpoint | [uint64](#uint64) | optional | The lowest checkpoint for which checkpoints and transaction data is available |
| lowest_available_checkpoint_objects | [uint64](#uint64) | optional | The lowest checkpoint for which object data is available |
| software_version | [string](#string) | optional |  |






<a name="haneul-node-v2-GetObjectOptions"></a>

### GetObjectOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object | [bool](#bool) | optional | Include the haneul.types.Object message in the response.

Defaults to `false` if not included |
| object_bcs | [bool](#bool) | optional | Include the Object formatted as BCS in the response.

Defaults to `false` if not included |






<a name="haneul-node-v2-GetObjectRequest"></a>

### GetObjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [haneul.types.ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| options | [GetObjectOptions](#haneul-node-v2-GetObjectOptions) | optional |  |






<a name="haneul-node-v2-GetObjectResponse"></a>

### GetObjectResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [haneul.types.ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this object |
| object | [haneul.types.Object](#haneul-types-Object) | optional |  |
| object_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |






<a name="haneul-node-v2-GetTransactionOptions"></a>

### GetTransactionOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [bool](#bool) | optional | Include the haneul.types.Transaction message in the response.

Defaults to `false` if not included |
| transaction_bcs | [bool](#bool) | optional | Include the Transaction formatted as BCS in the response.

Defaults to `false` if not included |
| signatures | [bool](#bool) | optional | Include the set of haneul.types.UserSignature&#39;s in the response.

Defaults to `false` if not included |
| signatures_bytes | [bool](#bool) | optional | Include the set of UserSignature&#39;s encoded as bytes in the response.

Defaults to `false` if not included |
| effects | [bool](#bool) | optional | Include the haneul.types.TransactionEffects message in the response.

Defaults to `false` if not included |
| effects_bcs | [bool](#bool) | optional | Include the TransactionEffects formatted as BCS in the response.

Defaults to `false` if not included |
| events | [bool](#bool) | optional | Include the haneul.types.TransactionEvents message in the response.

Defaults to `false` if not included |
| events_bcs | [bool](#bool) | optional | Include the TransactionEvents formatted as BCS in the response.

Defaults to `false` if not included |






<a name="haneul-node-v2-GetTransactionRequest"></a>

### GetTransactionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional |  |
| options | [GetTransactionOptions](#haneul-node-v2-GetTransactionOptions) | optional |  |






<a name="haneul-node-v2-GetTransactionResponse"></a>

### GetTransactionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [haneul.types.Digest](#haneul-types-Digest) | optional | The digest of this transaction |
| transaction | [haneul.types.Transaction](#haneul-types-Transaction) | optional |  |
| transaction_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| signatures | [UserSignatures](#haneul-node-v2-UserSignatures) | optional |  |
| signatures_bytes | [UserSignaturesBytes](#haneul-node-v2-UserSignaturesBytes) | optional |  |
| effects | [haneul.types.TransactionEffects](#haneul-types-TransactionEffects) | optional |  |
| effects_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| events | [haneul.types.TransactionEvents](#haneul-types-TransactionEvents) | optional |  |
| events_bcs | [haneul.types.Bcs](#haneul-types-Bcs) | optional |  |
| checkpoint | [uint64](#uint64) | optional |  |
| timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional |  |






<a name="haneul-node-v2-UserSignatures"></a>

### UserSignatures



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [haneul.types.UserSignature](#haneul-types-UserSignature) | repeated |  |






<a name="haneul-node-v2-UserSignaturesBytes"></a>

### UserSignaturesBytes



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [bytes](#bytes) | repeated |  |





 

 

 


<a name="haneul-node-v2-NodeService"></a>

### NodeService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetNodeInfo | [GetNodeInfoRequest](#haneul-node-v2-GetNodeInfoRequest) | [GetNodeInfoResponse](#haneul-node-v2-GetNodeInfoResponse) |  |
| GetCommittee | [GetCommitteeRequest](#haneul-node-v2-GetCommitteeRequest) | [GetCommitteeResponse](#haneul-node-v2-GetCommitteeResponse) |  |
| GetObject | [GetObjectRequest](#haneul-node-v2-GetObjectRequest) | [GetObjectResponse](#haneul-node-v2-GetObjectResponse) |  |
| GetTransaction | [GetTransactionRequest](#haneul-node-v2-GetTransactionRequest) | [GetTransactionResponse](#haneul-node-v2-GetTransactionResponse) |  |
| GetCheckpoint | [GetCheckpointRequest](#haneul-node-v2-GetCheckpointRequest) | [GetCheckpointResponse](#haneul-node-v2-GetCheckpointResponse) |  |
| GetFullCheckpoint | [GetFullCheckpointRequest](#haneul-node-v2-GetFullCheckpointRequest) | [GetFullCheckpointResponse](#haneul-node-v2-GetFullCheckpointResponse) |  |
| ExecuteTransaction | [ExecuteTransactionRequest](#haneul-node-v2-ExecuteTransactionRequest) | [ExecuteTransactionResponse](#haneul-node-v2-ExecuteTransactionResponse) |  |

 



<a name="haneul-types-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## haneul.types.proto



<a name="haneul-types-ActiveJwk"></a>

### ActiveJwk



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [JwkId](#haneul-types-JwkId) | optional |  |
| jwk | [Jwk](#haneul-types-Jwk) | optional |  |
| epoch | [uint64](#uint64) | optional |  |






<a name="haneul-types-Address"></a>

### Address



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) | optional |  |






<a name="haneul-types-AddressDeniedForCoinError"></a>

### AddressDeniedForCoinError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#haneul-types-Address) | optional |  |
| coin_type | [string](#string) | optional |  |






<a name="haneul-types-Argument"></a>

### Argument



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gas | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| input | [uint32](#uint32) |  |  |
| result | [uint32](#uint32) |  |  |
| nested_result | [NestedResult](#haneul-types-NestedResult) |  |  |






<a name="haneul-types-AuthenticatorStateExpire"></a>

### AuthenticatorStateExpire



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_epoch | [uint64](#uint64) | optional |  |
| authenticator_object_initial_shared_version | [uint64](#uint64) | optional |  |






<a name="haneul-types-AuthenticatorStateUpdate"></a>

### AuthenticatorStateUpdate



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| round | [uint64](#uint64) | optional |  |
| new_active_jwks | [ActiveJwk](#haneul-types-ActiveJwk) | repeated |  |
| authenticator_object_initial_shared_version | [uint64](#uint64) | optional |  |






<a name="haneul-types-Bcs"></a>

### Bcs



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bcs | [bytes](#bytes) | optional |  |






<a name="haneul-types-Bn254FieldElement"></a>

### Bn254FieldElement



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| element | [bytes](#bytes) | optional |  |






<a name="haneul-types-CancelledTransaction"></a>

### CancelledTransaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [Digest](#haneul-types-Digest) | optional |  |
| version_assignments | [VersionAssignment](#haneul-types-VersionAssignment) | repeated |  |






<a name="haneul-types-CancelledTransactions"></a>

### CancelledTransactions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cancelled_transactions | [CancelledTransaction](#haneul-types-CancelledTransaction) | repeated |  |






<a name="haneul-types-ChangeEpoch"></a>

### ChangeEpoch



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | The next (to become) epoch ID. |
| protocol_version | [uint64](#uint64) | optional | The protocol version in effect in the new epoch. |
| storage_charge | [uint64](#uint64) | optional | The total amount of gas charged for storage during the epoch. |
| computation_charge | [uint64](#uint64) | optional | The total amount of gas charged for computation during the epoch. |
| storage_rebate | [uint64](#uint64) | optional | The amount of storage rebate refunded to the txn senders. |
| non_refundable_storage_fee | [uint64](#uint64) | optional | The non-refundable storage fee. |
| epoch_start_timestamp_ms | [uint64](#uint64) | optional | Unix timestamp when epoch started |
| system_packages | [SystemPackage](#haneul-types-SystemPackage) | repeated | System packages (specifically framework and move stdlib) that are written before the new epoch starts. This tracks framework upgrades on chain. When executing the ChangeEpoch txn, the validator must write out the modules below. Modules are provided with the version they will be upgraded to, their modules in serialized form (which include their package ID), and a list of their transitive dependencies. |






<a name="haneul-types-ChangedObject"></a>

### ChangedObject



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| not_exist | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| exist | [ObjectExist](#haneul-types-ObjectExist) |  |  |
| removed | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| object_write | [ObjectWrite](#haneul-types-ObjectWrite) |  |  |
| package_write | [PackageWrite](#haneul-types-PackageWrite) |  |  |
| none | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| created | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| deleted | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="haneul-types-CheckpointCommitment"></a>

### CheckpointCommitment



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ecmh_live_object_set | [Digest](#haneul-types-Digest) |  |  |






<a name="haneul-types-CheckpointContents"></a>

### CheckpointContents



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [CheckpointContents.V1](#haneul-types-CheckpointContents-V1) |  |  |






<a name="haneul-types-CheckpointContents-V1"></a>

### CheckpointContents.V1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [CheckpointedTransactionInfo](#haneul-types-CheckpointedTransactionInfo) | repeated |  |






<a name="haneul-types-CheckpointSummary"></a>

### CheckpointSummary



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| sequence_number | [uint64](#uint64) | optional |  |
| total_network_transactions | [uint64](#uint64) | optional |  |
| content_digest | [Digest](#haneul-types-Digest) | optional |  |
| previous_digest | [Digest](#haneul-types-Digest) | optional |  |
| epoch_rolling_gas_cost_summary | [GasCostSummary](#haneul-types-GasCostSummary) | optional |  |
| timestamp_ms | [uint64](#uint64) | optional |  |
| commitments | [CheckpointCommitment](#haneul-types-CheckpointCommitment) | repeated |  |
| end_of_epoch_data | [EndOfEpochData](#haneul-types-EndOfEpochData) | optional |  |
| version_specific_data | [bytes](#bytes) | optional |  |






<a name="haneul-types-CheckpointedTransactionInfo"></a>

### CheckpointedTransactionInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [Digest](#haneul-types-Digest) | optional | TransactionDigest |
| effects | [Digest](#haneul-types-Digest) | optional | EffectsDigest |
| signatures | [UserSignature](#haneul-types-UserSignature) | repeated |  |






<a name="haneul-types-CircomG1"></a>

### CircomG1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| e0 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e1 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e2 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |






<a name="haneul-types-CircomG2"></a>

### CircomG2



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| e00 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e01 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e10 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e11 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e20 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |
| e21 | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |






<a name="haneul-types-Command"></a>

### Command



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| move_call | [MoveCall](#haneul-types-MoveCall) |  |  |
| transfer_objects | [TransferObjects](#haneul-types-TransferObjects) |  |  |
| split_coins | [SplitCoins](#haneul-types-SplitCoins) |  |  |
| merge_coins | [MergeCoins](#haneul-types-MergeCoins) |  |  |
| publish | [Publish](#haneul-types-Publish) |  |  |
| make_move_vector | [MakeMoveVector](#haneul-types-MakeMoveVector) |  |  |
| upgrade | [Upgrade](#haneul-types-Upgrade) |  |  |






<a name="haneul-types-CommandArgumentError"></a>

### CommandArgumentError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| argument | [uint32](#uint32) | optional |  |
| type_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | The type of the value does not match the expected type |
| invalid_bcs_bytes | [google.protobuf.Empty](#google-protobuf-Empty) |  | The argument cannot be deserialized into a value of the specified type |
| invalid_usage_of_pure_argument | [google.protobuf.Empty](#google-protobuf-Empty) |  | The argument cannot be instantiated from raw bytes |
| invalid_argument_to_private_entry_function | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid argument to private entry function. / Private entry functions cannot take arguments from other Move functions. |
| index_out_of_bounds | [uint32](#uint32) |  | Out of bounds access to input or results |
| secondary_index_out_of_bounds | [NestedResult](#haneul-types-NestedResult) |  | Out of bounds access to subresult |
| invalid_result_arity | [uint32](#uint32) |  | Invalid usage of result. / Expected a single result but found either no return value or multiple. |
| invalid_gas_coin_usage | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid usage of Gas coin. / The Gas coin can only be used by-value with a TransferObjects command. |
| invalid_value_usage | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid usage of move value. Mutably borrowed values require unique usage. Immutably borrowed values cannot be taken or borrowed mutably. Taken values cannot be used again. |
| invalid_object_by_value | [google.protobuf.Empty](#google-protobuf-Empty) |  | Immutable objects cannot be passed by-value. |
| invalid_object_by_mut_ref | [google.protobuf.Empty](#google-protobuf-Empty) |  | Immutable objects cannot be passed by mutable reference, &amp;mut. |
| shared_object_operation_not_allowed | [google.protobuf.Empty](#google-protobuf-Empty) |  | Shared object operations such a wrapping, freezing, or converting to owned are not / allowed. |






<a name="haneul-types-CongestedObjectsError"></a>

### CongestedObjectsError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| congested_objects | [ObjectId](#haneul-types-ObjectId) | repeated |  |






<a name="haneul-types-ConsensusCommitPrologue"></a>

### ConsensusCommitPrologue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| round | [uint64](#uint64) | optional |  |
| commit_timestamp_ms | [uint64](#uint64) | optional |  |
| consensus_commit_digest | [Digest](#haneul-types-Digest) | optional |  |
| sub_dag_index | [uint64](#uint64) | optional |  |
| consensus_determined_version_assignments | [ConsensusDeterminedVersionAssignments](#haneul-types-ConsensusDeterminedVersionAssignments) | optional |  |






<a name="haneul-types-ConsensusDeterminedVersionAssignments"></a>

### ConsensusDeterminedVersionAssignments



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cancelled_transactions | [CancelledTransactions](#haneul-types-CancelledTransactions) |  |  |






<a name="haneul-types-Digest"></a>

### Digest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [bytes](#bytes) | optional |  |






<a name="haneul-types-EndOfEpochData"></a>

### EndOfEpochData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_epoch_committee | [ValidatorCommitteeMember](#haneul-types-ValidatorCommitteeMember) | repeated |  |
| next_epoch_protocol_version | [uint64](#uint64) | optional |  |
| epoch_commitments | [CheckpointCommitment](#haneul-types-CheckpointCommitment) | repeated |  |






<a name="haneul-types-EndOfEpochTransaction"></a>

### EndOfEpochTransaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [EndOfEpochTransactionKind](#haneul-types-EndOfEpochTransactionKind) | repeated |  |






<a name="haneul-types-EndOfEpochTransactionKind"></a>

### EndOfEpochTransactionKind



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| change_epoch | [ChangeEpoch](#haneul-types-ChangeEpoch) |  |  |
| authenticator_state_expire | [AuthenticatorStateExpire](#haneul-types-AuthenticatorStateExpire) |  |  |
| authenticator_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  | Use higher field numbers for kinds which happen infrequently |
| randomness_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| deny_list_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| bridge_state_create | [Digest](#haneul-types-Digest) |  |  |
| bridge_committee_init | [uint64](#uint64) |  |  |






<a name="haneul-types-Event"></a>

### Event



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| module | [Identifier](#haneul-types-Identifier) | optional |  |
| sender | [Address](#haneul-types-Address) | optional |  |
| event_type | [StructTag](#haneul-types-StructTag) | optional |  |
| contents | [bytes](#bytes) | optional |  |






<a name="haneul-types-ExecutionStatus"></a>

### ExecutionStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| success | [bool](#bool) | optional |  |
| status | [FailureStatus](#haneul-types-FailureStatus) | optional |  |






<a name="haneul-types-FailureStatus"></a>

### FailureStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| command | [uint64](#uint64) | optional |  |
| insufficient_gas | [google.protobuf.Empty](#google-protobuf-Empty) |  | Insufficient Gas |
| invalid_gas_object | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid Gas Object. |
| invariant_violation | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invariant Violation |
| feature_not_yet_supported | [google.protobuf.Empty](#google-protobuf-Empty) |  | Attempted to used feature that is not supported yet |
| object_too_big | [SizeError](#haneul-types-SizeError) |  | Move object is larger than the maximum allowed size |
| package_too_big | [SizeError](#haneul-types-SizeError) |  | Package is larger than the maximum allowed size |
| circular_object_ownership | [ObjectId](#haneul-types-ObjectId) |  | Circular Object Ownership |
| insufficient_coin_balance | [google.protobuf.Empty](#google-protobuf-Empty) |  | Coin errors

/ Insufficient coin balance for requested operation |
| coin_balance_overflow | [google.protobuf.Empty](#google-protobuf-Empty) |  | Coin balance overflowed an u64 |
| publish_error_non_zero_address | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish/Upgrade errors

/ Publish Error, Non-zero Address. / The modules in the package must have their self-addresses set to zero. |
| haneul_move_verification_error | [google.protobuf.Empty](#google-protobuf-Empty) |  | Haneul Move Bytecode Verification Error. |
| move_primitive_runtime_error | [MoveError](#haneul-types-MoveError) |  | MoveVm Errors

/ Error from a non-abort instruction. / Possible causes: / Arithmetic error, stack overflow, max value depth, etc.&#34; |
| move_abort | [MoveError](#haneul-types-MoveError) |  | Move runtime abort |
| vm_verification_or_deserialization_error | [google.protobuf.Empty](#google-protobuf-Empty) |  | Bytecode verification error. |
| vm_invariant_violation | [google.protobuf.Empty](#google-protobuf-Empty) |  | MoveVm invariant violation |
| function_not_found | [google.protobuf.Empty](#google-protobuf-Empty) |  | Programmable Transaction Errors

/ Function not found |
| arity_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | Arity mismatch for Move function. / The number of arguments does not match the number of parameters |
| type_arity_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | Type arity mismatch for Move function. / Mismatch between the number of actual versus expected type arguments. |
| non_entry_function_invoked | [google.protobuf.Empty](#google-protobuf-Empty) |  | Non Entry Function Invoked. Move Call must start with an entry function. |
| command_argument_error | [CommandArgumentError](#haneul-types-CommandArgumentError) |  | Invalid command argument |
| type_argument_error | [TypeArgumentError](#haneul-types-TypeArgumentError) |  | Type argument error |
| unused_value_without_drop | [NestedResult](#haneul-types-NestedResult) |  | Unused result without the drop ability. |
| invalid_public_function_return_type | [uint32](#uint32) |  | Invalid public Move function signature. / Unsupported return type for return value |
| invalid_transfer_object | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid Transfer Object, object does not have public transfer. |
| effects_too_large | [SizeError](#haneul-types-SizeError) |  | Post-execution errors

/ Effects from the transaction are too large |
| publish_upgrade_missing_dependency | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish or Upgrade is missing dependency |
| publish_upgrade_dependency_downgrade | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish or Upgrade dependency downgrade. / / Indirect (transitive) dependency of published or upgraded package has been assigned an / on-chain version that is less than the version required by one of the package&#39;s / transitive dependencies. |
| package_upgrade_error | [PackageUpgradeError](#haneul-types-PackageUpgradeError) |  | Invalid package upgrade |
| written_objects_too_large | [SizeError](#haneul-types-SizeError) |  | Indicates the transaction tried to write objects too large to storage |
| certificate_denied | [google.protobuf.Empty](#google-protobuf-Empty) |  | Certificate is on the deny list |
| haneul_move_verification_timedout | [google.protobuf.Empty](#google-protobuf-Empty) |  | Haneul Move Bytecode verification timed out. |
| shared_object_operation_not_allowed | [google.protobuf.Empty](#google-protobuf-Empty) |  | The requested shared object operation is not allowed |
| input_object_deleted | [google.protobuf.Empty](#google-protobuf-Empty) |  | Requested shared object has been deleted |
| execution_cancelled_due_to_shared_object_congestion | [CongestedObjectsError](#haneul-types-CongestedObjectsError) |  | Certificate is cancelled due to congestion on shared objects |
| address_denied_for_coin | [AddressDeniedForCoinError](#haneul-types-AddressDeniedForCoinError) |  | Address is denied for this coin type |
| coin_type_global_pause | [string](#string) |  | Coin type is globally paused for use |
| execution_cancelled_due_to_randomness_unavailable | [google.protobuf.Empty](#google-protobuf-Empty) |  | Certificate is cancelled because randomness could not be generated this epoch |






<a name="haneul-types-GasCostSummary"></a>

### GasCostSummary



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| computation_cost | [uint64](#uint64) | optional |  |
| storage_cost | [uint64](#uint64) | optional |  |
| storage_rebate | [uint64](#uint64) | optional |  |
| non_refundable_storage_fee | [uint64](#uint64) | optional |  |






<a name="haneul-types-GasPayment"></a>

### GasPayment



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [ObjectReference](#haneul-types-ObjectReference) | repeated |  |
| owner | [Address](#haneul-types-Address) | optional |  |
| price | [uint64](#uint64) | optional |  |
| budget | [uint64](#uint64) | optional |  |






<a name="haneul-types-GenesisObject"></a>

### GenesisObject



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| owner | [Owner](#haneul-types-Owner) | optional |  |
| object | [ObjectData](#haneul-types-ObjectData) | optional |  |






<a name="haneul-types-GenesisTransaction"></a>

### GenesisTransaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [GenesisObject](#haneul-types-GenesisObject) | repeated |  |






<a name="haneul-types-I128"></a>

### I128
Little-endian encoded i128


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional |  |






<a name="haneul-types-Identifier"></a>

### Identifier



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| identifier | [string](#string) | optional |  |






<a name="haneul-types-Input"></a>

### Input



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pure | [bytes](#bytes) |  |  |
| immutable_or_owned | [ObjectReference](#haneul-types-ObjectReference) |  |  |
| shared | [SharedObjectInput](#haneul-types-SharedObjectInput) |  |  |
| receiving | [ObjectReference](#haneul-types-ObjectReference) |  |  |






<a name="haneul-types-Jwk"></a>

### Jwk



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kty | [string](#string) | optional | Key type parameter, &lt;https://datatracker.ietf.org/doc/html/rfc7517#section-4.1&gt; |
| e | [string](#string) | optional | RSA public exponent, &lt;https://datatracker.ietf.org/doc/html/rfc7517#section-9.3&gt; |
| n | [string](#string) | optional | RSA modulus, &lt;https://datatracker.ietf.org/doc/html/rfc7517#section-9.3&gt; |
| alg | [string](#string) | optional | Algorithm parameter, &lt;https://datatracker.ietf.org/doc/html/rfc7517#section-4.4&gt; |






<a name="haneul-types-JwkId"></a>

### JwkId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| iss | [string](#string) | optional |  |
| kid | [string](#string) | optional |  |






<a name="haneul-types-MakeMoveVector"></a>

### MakeMoveVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| element_type | [TypeTag](#haneul-types-TypeTag) | optional |  |
| elements | [Argument](#haneul-types-Argument) | repeated |  |






<a name="haneul-types-MergeCoins"></a>

### MergeCoins



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coin | [Argument](#haneul-types-Argument) | optional |  |
| coins_to_merge | [Argument](#haneul-types-Argument) | repeated |  |






<a name="haneul-types-ModifiedAtVersion"></a>

### ModifiedAtVersion



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |






<a name="haneul-types-MoveCall"></a>

### MoveCall



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package | [ObjectId](#haneul-types-ObjectId) | optional |  |
| module | [Identifier](#haneul-types-Identifier) | optional |  |
| function | [Identifier](#haneul-types-Identifier) | optional |  |
| type_arguments | [TypeTag](#haneul-types-TypeTag) | repeated |  |
| arguments | [Argument](#haneul-types-Argument) | repeated |  |






<a name="haneul-types-MoveError"></a>

### MoveError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| location | [MoveLocation](#haneul-types-MoveLocation) | optional |  |
| abort_code | [uint64](#uint64) | optional |  |






<a name="haneul-types-MoveField"></a>

### MoveField



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [Identifier](#haneul-types-Identifier) | optional |  |
| value | [MoveValue](#haneul-types-MoveValue) | optional |  |






<a name="haneul-types-MoveLocation"></a>

### MoveLocation



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package | [ObjectId](#haneul-types-ObjectId) | optional |  |
| module | [Identifier](#haneul-types-Identifier) | optional |  |
| function | [uint32](#uint32) | optional |  |
| instruction | [uint32](#uint32) | optional |  |
| function_name | [Identifier](#haneul-types-Identifier) | optional |  |






<a name="haneul-types-MoveModule"></a>

### MoveModule



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [Identifier](#haneul-types-Identifier) | optional |  |
| contents | [bytes](#bytes) | optional |  |






<a name="haneul-types-MovePackage"></a>

### MovePackage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| modules | [MoveModule](#haneul-types-MoveModule) | repeated |  |
| type_origin_table | [TypeOrigin](#haneul-types-TypeOrigin) | repeated |  |
| linkage_table | [UpgradeInfo](#haneul-types-UpgradeInfo) | repeated |  |






<a name="haneul-types-MoveStruct"></a>

### MoveStruct



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| object_type | [StructTag](#haneul-types-StructTag) | optional |  |
| has_public_transfer | [bool](#bool) | optional |  |
| version | [uint64](#uint64) | optional |  |
| contents | [bytes](#bytes) | optional |  |






<a name="haneul-types-MoveStructValue"></a>

### MoveStructValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| struct_type | [StructTag](#haneul-types-StructTag) | optional |  |
| fields | [MoveField](#haneul-types-MoveField) | repeated |  |






<a name="haneul-types-MoveValue"></a>

### MoveValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bool | [bool](#bool) |  |  |
| u8 | [uint32](#uint32) |  |  |
| u16 | [uint32](#uint32) |  |  |
| u32 | [uint32](#uint32) |  |  |
| u64 | [uint64](#uint64) |  |  |
| u128 | [U128](#haneul-types-U128) |  |  |
| u256 | [U256](#haneul-types-U256) |  |  |
| address | [Address](#haneul-types-Address) |  |  |
| vector | [MoveVector](#haneul-types-MoveVector) |  |  |
| struct | [MoveStructValue](#haneul-types-MoveStructValue) |  |  |
| signer | [Address](#haneul-types-Address) |  |  |
| variant | [MoveVariant](#haneul-types-MoveVariant) |  |  |






<a name="haneul-types-MoveVariant"></a>

### MoveVariant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enum_type | [StructTag](#haneul-types-StructTag) | optional |  |
| variant_name | [Identifier](#haneul-types-Identifier) | optional |  |
| tag | [uint32](#uint32) | optional |  |
| fields | [MoveField](#haneul-types-MoveField) | repeated |  |






<a name="haneul-types-MoveVector"></a>

### MoveVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [MoveValue](#haneul-types-MoveValue) | repeated |  |






<a name="haneul-types-MultisigAggregatedSignature"></a>

### MultisigAggregatedSignature



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [MultisigMemberSignature](#haneul-types-MultisigMemberSignature) | repeated |  |
| bitmap | [uint32](#uint32) | optional |  |
| legacy_bitmap | [RoaringBitmap](#haneul-types-RoaringBitmap) | optional |  |
| committee | [MultisigCommittee](#haneul-types-MultisigCommittee) | optional |  |






<a name="haneul-types-MultisigCommittee"></a>

### MultisigCommittee



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| members | [MultisigMember](#haneul-types-MultisigMember) | repeated |  |
| threshold | [uint32](#uint32) | optional |  |






<a name="haneul-types-MultisigMember"></a>

### MultisigMember



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [MultisigMemberPublicKey](#haneul-types-MultisigMemberPublicKey) | optional |  |
| weight | [uint32](#uint32) | optional |  |






<a name="haneul-types-MultisigMemberPublicKey"></a>

### MultisigMemberPublicKey



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ed25519 | [bytes](#bytes) |  |  |
| secp256k1 | [bytes](#bytes) |  |  |
| secp256r1 | [bytes](#bytes) |  |  |
| zklogin | [ZkLoginPublicIdentifier](#haneul-types-ZkLoginPublicIdentifier) |  |  |






<a name="haneul-types-MultisigMemberSignature"></a>

### MultisigMemberSignature



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ed25519 | [bytes](#bytes) |  |  |
| secp256k1 | [bytes](#bytes) |  |  |
| secp256r1 | [bytes](#bytes) |  |  |
| zklogin | [ZkLoginAuthenticator](#haneul-types-ZkLoginAuthenticator) |  |  |






<a name="haneul-types-NestedResult"></a>

### NestedResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [uint32](#uint32) | optional |  |
| subresult | [uint32](#uint32) | optional |  |






<a name="haneul-types-Object"></a>

### Object



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| owner | [Owner](#haneul-types-Owner) | optional |  |
| object | [ObjectData](#haneul-types-ObjectData) | optional |  |
| previous_transaction | [Digest](#haneul-types-Digest) | optional |  |
| storage_rebate | [uint64](#uint64) | optional |  |






<a name="haneul-types-ObjectData"></a>

### ObjectData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| struct | [MoveStruct](#haneul-types-MoveStruct) |  |  |
| package | [MovePackage](#haneul-types-MovePackage) |  |  |






<a name="haneul-types-ObjectExist"></a>

### ObjectExist



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional |  |
| digest | [Digest](#haneul-types-Digest) | optional |  |
| owner | [Owner](#haneul-types-Owner) | optional |  |






<a name="haneul-types-ObjectId"></a>

### ObjectId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [bytes](#bytes) | optional |  |






<a name="haneul-types-ObjectReference"></a>

### ObjectReference



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| digest | [Digest](#haneul-types-Digest) | optional |  |






<a name="haneul-types-ObjectReferenceWithOwner"></a>

### ObjectReferenceWithOwner



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reference | [ObjectReference](#haneul-types-ObjectReference) | optional |  |
| owner | [Owner](#haneul-types-Owner) | optional |  |






<a name="haneul-types-ObjectWrite"></a>

### ObjectWrite



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [Digest](#haneul-types-Digest) | optional |  |
| owner | [Owner](#haneul-types-Owner) | optional |  |






<a name="haneul-types-Owner"></a>

### Owner



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#haneul-types-Address) |  |  |
| object | [ObjectId](#haneul-types-ObjectId) |  |  |
| shared | [uint64](#uint64) |  |  |
| immutable | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="haneul-types-PackageIdDoesNotMatch"></a>

### PackageIdDoesNotMatch



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |






<a name="haneul-types-PackageUpgradeError"></a>

### PackageUpgradeError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unable_to_fetch_package | [ObjectId](#haneul-types-ObjectId) |  |  |
| not_a_package | [ObjectId](#haneul-types-ObjectId) |  |  |
| incompatible_upgrade | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| digets_does_not_match | [Digest](#haneul-types-Digest) |  |  |
| unknown_upgrade_policy | [uint32](#uint32) |  |  |
| package_id_does_not_match | [PackageIdDoesNotMatch](#haneul-types-PackageIdDoesNotMatch) |  |  |






<a name="haneul-types-PackageWrite"></a>

### PackageWrite



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional |  |
| digest | [Digest](#haneul-types-Digest) | optional |  |






<a name="haneul-types-PasskeyAuthenticator"></a>

### PasskeyAuthenticator



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authenticator_data | [bytes](#bytes) | optional |  |
| client_data_json | [string](#string) | optional |  |
| signature | [SimpleSignature](#haneul-types-SimpleSignature) | optional |  |






<a name="haneul-types-ProgrammableTransaction"></a>

### ProgrammableTransaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| inputs | [Input](#haneul-types-Input) | repeated |  |
| commands | [Command](#haneul-types-Command) | repeated |  |






<a name="haneul-types-Publish"></a>

### Publish



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| modules | [bytes](#bytes) | repeated |  |
| dependencies | [ObjectId](#haneul-types-ObjectId) | repeated |  |






<a name="haneul-types-RandomnessStateUpdate"></a>

### RandomnessStateUpdate



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| randomness_round | [uint64](#uint64) | optional |  |
| random_bytes | [bytes](#bytes) | optional |  |
| randomness_object_initial_shared_version | [uint64](#uint64) | optional |  |






<a name="haneul-types-ReadOnlyRoot"></a>

### ReadOnlyRoot



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional |  |
| digest | [Digest](#haneul-types-Digest) | optional |  |






<a name="haneul-types-RoaringBitmap"></a>

### RoaringBitmap



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bitmap | [bytes](#bytes) | optional |  |






<a name="haneul-types-SharedObjectInput"></a>

### SharedObjectInput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| initial_shared_version | [uint64](#uint64) | optional |  |
| mutable | [bool](#bool) | optional |  |






<a name="haneul-types-SimpleSignature"></a>

### SimpleSignature



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| scheme | [SignatureScheme](#haneul-types-SignatureScheme) | optional |  |
| signature | [bytes](#bytes) | optional |  |
| public_key | [bytes](#bytes) | optional |  |






<a name="haneul-types-SizeError"></a>

### SizeError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) | optional |  |
| max_size | [uint64](#uint64) | optional |  |






<a name="haneul-types-SplitCoins"></a>

### SplitCoins



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coin | [Argument](#haneul-types-Argument) | optional |  |
| amounts | [Argument](#haneul-types-Argument) | repeated |  |






<a name="haneul-types-StructTag"></a>

### StructTag



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#haneul-types-Address) | optional |  |
| module | [Identifier](#haneul-types-Identifier) | optional |  |
| name | [Identifier](#haneul-types-Identifier) | optional |  |
| type_parameters | [TypeTag](#haneul-types-TypeTag) | repeated |  |






<a name="haneul-types-SystemPackage"></a>

### SystemPackage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional |  |
| modules | [bytes](#bytes) | repeated |  |
| dependencies | [ObjectId](#haneul-types-ObjectId) | repeated |  |






<a name="haneul-types-Transaction"></a>

### Transaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [Transaction.TransactionV1](#haneul-types-Transaction-TransactionV1) |  |  |






<a name="haneul-types-Transaction-TransactionV1"></a>

### Transaction.TransactionV1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kind | [TransactionKind](#haneul-types-TransactionKind) | optional |  |
| sender | [Address](#haneul-types-Address) | optional |  |
| gas_payment | [GasPayment](#haneul-types-GasPayment) | optional |  |
| expiration | [TransactionExpiration](#haneul-types-TransactionExpiration) | optional |  |






<a name="haneul-types-TransactionEffects"></a>

### TransactionEffects



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [TransactionEffectsV1](#haneul-types-TransactionEffectsV1) |  |  |
| v2 | [TransactionEffectsV2](#haneul-types-TransactionEffectsV2) |  |  |






<a name="haneul-types-TransactionEffectsV1"></a>

### TransactionEffectsV1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [ExecutionStatus](#haneul-types-ExecutionStatus) | optional | The status of the execution |
| epoch | [uint64](#uint64) | optional | The epoch when this transaction was executed. |
| gas_used | [GasCostSummary](#haneul-types-GasCostSummary) | optional |  |
| modified_at_versions | [ModifiedAtVersion](#haneul-types-ModifiedAtVersion) | repeated | The version that every modified (mutated or deleted) object had before it was modified by / this transaction. |
| shared_objects | [ObjectReference](#haneul-types-ObjectReference) | repeated | The object references of the shared objects used in this transaction. Empty if no shared objects were used. |
| transaction_digest | [Digest](#haneul-types-Digest) | optional | The transaction digest |
| created | [ObjectReferenceWithOwner](#haneul-types-ObjectReferenceWithOwner) | repeated | ObjectReference and owner of new objects created. |
| mutated | [ObjectReferenceWithOwner](#haneul-types-ObjectReferenceWithOwner) | repeated | ObjectReference and owner of mutated objects, including gas object. |
| unwrapped | [ObjectReferenceWithOwner](#haneul-types-ObjectReferenceWithOwner) | repeated | ObjectReference and owner of objects that are unwrapped in this transaction. / Unwrapped objects are objects that were wrapped into other objects in the past, / and just got extracted out. |
| deleted | [ObjectReference](#haneul-types-ObjectReference) | repeated | Object Refs of objects now deleted (the new refs). |
| unwrapped_then_deleted | [ObjectReference](#haneul-types-ObjectReference) | repeated | Object refs of objects previously wrapped in other objects but now deleted. |
| wrapped | [ObjectReference](#haneul-types-ObjectReference) | repeated | Object refs of objects now wrapped in other objects. |
| gas_object | [ObjectReferenceWithOwner](#haneul-types-ObjectReferenceWithOwner) | optional | The updated gas object reference. Have a dedicated field for convenient access. / It&#39;s also included in mutated. |
| events_digest | [Digest](#haneul-types-Digest) | optional | The digest of the events emitted during execution, / can be None if the transaction does not emit any event. |
| dependencies | [Digest](#haneul-types-Digest) | repeated | The set of transaction digests this transaction depends on. |






<a name="haneul-types-TransactionEffectsV2"></a>

### TransactionEffectsV2



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [ExecutionStatus](#haneul-types-ExecutionStatus) | optional | The status of the execution |
| epoch | [uint64](#uint64) | optional | The epoch when this transaction was executed. |
| gas_used | [GasCostSummary](#haneul-types-GasCostSummary) | optional |  |
| transaction_digest | [Digest](#haneul-types-Digest) | optional | The transaction digest |
| gas_object_index | [uint32](#uint32) | optional | The updated gas object reference, as an index into the `changed_objects` vector. / Having a dedicated field for convenient access. / System transaction that don&#39;t require gas will leave this as None. |
| events_digest | [Digest](#haneul-types-Digest) | optional | The digest of the events emitted during execution, / can be None if the transaction does not emit any event. |
| dependencies | [Digest](#haneul-types-Digest) | repeated | The set of transaction digests this transaction depends on. |
| lamport_version | [uint64](#uint64) | optional | The version number of all the written Move objects by this transaction. |
| changed_objects | [ChangedObject](#haneul-types-ChangedObject) | repeated | Objects whose state are changed in the object store. |
| unchanged_shared_objects | [UnchangedSharedObject](#haneul-types-UnchangedSharedObject) | repeated | Shared objects that are not mutated in this transaction. Unlike owned objects, / read-only shared objects&#39; version are not committed in the transaction, / and in order for a node to catch up and execute it without consensus sequencing, / the version needs to be committed in the effects. |
| auxiliary_data_digest | [Digest](#haneul-types-Digest) | optional | Auxiliary data that are not protocol-critical, generated as part of the effects but are stored separately. / Storing it separately allows us to avoid bloating the effects with data that are not critical. / It also provides more flexibility on the format and type of the data. |






<a name="haneul-types-TransactionEvents"></a>

### TransactionEvents



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [Event](#haneul-types-Event) | repeated |  |






<a name="haneul-types-TransactionExpiration"></a>

### TransactionExpiration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| none | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| epoch | [uint64](#uint64) |  |  |






<a name="haneul-types-TransactionKind"></a>

### TransactionKind



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| programmable_transaction | [ProgrammableTransaction](#haneul-types-ProgrammableTransaction) |  |  |
| change_epoch | [ChangeEpoch](#haneul-types-ChangeEpoch) |  |  |
| genesis | [GenesisTransaction](#haneul-types-GenesisTransaction) |  |  |
| consensus_commit_prologue_v1 | [ConsensusCommitPrologue](#haneul-types-ConsensusCommitPrologue) |  |  |
| authenticator_state_update | [AuthenticatorStateUpdate](#haneul-types-AuthenticatorStateUpdate) |  |  |
| end_of_epoch | [EndOfEpochTransaction](#haneul-types-EndOfEpochTransaction) |  |  |
| randomness_state_update | [RandomnessStateUpdate](#haneul-types-RandomnessStateUpdate) |  |  |
| consensus_commit_prologue_v2 | [ConsensusCommitPrologue](#haneul-types-ConsensusCommitPrologue) |  |  |
| consensus_commit_prologue_v3 | [ConsensusCommitPrologue](#haneul-types-ConsensusCommitPrologue) |  |  |






<a name="haneul-types-TransferObjects"></a>

### TransferObjects



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [Argument](#haneul-types-Argument) | repeated |  |
| address | [Argument](#haneul-types-Argument) | optional |  |






<a name="haneul-types-TypeArgumentError"></a>

### TypeArgumentError



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type_argument | [uint32](#uint32) | optional |  |
| type_not_found | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| constraint_not_satisfied | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="haneul-types-TypeOrigin"></a>

### TypeOrigin



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| module_name | [Identifier](#haneul-types-Identifier) | optional |  |
| struct_name | [Identifier](#haneul-types-Identifier) | optional |  |
| package_id | [ObjectId](#haneul-types-ObjectId) | optional |  |






<a name="haneul-types-TypeTag"></a>

### TypeTag



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| u8 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u16 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u32 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u64 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u128 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u256 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| bool | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| address | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| signer | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| vector | [TypeTag](#haneul-types-TypeTag) |  |  |
| struct | [StructTag](#haneul-types-StructTag) |  |  |






<a name="haneul-types-U128"></a>

### U128
Little-endian encoded u128


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional |  |






<a name="haneul-types-U256"></a>

### U256
Little-endian encoded u256


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional |  |






<a name="haneul-types-UnchangedSharedObject"></a>

### UnchangedSharedObject



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| read_only_root | [ReadOnlyRoot](#haneul-types-ReadOnlyRoot) |  |  |
| mutate_deleted | [uint64](#uint64) |  |  |
| read_deleted | [uint64](#uint64) |  |  |
| cancelled | [uint64](#uint64) |  |  |
| per_epoch_config | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="haneul-types-Upgrade"></a>

### Upgrade



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| modules | [bytes](#bytes) | repeated |  |
| dependencies | [ObjectId](#haneul-types-ObjectId) | repeated |  |
| package | [ObjectId](#haneul-types-ObjectId) | optional |  |
| ticket | [Argument](#haneul-types-Argument) | optional |  |






<a name="haneul-types-UpgradeInfo"></a>

### UpgradeInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| original_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| upgraded_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| upgraded_version | [uint64](#uint64) | optional |  |






<a name="haneul-types-UserSignature"></a>

### UserSignature



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| simple | [SimpleSignature](#haneul-types-SimpleSignature) |  |  |
| multisig | [MultisigAggregatedSignature](#haneul-types-MultisigAggregatedSignature) |  |  |
| zklogin | [ZkLoginAuthenticator](#haneul-types-ZkLoginAuthenticator) |  |  |
| passkey | [PasskeyAuthenticator](#haneul-types-PasskeyAuthenticator) |  |  |






<a name="haneul-types-ValidatorAggregatedSignature"></a>

### ValidatorAggregatedSignature



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| signature | [bytes](#bytes) | optional |  |
| bitmap | [RoaringBitmap](#haneul-types-RoaringBitmap) | optional |  |






<a name="haneul-types-ValidatorCommittee"></a>

### ValidatorCommittee



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional |  |
| members | [ValidatorCommitteeMember](#haneul-types-ValidatorCommitteeMember) | repeated |  |






<a name="haneul-types-ValidatorCommitteeMember"></a>

### ValidatorCommitteeMember



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) | optional |  |
| stake | [uint64](#uint64) | optional |  |






<a name="haneul-types-VersionAssignment"></a>

### VersionAssignment



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#haneul-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |






<a name="haneul-types-ZkLoginAuthenticator"></a>

### ZkLoginAuthenticator



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| inputs | [ZkLoginInputs](#haneul-types-ZkLoginInputs) | optional |  |
| max_epoch | [uint64](#uint64) | optional |  |
| signature | [SimpleSignature](#haneul-types-SimpleSignature) | optional |  |






<a name="haneul-types-ZkLoginClaim"></a>

### ZkLoginClaim



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [string](#string) | optional |  |
| index_mod_4 | [uint32](#uint32) | optional |  |






<a name="haneul-types-ZkLoginInputs"></a>

### ZkLoginInputs



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| proof_points | [ZkLoginProof](#haneul-types-ZkLoginProof) | optional |  |
| iss_base64_details | [ZkLoginClaim](#haneul-types-ZkLoginClaim) | optional |  |
| header_base64 | [string](#string) | optional |  |
| address_seed | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |






<a name="haneul-types-ZkLoginProof"></a>

### ZkLoginProof



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a | [CircomG1](#haneul-types-CircomG1) | optional |  |
| b | [CircomG2](#haneul-types-CircomG2) | optional |  |
| c | [CircomG1](#haneul-types-CircomG1) | optional |  |






<a name="haneul-types-ZkLoginPublicIdentifier"></a>

### ZkLoginPublicIdentifier



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| iss | [string](#string) | optional |  |
| address_seed | [Bn254FieldElement](#haneul-types-Bn254FieldElement) | optional |  |





 


<a name="haneul-types-SignatureScheme"></a>

### SignatureScheme
note: values do not match their bcs serialized values

| Name | Number | Description |
| ---- | ------ | ----------- |
| SIGNATURE_SCHEME_UNKNOWN | 0 |  |
| SIGNATURE_SCHEME_ED25519 | 1 |  |
| SIGNATURE_SCHEME_SECP256K1 | 2 |  |
| SIGNATURE_SCHEME_SECP256R1 | 3 |  |
| SIGNATURE_SCHEME_MULTISIG | 4 |  |
| SIGNATURE_SCHEME_BLS12381 | 5 |  |
| SIGNATURE_SCHEME_ZKLOGIN | 6 |  |
| SIGNATURE_SCHEME_PASSKEY | 7 |  |


 

 

 



<a name="google_protobuf_empty-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## google/protobuf/empty.proto



<a name="google-protobuf-Empty"></a>

### Empty
A generic empty message that you can re-use to avoid defining duplicated
empty messages in your APIs. A typical example is to use it as the request
or the response type of an API method. For instance:

    service Foo {
      rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
    }





 

 

 

 



<a name="google_protobuf_timestamp-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## google/protobuf/timestamp.proto



<a name="google-protobuf-Timestamp"></a>

### Timestamp
A Timestamp represents a point in time independent of any time zone
or calendar, represented as seconds and fractions of seconds at
nanosecond resolution in UTC Epoch time. It is encoded using the
Proleptic Gregorian Calendar which extends the Gregorian calendar
backwards to year one. It is encoded assuming all minutes are 60
seconds long, i.e. leap seconds are &#34;smeared&#34; so that no leap second
table is needed for interpretation. Range is from
0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z.
By restricting to that range, we ensure that we can convert to
and from  RFC 3339 date strings.
See [https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).

# Examples

Example 1: Compute Timestamp from POSIX `time()`.

    Timestamp timestamp;
    timestamp.set_seconds(time(NULL));
    timestamp.set_nanos(0);

Example 2: Compute Timestamp from POSIX `gettimeofday()`.

    struct timeval tv;
    gettimeofday(&amp;tv, NULL);

    Timestamp timestamp;
    timestamp.set_seconds(tv.tv_sec);
    timestamp.set_nanos(tv.tv_usec * 1000);

Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.

    FILETIME ft;
    GetSystemTimeAsFileTime(&amp;ft);
    UINT64 ticks = (((UINT64)ft.dwHighDateTime) &lt;&lt; 32) | ft.dwLowDateTime;

    // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
    // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
    Timestamp timestamp;
    timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
    timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));

Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.

    long millis = System.currentTimeMillis();

    Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
        .setNanos((int) ((millis % 1000) * 1000000)).build();


Example 5: Compute Timestamp from current time in Python.

    timestamp = Timestamp()
    timestamp.GetCurrentTime()

# JSON Mapping

In JSON format, the Timestamp type is encoded as a string in the
[RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
format is &#34;{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z&#34;
where {year} is always expressed using four digits while {month}, {day},
{hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
are optional. The &#34;Z&#34; suffix indicates the timezone (&#34;UTC&#34;); the timezone
is required, though only UTC (as indicated by &#34;Z&#34;) is presently supported.

For example, &#34;2017-01-15T01:30:15.01Z&#34; encodes 15.01 seconds past
01:30 UTC on January 15, 2017.

In JavaScript, one can convert a Date object to this format using the
standard [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString]
method. In Python, a standard `datetime.datetime` object can be converted
to this format using [`strftime`](https://docs.python.org/2/library/time.html#time.strftime)
with the time format spec &#39;%Y-%m-%dT%H:%M:%S.%fZ&#39;. Likewise, in Java, one
can use the Joda Time&#39;s [`ISODateTimeFormat.dateTime()`](
http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime--)
to obtain a formatter capable of generating timestamps in this format.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| seconds | [int64](#int64) |  | Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive. |
| nanos | [int32](#int32) |  | Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive. |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

