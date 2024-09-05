use crate::proto::blockchain::wallet::{PublicKey, Signature};

use super::{ClientChallenge, PkiClientChallenge, Realm};

pub(crate) struct PkiClientChallengeBuilder;

impl PkiClientChallengeBuilder {
    pub(crate) fn realm(self, realm: Realm) -> Step1 {
        Step1 { realm }
    }
}

pub(crate) struct Step1 {
    realm: Realm,
}

impl Step1 {
    pub(crate) fn challenge(self, challenge: ClientChallenge) -> Step2 {
        let Self { realm } = self;
        Step2 { realm, challenge }
    }
}

pub(crate) struct Step2 {
    realm: Realm,
    challenge: ClientChallenge,
}

impl Step2 {
    pub(crate) fn signature(self, signature: Signature) -> Step3 {
        let Self { realm, challenge } = self;
        Step3 {
            realm,
            challenge,
            signature,
        }
    }
}

pub(crate) struct Step3 {
    realm: Realm,
    challenge: ClientChallenge,
    signature: Signature,
}

impl Step3 {
    pub(crate) fn public_key(self, public_key: PublicKey) -> Step4 {
        let Self {
            realm,
            challenge,
            signature,
        } = self;
        Step4 {
            realm,
            challenge,
            signature,
            public_key,
        }
    }
}

pub(crate) struct Step4 {
    realm: Realm,
    challenge: ClientChallenge,
    signature: Signature,
    public_key: PublicKey,
}

impl Step4 {
    pub(crate) fn finish(self) -> PkiClientChallenge {
        let Self {
            realm,
            challenge,
            signature,
            public_key,
        } = self;
        PkiClientChallenge {
            realm,
            challenge,
            signature,
            public_key,
        }
    }
}
