use codec::Encode;
use sp_core::sr25519::Signature;
use sp_runtime::{MultiAddress, AccountId32, MultiSignature};
use substrate_api_client::{
    ac_compose_macros::compose_extrinsic,
    ac_primitives::{
        extrinsics::{CallIndex, UncheckedExtrinsicV4},
        Config, ExtrinsicParams, SignExtrinsic, GenericSignedExtra, PlainTip,
    },
    extrinsic::utility::Batch,
};

use crate::{ApiClient, primitives::address::Address32};
pub type UtilityBatchAllFn<Call> = (CallIndex, Batch<Call>);
pub type UtilityBatchAllXt<Call, SignedExtra> = UncheckedExtrinsicV4<Address32, Call, MultiSignature, SignedExtra>;

#[maybe_async::maybe_async(?Send)]
pub trait BatchAllPatch {
    // type Extrinsic<Call>;

    async fn batch_all<Call: Encode + Clone>(
        &self,
        calls: &[Call],
    ) -> UncheckedExtrinsicV4<sp_runtime::MultiAddress<sp_runtime::AccountId32, u32>, ([u8; 2], Box<IdentityInfo<_>>), sp_runtime::MultiSignature, _>;
}

const UTILITY_MODULE: &str = "Utility";
const UTILITY_BATCH_ALL: &str = "batch_all";

#[maybe_async::maybe_async(?Send)]
impl<T> BatchAllPatch for ApiClient<T>
where
    T: Config,
{
    // type Extrinsic<Call> = UncheckedExtrinsicV4<
    //     MultiAddress<AccountId32, u32>,
    //     Call,
    //     Signature,
	// 	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::SignedExtra,
    // >;

    async fn batch_all<Call: Encode + Clone>(
        &self,
        calls: &[Call],
    ) -> UncheckedExtrinsicV4<sp_runtime::MultiAddress<sp_runtime::AccountId32, u32>, ([u8; 2], Box<IdentityInfo<_>>), sp_runtime::MultiSignature, _> {
        let calls = Batch {
            calls: calls.to_vec(),
        };
        compose_extrinsic!(self.api, calls, UTILITY_MODULE, UTILITY_BATCH_ALL)
    }
}
