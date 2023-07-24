// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::error::FastCryptoError;
use hyper::header::InvalidHeaderValue;
use jsonrpsee::core::Error as RpcError;
use jsonrpsee::types::error::CallError;
use jsonrpsee::types::ErrorObject;
use haneul_types::error::{HaneulError, HaneulObjectResponseError, UserInputError};
use haneul_types::quorum_driver_types::{QuorumDriverError, NON_RECOVERABLE_ERROR_MSG};
use thiserror::Error;
use tokio::task::JoinError;

pub type RpcInterimResult<T = ()> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    HaneulError(HaneulError),

    #[error(transparent)]
    InternalError(#[from] anyhow::Error),

    #[error("Deserialization error: {0}")]
    BcsError(#[from] bcs::Error),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error(transparent)]
    RPCServerError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    UserInputError(#[from] UserInputError),

    #[error(transparent)]
    EncodingError(#[from] eyre::Report),

    #[error(transparent)]
    TokioJoinError(#[from] JoinError),

    #[error(transparent)]
    QuorumDriverError(#[from] QuorumDriverError),

    #[error(transparent)]
    FastCryptoError(#[from] FastCryptoError),

    #[error(transparent)]
    HaneulObjectResponseError(#[from] HaneulObjectResponseError),

    #[error(transparent)]
    HaneulRpcInputError(#[from] HaneulRpcInputError),
}

impl From<Error> for RpcError {
    fn from(e: Error) -> Self {
        e.to_rpc_error()
    }
}

impl From<HaneulError> for Error {
    fn from(e: HaneulError) -> Self {
        match e {
            HaneulError::UserInputError { error } => Self::UserInputError(error),
            HaneulError::HaneulObjectResponseError { error } => Self::HaneulObjectResponseError(error),
            other => Self::HaneulError(other),
        }
    }
}

impl Error {
    pub fn to_rpc_error(self) -> RpcError {
        match self {
            Error::UserInputError(_) => RpcError::Call(CallError::InvalidParams(self.into())),
            Error::HaneulObjectResponseError(err) => match err {
                HaneulObjectResponseError::NotExists { .. }
                | HaneulObjectResponseError::DynamicFieldNotFound { .. }
                | HaneulObjectResponseError::Deleted { .. }
                | HaneulObjectResponseError::DisplayError { .. } => {
                    RpcError::Call(CallError::InvalidParams(err.into()))
                }
                _ => RpcError::Call(CallError::Failed(err.into())),
            },
            Error::HaneulRpcInputError(err) => err.into(),
            Error::HaneulError(haneul_error) => match haneul_error {
                HaneulError::TransactionNotFound { .. }
                | HaneulError::TransactionsNotFound { .. }
                | HaneulError::TransactionEventsNotFound { .. } => {
                    RpcError::Call(CallError::InvalidParams(haneul_error.into()))
                }
                _ => RpcError::Call(CallError::Failed(haneul_error.into())),
            },
            Error::QuorumDriverError(err) => match err {
                QuorumDriverError::NonRecoverableTransactionError { errors } => {
                    let error_object =
                        ErrorObject::owned(-32000, NON_RECOVERABLE_ERROR_MSG, Some(errors));
                    RpcError::Call(CallError::Custom(error_object))
                }
                _ => RpcError::Call(CallError::Failed(err.into())),
            },
            _ => RpcError::Call(CallError::Failed(self.into())),
        }
    }
}

#[derive(Debug, Error)]
pub enum HaneulRpcInputError {
    #[error("Input contains duplicates")]
    ContainsDuplicates,

    #[error("Input exceeds limit of {0}")]
    SizeLimitExceeded(String),

    #[error("{0}")]
    GenericNotFound(String),

    #[error("{0}")]
    GenericInvalid(String),

    #[error("request_type` must set to `None` or `WaitForLocalExecution` if effects is required in the response")]
    InvalidExecuteTransactionRequestType,

    #[error("Unsupported protocol version requested. Min supported: {0}, max supported: {1}")]
    ProtocolVersionUnsupported(u64, u64),

    #[error("{0}")]
    CannotParseHaneulStructTag(String),

    #[error(transparent)]
    Base64(#[from] eyre::Report),

    #[error("Deserialization error: {0}")]
    Bcs(#[from] bcs::Error),

    #[error(transparent)]
    FastCryptoError(#[from] FastCryptoError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    UserInputError(#[from] UserInputError),
}

impl From<HaneulRpcInputError> for RpcError {
    fn from(e: HaneulRpcInputError) -> Self {
        RpcError::Call(CallError::InvalidParams(e.into()))
    }
}
