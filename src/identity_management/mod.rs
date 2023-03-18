pub mod api;
pub mod events;

use crate::{
    primitives::{Address32, MrEnclave},
    API,
};
use sp_core::H256;
use substrate_api_client::{
    compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};

pub const IDENTITY_PALLET_NAME: &str = "IdentityManagement";

pub type SetUserShieldingKeyFn = (CallIndex, H256, Vec<u8>);
pub type SetUserShieldingKeyXt<SignedExtra> =
    UncheckedExtrinsicV4<SetUserShieldingKeyFn, SignedExtra>;
pub type CreateIdentityFn = (CallIndex, H256, Address32, Vec<u8>, Option<Vec<u8>>);
pub type CreateIdentityXt<SignedExtra> = UncheckedExtrinsicV4<CreateIdentityFn, SignedExtra>;

pub fn build_set_user_shielding_key_extrinsic(
    shard: MrEnclave,
    encrpted_shielding_key: Vec<u8>,
) -> SetUserShieldingKeyXt<SubstrateDefaultSignedExtra<PlainTip>> {
    compose_extrinsic!(
        API.clone(),
        IDENTITY_PALLET_NAME,
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key
    )
}

pub fn build_create_identity_extrinsic(
    shard: MrEnclave,
    address: Address32,
    ciphertext: Vec<u8>,
    ciphertext_metadata: Option<Vec<u8>>,
) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
    compose_extrinsic!(
        API.clone(),
        IDENTITY_PALLET_NAME,
        "create_identity",
        H256::from(shard),
        address,
        ciphertext,
        ciphertext_metadata
    )
}
