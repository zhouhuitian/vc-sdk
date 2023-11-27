use sp_core::{sr25519, Pair};
use substrate_api_client::{api_client, ac_primitives::DefaultRuntimeConfig};
use vc_sdk::{
    api_client_patch::parachain::ParachainPatch,
    direct_call::{
        primitives::TrustedGetter,
        top::{DirectCall, TrustedOperation},
        trusted_call_signed::TrustedCall,
        types::{AccountId, KeyPair},
    },
    utils::{
        crypto::{generate_user_shielding_key, to_user_shielding_key_type},
        di::decode_user_shielding_key,
    },
    ApiClient, primitives::assertion::Assertion,
};
use vc_sdk::utils::di::{decode_nonce};

#[test]
fn tc_decode_nonce_works() {
    // RpcReturnValue.value
    // 0x011002000000
    let encoded_nonce = "011002000000";
    let nonce = decode_nonce(encoded_nonce).unwrap();
    assert_eq!(nonce, 2);
}

#[test]
fn tc_di_set_user_shielding_key_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::<DefaultRuntimeConfig>::new_with_signer(alice.clone()).unwrap();
    let shard = api_client.get_shard().unwrap();

    let nonce = 0_u32;

    let top: TrustedOperation = TrustedCall::request_vc(
        alice.public().into(),
        alice.public().into(),
        Assertion::A1,
        Default::default(),
    )
    .sign(
        &KeyPair::Sr25519(Box::new(alice.clone())),
        nonce,
        &shard,
        &sp_core::H256::from(shard),
    )
    .into();
    let _ = api_client.send_request_di(&top);
}
