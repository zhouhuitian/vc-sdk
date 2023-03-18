use super::VC_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
    ApiClient,
};
use codec::Decode;
use sp_core::Pair;
use sp_core::H256;
use sp_runtime::{MultiSignature, MultiSigner};
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

/// VCIssuedEvent
#[derive(Decode, Debug)]
pub struct VCIssuedEvent {
    pub account: AccountId,
    pub vc_index: H256,
    pub vc: AesOutput,
}

impl StaticEvent for VCIssuedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCIssued";
}

pub trait VcManagementEventApi {
    fn wait_event_vc_issued(&self) -> VCIssuedEvent;
}

impl<P> VcManagementEventApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_event_vc_issued(&self) -> VCIssuedEvent {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_issued_event: ApiResult<VCIssuedEvent> = self.api.wait_for_event(&events_out);
        vc_issued_event.unwrap()
    }
}