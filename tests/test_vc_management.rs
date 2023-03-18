use litentry_test_suit::{
    identity_management::api::*,
    primitives::{Assertion, AssertionNetworks, Network, ParameterString},
    utils::verify_vc_schema,
    vc_management::{api::*, events::VcManagementEventApi, VcManagementXtBuilder},
    ApiClient, USER_AES256G_KEY,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    // Inputs
    let a1 = Assertion::A1;

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let a2 = Assertion::A2(guild_id.clone());

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
    let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
    let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());

    let balance = 10_u128;
    let a4 = Assertion::A4(balance);

    let a6 = Assertion::A6;

    let balance = 10_u128;
    let a7 = Assertion::A7(balance);

    let litentry = Network::try_from("litentry".as_bytes().to_vec()).unwrap();
    let mut networks = AssertionNetworks::with_bounded_capacity(1);
    networks.try_push(litentry).unwrap();
    let a8 = Assertion::A8(networks);

    let balance = 10_u128;
    let a10 = Assertion::A10(balance);

    let balance = 10_u128;
    let a11 = Assertion::A11(balance);

    let assertions = vec![a1, a2, a3, a4, a6, a7, a8, a10, a11];
    assertions.into_iter().for_each(|assertion| {
        api_client.request_vc(shard, assertion);

        // Wait event
        let event = api_client.wait_event_vc_issued();
        println!(" ✅ [VCRequest] VC Index : {:?}", event.vc_index);
    });
}

#[test]
pub fn tc_batch_all_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    let balance = 1_u128;
    let a4 = Assertion::A4(balance);
    let a7 = Assertion::A7(balance);
    let a10 = Assertion::A10(balance);
    let a11 = Assertion::A11(balance);

    let assertions = [a4, a7, a10, a11];
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        assertion_calls.push(
            api_client
                .build_extrinsic_request_vc(shard, assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.api.batch(assertion_calls).hex_encode());
}

