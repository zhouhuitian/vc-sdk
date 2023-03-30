use sp_core::{sr25519, Pair};

use crate::{ApiClient, utils::{generate_user_shielding_key, hex_account_to_address32, print_passed}, identity_management::{IdentityManagementApi, events::IdentityManagementEventApi}, primitives::{SubstrateNetwork, Identity}};



fn tc_create_identity(data: &[u8]) {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();

    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate {
        network,
        address: address.clone(),
    };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    print_passed();
}
