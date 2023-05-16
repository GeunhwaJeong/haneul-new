// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::api::MoveUtilsServer;
use crate::error::{Error, HaneulRpcInputError};
use crate::read_api::{get_move_module, get_move_modules_by_package};
use crate::{with_tracing, HaneulRpcModule};
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_binary_format::file_format_common::VERSION_MAX;
use move_binary_format::normalized::Type;
use move_core_types::identifier::Identifier;
use std::collections::BTreeMap;
use std::sync::Arc;
use haneul_core::authority::AuthorityState;
use haneul_json_rpc_types::{
    MoveFunctionArgType, ObjectValueKind, HaneulMoveNormalizedFunction, HaneulMoveNormalizedModule,
    HaneulMoveNormalizedStruct,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::ObjectID;
use haneul_types::move_package::normalize_modules;
use haneul_types::object::{Data, ObjectRead};
use tracing::instrument;

pub struct MoveUtils {
    state: Arc<AuthorityState>,
}

impl MoveUtils {
    pub fn new(state: Arc<AuthorityState>) -> Self {
        Self { state }
    }
}

impl HaneulRpcModule for MoveUtils {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::MoveUtilsOpenRpc::module_doc()
    }
}

#[async_trait]
impl MoveUtilsServer for MoveUtils {
    #[instrument(skip(self))]
    async fn get_normalized_move_modules_by_package(
        &self,
        package: ObjectID,
    ) -> RpcResult<BTreeMap<String, HaneulMoveNormalizedModule>> {
        with_tracing!(async move {
            let modules = get_move_modules_by_package(&self.state, package).await?;
            Ok(modules
                .into_iter()
                .map(|(name, module)| (name, module.into()))
                .collect::<BTreeMap<String, HaneulMoveNormalizedModule>>())
        })
    }

    #[instrument(skip(self))]
    async fn get_normalized_move_module(
        &self,
        package: ObjectID,
        module_name: String,
    ) -> RpcResult<HaneulMoveNormalizedModule> {
        with_tracing!(async move {
            let module = get_move_module(&self.state, package, module_name).await?;
            Ok(module.into())
        })
    }

    #[instrument(skip(self))]
    async fn get_normalized_move_struct(
        &self,
        package: ObjectID,
        module_name: String,
        struct_name: String,
    ) -> RpcResult<HaneulMoveNormalizedStruct> {
        with_tracing!(async move {
            let module = get_move_module(&self.state, package, module_name).await?;
            let structs = module.structs;
            let identifier = Identifier::new(struct_name.as_str()).map_err(|e| {
                Error::HaneulRpcInputError(HaneulRpcInputError::GenericInvalid(format!("{e}")))
            })?;
            Ok(match structs.get(&identifier) {
                Some(struct_) => Ok(struct_.clone().into()),
                None => Err(Error::HaneulRpcInputError(HaneulRpcInputError::GenericNotFound(
                    format!("No struct was found with struct name {}", struct_name),
                ))),
            }?)
        })
    }

    #[instrument(skip(self))]
    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<HaneulMoveNormalizedFunction> {
        with_tracing!(async move {
            let module = get_move_module(&self.state, package, module_name).await?;
            let functions = module.functions;
            let identifier = Identifier::new(function_name.as_str()).map_err(|e| {
                Error::HaneulRpcInputError(HaneulRpcInputError::GenericInvalid(format!("{e}")))
            })?;
            Ok(match functions.get(&identifier) {
                Some(function) => Ok(function.clone().into()),
                None => Err(Error::HaneulRpcInputError(HaneulRpcInputError::GenericNotFound(
                    format!("No function was found with function name {}", function_name),
                ))),
            }?)
        })
    }

    #[instrument(skip(self))]
    async fn get_move_function_arg_types(
        &self,
        package: ObjectID,
        module: String,
        function: String,
    ) -> RpcResult<Vec<MoveFunctionArgType>> {
        with_tracing!(async move {
            let object_read = self.state.get_object_read(&package).map_err(Error::from)?;

            let normalized = match object_read {
                ObjectRead::Exists(_obj_ref, object, _layout) => match object.data {
                    Data::Package(p) => {
                        // we are on the read path - it's OK to use VERSION_MAX of the supported Move
                        // binary format
                        normalize_modules(
                            p.serialized_module_map().values(),
                            /* max_binary_format_version */ VERSION_MAX,
                            /* no_extraneous_module_bytes */ false,
                        )
                        .map_err(Error::from)
                    }
                    _ => Err(Error::HaneulRpcInputError(HaneulRpcInputError::GenericInvalid(
                        format!("Object is not a package with ID {}", package),
                    ))),
                },
                _ => Err(Error::HaneulRpcInputError(HaneulRpcInputError::GenericNotFound(
                    format!("Package object does not exist with ID {}", package),
                ))),
            }?;

            let identifier = Identifier::new(function.as_str()).map_err(|e| {
                Error::HaneulRpcInputError(HaneulRpcInputError::GenericInvalid(format!("{e}")))
            })?;
            let parameters = normalized
                .get(&module)
                .and_then(|m| m.functions.get(&identifier).map(|f| f.parameters.clone()));

            Ok(match parameters {
                Some(parameters) => Ok(parameters
                    .iter()
                    .map(|p| match p {
                        Type::Struct {
                            address: _,
                            module: _,
                            name: _,
                            type_arguments: _,
                        } => MoveFunctionArgType::Object(ObjectValueKind::ByValue),
                        Type::Reference(_) => {
                            MoveFunctionArgType::Object(ObjectValueKind::ByImmutableReference)
                        }
                        Type::MutableReference(_) => {
                            MoveFunctionArgType::Object(ObjectValueKind::ByMutableReference)
                        }
                        _ => MoveFunctionArgType::Pure,
                    })
                    .collect::<Vec<MoveFunctionArgType>>()),
                None => Err(Error::HaneulRpcInputError(HaneulRpcInputError::GenericNotFound(
                    format!("No parameters found for function {}", function),
                ))),
            }?)
        })
    }
}
