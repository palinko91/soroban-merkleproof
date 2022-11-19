#![cfg(test)]

use super::*;
extern crate std;

use soroban_sdk::{testutils::Accounts, vec, Bytes, Env, Vec};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MerkleProof);
    let client = MerkleProofClient::new(&env, &contract_id);

    // These values coming from merkletree_generator.rs
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

    let root: Bytes = Bytes::from_array(
        &env,
        &[
            98, 133, 238, 33, 147, 27, 240, 69, 100, 156, 61, 115, 63, 226, 219, 222, 119, 242,
            141, 205, 92, 207, 140, 53, 195, 182, 25, 72, 151, 50, 44, 225,
        ],
    );

    // Leaf generated from Account ID from the website, in our case merkletree_generator.rs solving that
    // Whitelisted
    let leaf_true: Bytes = Bytes::from_array(
        &env,
        &[
            110, 35, 82, 143, 132, 188, 229, 37, 112, 183, 78, 197, 241, 183, 63, 61, 22, 249, 133,
            249, 37, 120, 133, 251, 39, 163, 93, 156, 20, 116, 82, 226,
        ],
    );
    // Not whitelisted
    let leaf_false: Bytes = Bytes::from_array(
        &env,
        &[
            111, 34, 83, 142, 133, 187, 228, 36, 113, 184, 79, 198, 240, 183, 63, 61, 22, 249, 133,
            249, 37, 120, 133, 251, 39, 163, 93, 156, 20, 116, 82, 226,
        ],
    );

    let user = env.accounts().generate();

    assert_eq!(
        client
            .with_source_account(&user)
            .verify(&proof, &root, &leaf_true),
        true
    );
    assert_eq!(
        client
            .with_source_account(&user)
            .verify(&proof, &root, &leaf_false),
        false
    );
}
