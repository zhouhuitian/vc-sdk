use crate::{
    primitives::{assertion::Assertion, MrEnclave},
    vc_management::VC_PALLET_NAME,
    ApiClient,
};
use sp_core::H256;
use substrate_api_client::{
    ac_compose_macros::compose_extrinsic,
    ac_primitives::{
        extrinsics::{CallIndex, UncheckedExtrinsicV4},
        Config, ExtrinsicParams, SignExtrinsic,
    },
};

pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCDisableFn = (CallIndex, H256);
pub type VCRevokeFn = (CallIndex, H256);

pub trait VcManagementXtBuilder {
    type Extrinsic<Call>;

    fn build_extrinsic_request_vc(
        &self,
        shard: &MrEnclave,
        assertion: &Assertion,
    ) -> Self::Extrinsic<VCRequestFn>;

    fn build_extrinsic_disable_vc(&self, vc_index: &H256) -> Self::Extrinsic<VCDisableFn>;

    fn build_extrinsic_revoke_vc(&self, vc_index: &H256) -> Self::Extrinsic<VCRevokeFn>;
}

impl<T> VcManagementXtBuilder for ApiClient<T>
where
    T: Config,
{
    type Extrinsic<Call> = UncheckedExtrinsicV4<
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::ExtrinsicAddress,
        Call,
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::Signature,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::SignedExtra,
    >;

    fn build_extrinsic_request_vc(
        &self,
        shard: &MrEnclave,
        assertion: &Assertion,
    ) -> Self::Extrinsic<VCRequestFn> {
        // compose_extrinsic!(
        //     &self.api,
        //     VC_PALLET_NAME,
        //     "request_vc",
        //     H256::from(shard),
        //     assertion.clone()
        // )

        todo!()
    }

    fn build_extrinsic_disable_vc(&self, vc_index: &H256) -> Self::Extrinsic<VCDisableFn> {
        // compose_extrinsic!(self.api, VC_PALLET_NAME, "disable_vc", *vc_index)

        todo!()
    }

    fn build_extrinsic_revoke_vc(&self, vc_index: &H256) -> Self::Extrinsic<VCRevokeFn> {
        // let xt: UncheckedExtrinsicV4<_, _, _, _> =
        //     compose_extrinsic!(self.api, VC_PALLET_NAME, "revoke_vc", *vc_index);

        // xt

        todo!()
    }
}
