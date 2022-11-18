// #![no_std]
// use soroban_sdk::{contractimpl, Bytes, BytesN, Env, Vec};
// use tiny_keccak::{Hasher, Keccak};

// mod merkleproof {
//     soroban_sdk::contractimport!(
//         file = "./target/wasm32-unknown-unknown/release/soroban_merkleproof.wasm"
//     );
// }

// pub struct Whitelist;

// #[contractimpl]
// impl Whitelist {
//     pub fn is_wled(env: Env, contract_id: BytesN<32>, proof: Vec<Bytes>) -> bool {
//         let root: Bytes = Bytes::from_array(
//             &env,
//             &[
//                 98, 100, 54, 49, 52, 53, 57, 97, 100, 97, 102, 53, 50, 56, 54, 55, 48, 99, 54, 50,
//                 53, 100, 51, 49, 102, 102, 48, 51, 51, 102, 99, 102, 101, 54, 98, 55, 56, 57, 55,
//                 102, 57, 99, 52, 56, 99, 55, 100, 55, 52, 52, 53, 48, 56, 51, 55, 53, 98, 101, 98,
//                 49, 55, 49, 48, 51,
//             ],
//         );
//         let client = merkleproof::ContractClient::new(&env, contract_id);
//         let mut k256 = Keccak::v256();
//         let mut keccak_result = [0; 32];
//         let account = env.invoker().as_bytes();
//         k256.update(account.as_bytes());
//         k256.finalize(&mut keccak_result);
//         let leaf: Bytes = Bytes::from_array(&env, &keccak_result);
//         client.verify(proof, root, leaf)
//     }
// }
