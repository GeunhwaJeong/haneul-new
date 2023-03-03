// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use haneul_types::committee::Committee;
use haneul_types::crypto::AuthoritySignInfo;
use haneul_types::error::HaneulResult;
use haneul_types::message_envelope::{Envelope, Message, VerifiedEnvelope};

pub trait SignatureVerifier: Sync + Send + Clone + 'static {
    fn verify_one<T: Message + Serialize>(
        &self,
        envelope: Envelope<T, AuthoritySignInfo>,
        committee: &Committee,
    ) -> HaneulResult<VerifiedEnvelope<T, AuthoritySignInfo>>;
}

#[derive(Default, Clone)]
pub struct DefaultSignatureVerifier;

impl SignatureVerifier for DefaultSignatureVerifier {
    fn verify_one<T: Message + Serialize>(
        &self,
        envelope: Envelope<T, AuthoritySignInfo>,
        committee: &Committee,
    ) -> HaneulResult<VerifiedEnvelope<T, AuthoritySignInfo>> {
        envelope.verify(committee)
    }
}

#[derive(Default, Clone)]
pub struct IgnoreSignatureVerifier;

impl SignatureVerifier for IgnoreSignatureVerifier {
    fn verify_one<T: Message + Serialize>(
        &self,
        envelope: Envelope<T, AuthoritySignInfo>,
        _committee: &Committee,
    ) -> HaneulResult<VerifiedEnvelope<T, AuthoritySignInfo>> {
        Ok(VerifiedEnvelope::new_unchecked(envelope))
    }
}
