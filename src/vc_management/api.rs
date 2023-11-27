use super::VcManagementApi;
use crate::{
    primitives::{assertion::Assertion, MrEnclave},
    vc_management::xtbuilder::VcManagementXtBuilder,
    ApiClient, SendExtrinsic,
};
use sp_core::H256;
use substrate_api_client::ac_primitives::Config;

impl<T: Config> VcManagementApi for ApiClient<T> {
    fn request_vc(&self, shard: &MrEnclave, assertion: &Assertion) {
        let xt = self.build_extrinsic_request_vc(shard, assertion);
        self.send_extrinsic(xt);
    }

    fn disable_vc(&self, vc_index: &H256) {
        let xt = self.build_extrinsic_disable_vc(vc_index);
        self.send_extrinsic(xt);
    }

    fn revoke_vc(&self, vc_index: &H256) {
        let xt = self.build_extrinsic_revoke_vc(vc_index);
        self.send_extrinsic(xt);
    }
}
