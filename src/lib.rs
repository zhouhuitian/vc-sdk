#![recursion_limit = "256"]
#![feature(string_remove_matches)]

pub mod api_client_patch;
pub mod direct_call;
pub mod identity_management;
pub mod primitives;

#[cfg(target_arch = "x86_64")]
pub mod ra;

pub mod sidechain;
pub mod utils;
pub mod vc_management;

use std::marker::PhantomData;

use codec::Encode;
use sidechain::rpc::SidechainRpcClient;
use sp_core::{crypto::AccountId32 as AccountId, sr25519};
use sp_runtime::MultiAddress;
use substrate_api_client::{
    ac_primitives::{
        Config, DefaultRuntimeConfig, ExtrinsicParams, ExtrinsicSigner, SignExtrinsic,
        UncheckedExtrinsicV4,
    },
    api::Result as ApiResult,
    rpc::JsonrpseeClient,
    Api, SubmitAndWatch, XtStatus,
};

pub type Address = MultiAddress<AccountId, ()>;

#[cfg(feature = "local")]
const NODE_URL: &str = "ws://127.0.0.1:9944";
#[cfg(feature = "local")]
const WORKER_URL: &str = "wss://localhost:2000";

#[cfg(feature = "staging")]
const NODE_URL: &str = "wss://tee-staging.litentry.io:443";
#[cfg(feature = "staging")]
const WORKER_URL: &str = "wss://tee-staging.litentry.io:2000";

#[cfg(feature = "prod2")]
const NODE_URL: &str = "wss://tee-internal.litentry.io:443";
#[cfg(feature = "prod2")]
const WORKER_URL: &str = "wss://tee-internal.litentry.io:2000";

// Default to `local` worker mode when no cargo features are set.
#[cfg(not(any(feature = "local", feature = "staging", feature = "prod2")))]
const NODE_URL: &str = "ws://127.0.0.1:9944";
#[cfg(not(any(feature = "local", feature = "staging", feature = "prod2")))]
const WORKER_URL: &str = "wss://localhost:2000";

pub struct ApiClient<T>
where
    T: Config,
{
    pub api: Api<DefaultRuntimeConfig, JsonrpseeClient>,
    pub sidechain: SidechainRpcClient,
    phantom: PhantomData<T>,
}

impl<T> ApiClient<T>
where
    T: Config,
{
    pub fn new_with_signer(signer: sr25519::Pair) -> ApiResult<Self> {
        let client = JsonrpseeClient::new(NODE_URL)?;
        let mut api = Api::<DefaultRuntimeConfig, JsonrpseeClient>::new(client)?;

        let signer = ExtrinsicSigner::new(signer);
        api.set_signer(signer);

        let sidechain = SidechainRpcClient::new(WORKER_URL);

        println!("[+] Parachain rpc : {}", NODE_URL);
        println!("[+] Sidechain rpc : {}", WORKER_URL);

        Ok(ApiClient {
            api,
            sidechain,
            phantom: PhantomData,
        })
    }

    pub fn get_signer(&self) -> Option<&T::AccountId> {
        // self.api.signer_account()
        todo!()
    }
}

pub trait SendExtrinsic {
    type Extrinsic<Call>;

    fn send_extrinsic<Call: Encode + Clone>(&self, extrinsic: Self::Extrinsic<Call>);
}

impl<T> SendExtrinsic for ApiClient<T>
where
    T: Config,
{
    type Extrinsic<Call> = UncheckedExtrinsicV4<
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::ExtrinsicAddress,
        Call,
        <T::ExtrinsicSigner as SignExtrinsic<T::AccountId>>::Signature,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::SignedExtra,
    >;

    fn send_extrinsic<Call: Encode + Clone>(&self, extrinsic: Self::Extrinsic<Call>) {
        match self
            .api
            .submit_and_watch_extrinsic_until(extrinsic, XtStatus::InBlock)
        {
            Ok(tx_hash) => println!(" ✅ Transaction got included. Hash: {:?}", tx_hash),
            Err(e) => println!(" ❌ Transaction error : {:?}", e),
        }
    }
}
