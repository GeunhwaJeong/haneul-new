// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Encoding;
use move_binary_format::CompiledModule;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use haneul_types::{base_types::TxContext, crypto::PublicKeyBytes, object::Object};
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Genesis {
    #[serde_as(as = "Vec<Vec<SerdeCompiledModule>>")]
    modules: Vec<Vec<CompiledModule>>,
    objects: Vec<Object>,
    genesis_ctx: TxContext,
}

impl Genesis {
    pub fn modules(&self) -> &[Vec<CompiledModule>] {
        &self.modules
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn genesis_ctx(&self) -> &TxContext {
        &self.genesis_ctx
    }

    pub fn get_default_genesis() -> Self {
        Builder::new(haneul_adapter::genesis::get_genesis_context()).build()
    }
}

struct SerdeCompiledModule;

impl SerializeAs<CompiledModule> for SerdeCompiledModule {
    fn serialize_as<S>(module: &CompiledModule, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;

        let mut serialized_module = Vec::new();
        module
            .serialize(&mut serialized_module)
            .map_err(|e| Error::custom(e.to_string()))?;

        if serializer.is_human_readable() {
            let s = base64ct::Base64::encode_string(serialized_module.as_ref());
            s.serialize(serializer)
        } else {
            serialized_module.serialize(serializer)
        }
    }
}

impl<'de> DeserializeAs<'de, CompiledModule> for SerdeCompiledModule {
    fn deserialize_as<D>(deserializer: D) -> Result<CompiledModule, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            let data =
                base64ct::Base64::decode_vec(&s).map_err(|e| Error::custom(e.to_string()))?;
            CompiledModule::deserialize(&data).map_err(|e| Error::custom(e.to_string()))
        } else {
            let data: Vec<u8> = Vec::deserialize(deserializer)?;
            CompiledModule::deserialize(&data).map_err(|e| Error::custom(e.to_string()))
        }
    }
}

pub struct Builder {
    haneul_framework: Option<Vec<CompiledModule>>,
    move_framework: Option<Vec<CompiledModule>>,
    move_modules: Vec<Vec<CompiledModule>>,
    objects: Vec<Object>,
    genesis_ctx: TxContext,
    validators: Vec<(PublicKeyBytes, usize)>,
}

impl Builder {
    pub fn new(genesis_ctx: TxContext) -> Self {
        Self {
            haneul_framework: None,
            move_framework: None,
            move_modules: vec![],
            objects: vec![],
            genesis_ctx,
            validators: vec![],
        }
    }

    pub fn haneul_framework(mut self, haneul_framework: Vec<CompiledModule>) -> Self {
        self.haneul_framework = Some(haneul_framework);
        self
    }

    pub fn move_framework(mut self, move_framework: Vec<CompiledModule>) -> Self {
        self.move_framework = Some(move_framework);
        self
    }

    pub fn add_move_modules(mut self, modules: Vec<Vec<CompiledModule>>) -> Self {
        self.move_modules = modules;
        self
    }

    pub fn add_object(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }

    pub fn add_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects.extend(objects);
        self
    }

    // pub fn add_account(mut self, config: AccountConfig) -> Self {
    //     self.accounts.push(config);
    //     self
    // }

    //TODO actually use the validators added to genesis
    pub fn add_validator(mut self, public_key: PublicKeyBytes, stake: usize) -> Self {
        self.validators.push((public_key, stake));
        self
    }

    pub fn build(self) -> Genesis {
        let mut modules = Vec::new();
        let objects = self.objects;

        // Load Move Framework
        info!("Loading Move framework lib from {:?}", self.move_framework);
        let move_modules = self
            .move_framework
            .unwrap_or_else(haneul_framework::get_move_stdlib);
        // let move_framework =
        //     Object::new_package(move_modules.clone(), TransactionDigest::genesis());
        modules.push(move_modules);
        // objects.push(move_framework);

        // Load Haneul Framework
        info!("Loading Haneul framework lib from {:?}", self.haneul_framework);
        let haneul_modules = self
            .haneul_framework
            .unwrap_or_else(haneul_framework::get_haneul_framework);
        // let haneul_framework = Object::new_package(haneul_modules.clone(), TransactionDigest::genesis());
        modules.push(haneul_modules);
        // objects.push(haneul_framework);

        // add custom modules
        modules.extend(self.move_modules);

        Genesis {
            modules,
            objects,
            genesis_ctx: self.genesis_ctx,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Genesis;

    #[test]
    fn roundtrip() {
        let haneul_lib = haneul_framework::get_haneul_framework();

        let genesis = Genesis {
            modules: vec![haneul_lib],
            objects: vec![],
            genesis_ctx: haneul_adapter::genesis::get_genesis_context(),
        };

        let s = serde_json::to_string_pretty(&genesis).unwrap();
        let from_s = serde_json::from_str(&s).unwrap();
        assert_eq!(genesis, from_s);
    }
}
