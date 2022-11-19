use tiny_keccak::{Hasher, Keccak};

fn hash(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut k256 = Keccak::v256();
    let mut res = [0u8; 32];

    k256.update(&a);
    k256.update(&b);
    k256.finalize(&mut res);

    return res;
}

pub fn verify(leaf: &str, proof: Vec<&str>, root: &str) -> bool {
    let leaf_b: [u8; 32] = hex::decode(leaf).unwrap().try_into().unwrap();
    let root_b: [u8; 32] = hex::decode(root).unwrap().try_into().unwrap();

    let mut proof_b: Vec<[u8; 32]> = Vec::new();
    for i in 0..proof.len() {
        let htob: [u8; 32] = hex::decode(proof[i]).unwrap().try_into().unwrap();
        proof_b.push(htob);
    }
    println!("Leaf in bytes: {:?}", &leaf_b);
    println!("Root in bytes: {:?}", &root_b);
    println!("Proof in bytes: {:?}", &proof_b);

    let mut computedhash = leaf_b;
    for proofelement in proof_b.into_iter() {
        if computedhash <= proofelement {
            // Hash(current computed hash + current element of the proof)
            computedhash = hash(computedhash, proofelement);
        } else {
            // Hash(current element of the proof + current computed hash)
            computedhash = hash(proofelement, computedhash);
        }
        println!("Computed hash bytes: {:?}", &computedhash);
    }

    computedhash == root_b
}

fn main() {
    // The wallets should be whitelisted. Since the cargos I found not generated me valid merkletree we are using merkletreejs to generate
    // https://replit.com/@palinko/MerkleTree#index.js
    // This should be handled by the website usually
    // https://medium.com/@ItsCuzzo/using-merkle-trees-for-nft-whitelists-523b58ada3f9
    let _wallet_addresses_to_whitelist = vec![
        "GBADIKRAYTCM5TL34XIFBY6JN5JDDU23OC6TCG2HJIYVNJMCY5WQARPI",
        "GB2NRKJVDMXKJQA7JMQ27FB2HX6CSLCKRFQQVPNXK6QEDOMN2H2N6PSH",
        "GDIKL43MU7AM2MVJNDD63MAKSCOFGCEGRP4CXKXHTV6WR2FFRJHCNWH2",
        "GCUL2IG2KKV6M65Z5ZBW6GD5SQZGBMNI5TKQP5UM6XPRHQM7VETL3WVX",
    ];

    // Result
    // └─ 6285ee21931bf045649c3d733fe2dbde77f28dcd5ccf8c35c3b6194897322ce1
    //    ├─ 29616ec520fa85b9b8fb2036dde47a4d7391e20ff46407b27200912630eee127
    //    │  ├─ 6e23528f84bce52570b74ec5f1b73f3d16f985f9257885fb27a35d9c147452e2
    //    │  └─ e314b422cadcde46a2a819189c6d40f63bc78e77d70690f25a806dbd91a9c612
    //    └─ 55103311da71177355114e49d6c1698decad87788d6daad983ba9fa157f6a3db
    //       ├─ 42b53059a452803a7a22f368fb74b45ec77f6453573587b4bcfbdfeb65534357
    //       └─ 1e6fbd9f127151e0b4ea1c1c105f71f96dd6500dca258b0c990ed60e7fef4c53

    let root = "6285ee21931bf045649c3d733fe2dbde77f28dcd5ccf8c35c3b6194897322ce1";
    let leaf = "6e23528f84bce52570b74ec5f1b73f3d16f985f9257885fb27a35d9c147452e2";
    let proof = vec![
        "e314b422cadcde46a2a819189c6d40f63bc78e77d70690f25a806dbd91a9c612",
        "55103311da71177355114e49d6c1698decad87788d6daad983ba9fa157f6a3db",
    ];

    let verify_result = verify(leaf, proof, root);
    println!(
        "Verification result (expected: true) = {:?}",
        &verify_result
    );
}
