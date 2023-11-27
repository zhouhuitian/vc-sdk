use crate::{
    identity_management::xtbuilder::IdentityManagementXtBuilder,
    primitives::{address::Address32, identity::Identity, identity::ValidationData, MrEnclave},
    ApiClient, SendExtrinsic,
};
use substrate_api_client::ac_primitives::Config;
use substrate_api_client::ac_primitives::{ExtrinsicParams, SignExtrinsic, UncheckedExtrinsicV4};

use super::IdentityManagementApi;

impl<T> IdentityManagementApi for ApiClient<T>
where
    T: Config,
{
    type Extrinsic<Call> = UncheckedExtrinsicV4<
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::ExtrinsicAddress,
        Call,
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::Signature,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::SignedExtra,
    >;

    fn add_delegatee(&self, account: &Address32) {
        let xt = self.build_extrinsic_add_delegatee(account);
        self.send_extrinsic(xt);
    }

    fn create_identity(
        &self,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) {
        let xt =
            self.build_extrinsic_create_identity(shard, address, identity, ciphertext_metadata);
        self.send_extrinsic(xt);
    }

    fn create_identity_offline(
        &self,
        nonce: u32,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    ) {
        let xt = self.build_extrinsic_offline_create_identity(
            nonce,
            shard,
            address,
            identity,
            ciphertext_metadata,
        );
        self.send_extrinsic(xt);
    }

    fn remove_identity(&self, shard: &MrEnclave, identity: &Identity) {
        let xt = self.build_extrinsic_remove_identity(shard, identity);
        self.send_extrinsic(xt);
    }

    fn verify_identity(&self, shard: &MrEnclave, identity: &Identity, vdata: &ValidationData) {
        let xt = self.build_extrinsic_verify_identity(shard, identity, vdata);
        self.send_extrinsic(xt);
    }
}
