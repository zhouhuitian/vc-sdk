use crate::{
    api_client_patch::parachain::ParachainPatch,
    identity_management::IDENTITY_PALLET_NAME,
    primitives::{
        address::Address32,
        identity::{Identity, ValidationData},
        MrEnclave,
    },
    utils::crypto::encrypt_with_tee_shielding_pubkey,
    ApiClient,
};
use codec::Encode;
use rsa::RsaPublicKey;
use sp_core::H256;
use substrate_api_client::{
    ac_compose_macros::{compose_call, compose_extrinsic, compose_extrinsic_offline},
    ac_primitives::{
        extrinsics::CallIndex, Config, ExtrinsicParams, SignExtrinsic, UncheckedExtrinsicV4,
    },
    api::Result as ApiResult,
};

pub type AddDelegateFn = (CallIndex, Address32);
pub type CreateIdentityFn = (CallIndex, H256, Address32, Vec<u8>, Option<Vec<u8>>);
pub type RemoveIdentityFn = (CallIndex, H256, Vec<u8>);
pub type VerifyIdentityFn = (CallIndex, H256, Vec<u8>, Vec<u8>);

pub trait IdentityManagementXtBuilder {
    type Extrinsic<Call>;

    fn build_extrinsic_add_delegatee(&self, account: &Address32) -> Self::Extrinsic<AddDelegateFn>;

    fn build_extrinsic_create_identity(
        &self,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) -> Self::Extrinsic<CreateIdentityFn>;

    fn build_extrinsic_offline_create_identity(
        &self,
        nonce: u32,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) -> Self::Extrinsic<CreateIdentityFn>;

    fn build_extrinsic_remove_identity(
        &self,
        shard: &MrEnclave,
        identity: &Identity,
    ) -> Self::Extrinsic<RemoveIdentityFn>;

    fn build_extrinsic_verify_identity(
        &self,
        shard: &MrEnclave,
        identity: &Identity,
        validation_data: &ValidationData,
    ) -> Self::Extrinsic<VerifyIdentityFn>;

    fn encrypt_identity_with_tee_shielding_key(
        tee_shielding_pubkey: RsaPublicKey,
        identity: Identity,
    ) -> ApiResult<Vec<u8>>;
}

impl<T> IdentityManagementXtBuilder for ApiClient<T>
where
    T: Config,
{
    type Extrinsic<Call> = UncheckedExtrinsicV4<
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::ExtrinsicAddress,
        Call,
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::Signature,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::SignedExtra,
    >;

    fn encrypt_identity_with_tee_shielding_key(
        tee_shielding_pubkey: RsaPublicKey,
        identity: Identity,
    ) -> ApiResult<Vec<u8>> {
        let identity_encoded = identity.encode();
        let encrypted_identity =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

        Ok(encrypted_identity)
    }

    fn build_extrinsic_add_delegatee(&self, account: &Address32) -> Self::Extrinsic<AddDelegateFn> {
        // compose_extrinsic!(self.api, IDENTITY_PALLET_NAME, "add_delegatee", *account)
        todo!()
    }

    fn build_extrinsic_create_identity(
        &self,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) -> Self::Extrinsic<CreateIdentityFn> {
        let identity_encoded = identity.encode();
        let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
        let encrypted_identity =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

        // compose_extrinsic!(
        //     self.api,
        //     IDENTITY_PALLET_NAME,
        //     "create_identity",
        //     H256::from(shard),
        //     *address,
        //     encrypted_identity,
        //     ciphertext_metadata.clone()
        // )

        todo!()
    }

    fn build_extrinsic_offline_create_identity(
        &self,
        nonce: u32,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) -> Self::Extrinsic<CreateIdentityFn> {
        let identity_encoded = identity.encode();
        let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
        let encrypted_identity =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

        let meta = self.api.metadata();
        let call = compose_call!(
            meta,
            IDENTITY_PALLET_NAME,
            "create_identity",
            H256::from(shard),
            *address,
            encrypted_identity,
            ciphertext_metadata.clone()
        );

        // compose_extrinsic_offline!(
        //     self.api.signer().unwrap(),
        //     call,
        //     self.api.extrinsic_params(nonce.into())
        // )

        todo!()
    }

    fn build_extrinsic_remove_identity(
        &self,
        shard: &MrEnclave,
        identity: &Identity,
    ) -> Self::Extrinsic<RemoveIdentityFn> {
        let identity_encoded = identity.encode();
        let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
        let encrypted_identity =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

        // compose_extrinsic!(
        //     self.api,
        //     IDENTITY_PALLET_NAME,
        //     "remove_identity",
        //     H256::from(shard),
        //     encrypted_identity
        // )

        todo!()
    }

    fn build_extrinsic_verify_identity(
        &self,
        shard: &MrEnclave,
        identity: &Identity,
        validation_data: &ValidationData,
    ) -> Self::Extrinsic<VerifyIdentityFn> {
        let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();

        let identity_encoded = identity.encode();
        let encrypted_identity =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);
        let validation_data_encoded = validation_data.encode();
        let encrypted_validation_data =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &validation_data_encoded);

        // compose_extrinsic!(
        //     self.api,
        //     IDENTITY_PALLET_NAME,
        //     "verify_identity",
        //     H256::from(shard),
        //     encrypted_identity,
        //     encrypted_validation_data
        // )

        todo!()
    }
}
