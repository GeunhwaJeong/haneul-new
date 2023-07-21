// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { fromExportedKeypair } from './cryptography/utils.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/ed5519` instead */
	type Ed25519KeypairData,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/ed5519` instead */
	Ed25519Keypair,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/ed5519` instead */
	Ed25519PublicKey,
} from './keypairs/ed25519/index.js';
export {
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256k1` instead */
	DEFAULT_SECP256K1_DERIVATION_PATH,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256k1` instead */
	type Secp256k1KeypairData,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256k1` instead */
	Secp256k1Keypair,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256k1` instead */
	Secp256k1PublicKey,
} from './keypairs/secp256k1/index.js';
export {
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256r1` instead */
	DEFAULT_SECP256R1_DERIVATION_PATH,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256r1` instead */
	type Secp256r1KeypairData,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256r1` instead */
	Secp256r1Keypair,
	/** @deprecated Import from `@haneullabs/haneul.js/keypairs/secp256k1` instead */
	Secp256r1PublicKey,
} from './keypairs/secp256r1/index.js';
export {
	/** @deprecated Signing methods are now available on the KeyPair classes */
	BaseSigner,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type ExportedKeypair,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	Keypair,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	LEGACY_PRIVATE_KEY_SIZE,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	PRIVATE_KEY_SIZE,
} from './cryptography/keypair.js';
export {
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	type CompressedSignature,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	MAX_SIGNER_IN_MULTISIG,
	/** @deprecated Use the MultiSigStruct from `@haneullabs/haneul.js/multisig` instead */
	type MultiSig,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	type MultiSigPublicKey,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	type PubkeyEnumWeightPair,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	type PubkeyWeightPair,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	type PublicKeyEnum,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	combinePartialSigs,
	/** @deprecated Use the parseSerializedSignature from `@haneullabs/haneul.js/cryptography` instead */
	decodeMultiSig,
	/** @deprecated Use the MultiSigPublicKey class from `@haneullabs/haneul.js/multisig` instead */
	toMultiSigAddress,
} from './cryptography/multisig.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	PublicKey,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type PublicKeyInitData,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	bytesEqual,
} from './cryptography/publickey.js';
export {
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	isValidBIP32Path,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	isValidHardenedPath,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	mnemonicToSeed,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	mnemonicToSeedHex,
} from './cryptography/mnemonics.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	SIGNATURE_FLAG_TO_SCHEME,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	SIGNATURE_SCHEME_TO_FLAG,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	SIGNATURE_SCHEME_TO_SIZE,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type SerializeSignatureInput,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type SerializedSignature,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type SignatureFlag,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type SignatureScheme,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	parseSerializedSignature,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	toSerializedSignature,
} from './cryptography/signature.js';

export {
	/** @deprecated This type will be removed because it cant accurately represent parsed signatures */
	type SignaturePubkeyPair,
	/** @deprecated use `publicKeyFromBytes` from `@haneullabs/haneul.j/verify` instead */
	publicKeyFromSerialized,
	/** @deprecated use `parseSerializedSignature` from `@haneullabs/haneul.j/cryptography` instead */
	toParsedSignaturePubkeyPair,
	/** @deprecated use `parseSerializedSignature` from `@haneullabs/haneul.j/cryptography` instead */
	toSingleSignaturePubkeyPair,
} from './cryptography/utils.js';
export {
	/** @deprecated Use `HaneulClient` from `@haneullabs/haneul.js/client` instead */
	JsonRpcProvider,
	/** @deprecated Import from `@haneullabs/haneul.js/client` instead */
	type OrderArguments,
	/** @deprecated Import from `@haneullabs/haneul.js/client` instead */
	type PaginationArguments,
	/** @deprecated Use `HaneulClientOptions` from `@haneullabs/haneul.js/client` instead */
	type RpcProviderOptions,
} from './providers/json-rpc-provider.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/client` instead */
	type HttpHeaders,
	/** @deprecated This client will not be exported in the future */
	JsonRpcClient,
} from './rpc/client.js';

export {
	/** @deprecated Use `getFullnodeUrl` from `@haneullabs/haneul.js/client` instead */
	Connection,
	/** @deprecated Use `getFullnodeUrl` from `@haneullabs/haneul.js/client` instead */
	devnetConnection,
	/** @deprecated Use `getFullnodeUrl` from `@haneullabs/haneul.js/client` instead */
	localnetConnection,
	/** @deprecated Use `getFullnodeUrl` from `@haneullabs/haneul.js/client` instead */
	mainnetConnection,
	/** @deprecated Use `getFullnodeUrl` from `@haneullabs/haneul.js/client` instead */
	testnetConnection,
} from './rpc/connection.js';

export {
	/** @deprecated This will not be exported from future version of this package */
	TypeTagSerializer,
} from './builder/type-tag-serializer.js';