#[test]
pub fn tc_verify_vc_schema() {
    let decrypt_vc = [
        123, 34, 64, 99, 111, 110, 116, 101, 120, 116, 34, 58, 91, 34, 104, 116, 116, 112, 115, 58,
        47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 49, 56, 47, 99, 114,
        101, 100, 101, 110, 116, 105, 97, 108, 115, 47, 118, 49, 34, 44, 34, 104, 116, 116, 112,
        115, 58, 47, 47, 119, 51, 105, 100, 46, 111, 114, 103, 47, 115, 101, 99, 117, 114, 105,
        116, 121, 47, 115, 117, 105, 116, 101, 115, 47, 101, 100, 50, 53, 53, 49, 57, 45, 50, 48,
        50, 48, 47, 118, 49, 34, 93, 44, 34, 105, 100, 34, 58, 34, 48, 120, 97, 97, 102, 102, 57,
        102, 101, 49, 99, 51, 102, 100, 48, 102, 55, 49, 98, 97, 50, 100, 48, 55, 52, 102, 57, 52,
        57, 54, 55, 51, 57, 98, 57, 49, 101, 54, 50, 51, 48, 98, 53, 97, 102, 49, 97, 100, 55, 56,
        99, 57, 49, 50, 57, 52, 51, 98, 102, 51, 51, 48, 57, 56, 100, 100, 34, 44, 34, 116, 121,
        112, 101, 34, 58, 91, 34, 86, 101, 114, 105, 102, 105, 97, 98, 108, 101, 67, 114, 101, 100,
        101, 110, 116, 105, 97, 108, 34, 93, 44, 34, 99, 114, 101, 100, 101, 110, 116, 105, 97,
        108, 83, 117, 98, 106, 101, 99, 116, 34, 58, 123, 34, 105, 100, 34, 58, 34, 100, 52, 51,
        53, 57, 51, 99, 55, 49, 53, 102, 100, 100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48,
        52, 97, 57, 57, 102, 100, 54, 56, 50, 50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101,
        51, 57, 97, 53, 54, 56, 52, 101, 55, 97, 53, 54, 100, 97, 50, 55, 100, 34, 44, 34, 100,
        101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 73, 100, 101, 110, 116, 105,
        116, 121, 32, 76, 105, 110, 107, 101, 100, 32, 65, 110, 100, 32, 86, 101, 114, 105, 102,
        105, 101, 100, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 73, 100, 101, 110, 116, 105,
        116, 121, 76, 105, 110, 107, 101, 100, 86, 101, 114, 105, 102, 105, 101, 100, 34, 44, 34,
        116, 97, 103, 34, 58, 91, 34, 73, 68, 72, 117, 98, 34, 93, 44, 34, 97, 115, 115, 101, 114,
        116, 105, 111, 110, 115, 34, 58, 91, 123, 34, 97, 110, 100, 34, 58, 91, 123, 34, 115, 114,
        99, 34, 58, 34, 36, 104, 97, 115, 95, 119, 101, 98, 50, 95, 97, 99, 99, 111, 117, 110, 116,
        34, 44, 34, 111, 112, 34, 58, 34, 61, 61, 34, 44, 34, 100, 115, 116, 34, 58, 34, 116, 114,
        117, 101, 34, 125, 44, 123, 34, 115, 114, 99, 34, 58, 34, 36, 104, 97, 115, 95, 119, 101,
        98, 51, 95, 97, 99, 99, 111, 117, 110, 116, 34, 44, 34, 111, 112, 34, 58, 34, 61, 61, 34,
        44, 34, 100, 115, 116, 34, 58, 34, 116, 114, 117, 101, 34, 125, 93, 125, 93, 44, 34, 118,
        97, 108, 117, 101, 115, 34, 58, 91, 102, 97, 108, 115, 101, 93, 44, 34, 101, 110, 100, 112,
        111, 105, 110, 116, 34, 58, 34, 104, 116, 116, 112, 115, 58, 47, 47, 108, 105, 116, 101,
        110, 116, 114, 121, 46, 99, 111, 109, 47, 112, 97, 114, 97, 99, 104, 97, 105, 110, 47, 101,
        120, 116, 114, 105, 110, 115, 105, 99, 34, 125, 44, 34, 105, 115, 115, 117, 101, 114, 34,
        58, 123, 34, 105, 100, 34, 58, 34, 49, 53, 54, 56, 48, 49, 48, 56, 52, 56, 51, 51, 101,
        102, 53, 48, 51, 52, 53, 57, 56, 98, 57, 99, 48, 55, 100, 52, 100, 99, 57, 98, 56, 50, 97,
        54, 52, 53, 55, 100, 57, 98, 53, 49, 99, 101, 51, 99, 102, 48, 49, 56, 50, 54, 56, 99, 98,
        50, 55, 53, 52, 99, 56, 52, 34, 44, 34, 110, 97, 109, 101, 34, 58, 34, 76, 105, 116, 101,
        110, 116, 114, 121, 32, 84, 69, 69, 32, 87, 111, 114, 107, 101, 114, 34, 44, 34, 109, 114,
        101, 110, 99, 108, 97, 118, 101, 34, 58, 34, 68, 121, 57, 86, 84, 69, 101, 81, 114, 67, 77,
        81, 52, 72, 81, 110, 117, 52, 75, 118, 53, 85, 65, 70, 110, 72, 81, 54, 66, 101, 83, 50,
        116, 111, 49, 70, 105, 70, 114, 87, 87, 114, 49, 109, 34, 125, 44, 34, 105, 115, 115, 117,
        97, 110, 99, 101, 66, 108, 111, 99, 107, 78, 117, 109, 98, 101, 114, 34, 58, 49, 50, 55,
        44, 34, 112, 114, 111, 111, 102, 34, 58, 123, 34, 99, 114, 101, 97, 116, 101, 100, 66, 108,
        111, 99, 107, 78, 117, 109, 98, 101, 114, 34, 58, 49, 50, 55, 44, 34, 116, 121, 112, 101,
        34, 58, 34, 69, 100, 50, 53, 53, 49, 57, 83, 105, 103, 110, 97, 116, 117, 114, 101, 50, 48,
        50, 48, 34, 44, 34, 112, 114, 111, 111, 102, 80, 117, 114, 112, 111, 115, 101, 34, 58, 34,
        97, 115, 115, 101, 114, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 44, 34, 112,
        114, 111, 111, 102, 86, 97, 108, 117, 101, 34, 58, 34, 57, 98, 54, 100, 48, 97, 49, 99, 48,
        52, 97, 53, 52, 56, 51, 49, 101, 56, 50, 56, 99, 101, 53, 56, 57, 97, 52, 54, 55, 102, 48,
        97, 51, 99, 99, 53, 49, 54, 56, 48, 101, 49, 54, 99, 100, 97, 99, 57, 98, 56, 100, 102, 56,
        57, 51, 49, 50, 101, 99, 99, 57, 52, 98, 101, 101, 55, 50, 49, 101, 51, 48, 48, 97, 57, 48,
        97, 56, 55, 100, 53, 102, 102, 97, 97, 51, 53, 53, 97, 52, 50, 97, 50, 55, 97, 55, 49, 99,
        57, 49, 50, 101, 102, 97, 54, 49, 102, 57, 54, 54, 56, 48, 55, 56, 100, 48, 101, 51, 56,
        52, 99, 98, 98, 57, 100, 53, 55, 48, 52, 34, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97,
        116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 34, 49, 53, 54, 56, 48, 49, 48,
        56, 52, 56, 51, 51, 101, 102, 53, 48, 51, 52, 53, 57, 56, 98, 57, 99, 48, 55, 100, 52, 100,
        99, 57, 98, 56, 50, 97, 54, 52, 53, 55, 100, 57, 98, 53, 49, 99, 101, 51, 99, 102, 48, 49,
        56, 50, 54, 56, 99, 98, 50, 55, 53, 52, 99, 56, 52, 34, 125, 125,
    ];

    let is_valid = verify_vc_schema(&decrypt_vc);
    assert!(is_valid);

    println!(" ✅ [VCManagement] tc_verify_vc_schema passed 🚩.");
}
