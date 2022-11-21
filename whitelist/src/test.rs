#[cfg(test)]
use crate::{merkleproof,Whitelist, WhitelistClient};
use soroban_sdk::{symbol, vec, Env, Vec, Bytes, BytesN};
use soroban_sdk::testutils::Accounts;



#[test]
fn test() {
    let env = Env::default();
    let contract_merkle_id = env.register_contract_wasm(None, merkleproof::WASM);
    let contract_id = env.register_contract(None, Whitelist);
    let client = WhitelistClient::new(&env, &contract_id);
    let proof: Vec<Bytes> = vec![
    &env,
    Bytes::from_array(
        &env,
        &[
            227, 20, 180, 34, 202, 220, 222, 70, 162, 168, 25, 24, 156, 109, 64, 246, 59, 199,
            142, 119, 215, 6, 144, 242, 90, 128, 109, 189, 145, 169, 198, 18,
        ],
    ),
    Bytes::from_array(
        &env,
        &[
            85, 16, 51, 17, 218, 113, 23, 115, 85, 17, 78, 73, 214, 193, 105, 141, 236, 173,
            135, 120, 141, 109, 170, 217, 131, 186, 159, 161, 87, 246, 163, 219,
        ],
    ),
    ];

    let user_no_wl = env.accounts().generate();

    assert_eq!(client.with_source_account(&user_no_wl).is_wled(&contract_merkle_id, &proof), false); 
}