export {
	/** @deprecated Use KeyPair classes from `@haneullabs/haneul.js/keypairs/*` instead */
	type Signer,
} from './signers/signer.js';
export {
	/** @deprecated Use KeyPair classes from `@haneullabs/haneul.js/keypairs/*` instead */
	RawSigner,
} from './signers/raw-signer.js';
export {
	/** @deprecated Use KeyPair classes from `@haneullabs/haneul.js/keypairs/*` instead */
	SignerWithProvider,
} from './signers/signer-with-provider.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	AppId,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	type Intent,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	IntentScope,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	IntentVersion,
	/** @deprecated Import from `@haneullabs/haneul.js/cryptography` instead */
	messageWithIntent,
} from './cryptography/intent.js';

export {
	/** @deprecated Use verify methods on PublicKey classes from `@haneullabs/haneul.js/keypairs/*` instead */
	verifyMessage,
} from './utils/verify.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/client` instead */
	RPCValidationError,
} from './rpc/errors.js';

export {
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	fromB64,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	toB64,
} from '@haneullabs/bcs';

export {
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	HANEUL_ADDRESS_LENGTH,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	isValidHaneulAddress,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	isValidHaneulObjectId,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	isValidTransactionDigest,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	normalizeStructTag,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	normalizeHaneulAddress,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	normalizeHaneulObjectId,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	parseStructTag,
} from './utils/haneul-types.js';

export {
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	formatAddress,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	formatDigest,
} from './utils/format.js';

export {
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	is,
	/** @deprecated Import from @haneullabs/haneul.js/utils instead */
	assert,
} from 'superstruct';

export {
	/** @deprecated this will be removed in future versions of the SDK */
	DEFAULT_CLIENT_OPTIONS,
	/** @deprecated this will be removed in future versions of the SDK */
	WebsocketClient,
	/** @deprecated this will be removed in future versions of the SDK */
	type WebsocketClientOptions,
	/** @deprecated this will be removed in future versions of the SDK */
	getWebsocketUrl,
} from './rpc/websocket-client.js';

export {
	/** @deprecated messages are now signed through the KeyPair classes */
	type SignedMessage,
	/**
	 * deprecated transactions are now signed through the KeyPair classes
	 * or signed and executed directly using the HaneulClient
	 */
	type SignedTransaction,
} from './signers/types.js';

export {
	/** @deprecated Import from `@haneullabs/haneul.js/transactions` instead */
	builder,
	/** @deprecated Import from `@haneullabs/haneul.js/transactions` instead */
	Transactions,
	/** @deprecated Import from `@haneullabs/haneul.js/transactions` instead */
	Inputs,
	/** @deprecated Import from `@haneullabs/haneul.js/transactions` instead */
	TransactionBlock,
	/** @deprecated Import from `@haneullabs/haneul.js/transactions` instead */
	TransactionArgument,
	ARGUMENT,
	ARGUMENT_INNER,
	BuilderCallArg,
	CALL_ARG,
	COMPRESSED_SIGNATURE,
	ENUM_KIND,
	MULTISIG,
	MULTISIG_PK_MAP,
	MULTISIG_PUBLIC_KEY,
	MakeMoveVecTransaction,
	MergeCoinsTransaction,
	MoveCallTransaction,
	OBJECT_ARG,
	OPTION,
	ObjectCallArg,
	ObjectTransactionArgument,
	type Option,
	PROGRAMMABLE_CALL,
	PROGRAMMABLE_CALL_INNER,
	PROGRAMMABLE_TX_BLOCK,
	PUBLIC_KEY,
	PublishTransaction,
	PureCallArg,
	PureTransactionArgument,
	SplitCoinsTransaction,
	TRANSACTION,
	TRANSACTION_INNER,
	TYPE_TAG,
	TransactionBlockInput,
	TransactionType,
	TransferObjectsTransaction,
	UpgradePolicy,
	UpgradeTransaction,
	VECTOR,
	getIdFromCallArg,
	getPureSerializationType,
	getSharedObjectInput,
	getTransactionType,
	isMutableSharedObjectInput,
	isSharedObjectInput,
	isTxContext,
} from './builder/index.js';

export {
	ADD_STAKE_FUN_NAME,
	ADD_STAKE_LOCKED_COIN_FUN_NAME,
	COIN_TYPE_ARG_REGEX,
	Coin,
	type CoinMetadata,
	CoinMetadataStruct,
	Delegation,
	type DelegationData,
	type DelegationHaneulObject,
	ID_STRUCT_NAME,
	MOVE_STDLIB_ADDRESS,
	OBJECT_MODULE_NAME,
	PAY_JOIN_COIN_FUNC_NAME,
	PAY_MODULE_NAME,
	PAY_SPLIT_COIN_VEC_FUNC_NAME,
	HANEUL_CLOCK_OBJECT_ID,
	HANEUL_FRAMEWORK_ADDRESS,
	HANEUL_SYSTEM_ADDRESS,
	HANEUL_SYSTEM_MODULE_NAME,
	HANEUL_SYSTEM_STATE_OBJECT_ID,
	HANEUL_TYPE_ARG,
	HaneulSystemStateUtil,
	UID_STRUCT_NAME,
	VALIDATORS_EVENTS_QUERY,
	WITHDRAW_STAKE_FUN_NAME,
	isObjectDataFull,
} from './framework/index.js';

export * from './types/index.js';

export { fromExportedKeypair };
