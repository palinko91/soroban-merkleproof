#![no_std]
use soroban_sdk::{contractimpl, Bytes, Env, Vec};
use tiny_keccak::{Hasher, Keccak};

pub struct MerkleProof;

#[contractimpl]
impl MerkleProof {
    /// Returns true if a `leaf` can be proved to be a part of a Merkle tree
    /// defined by `root`. For this, a `proof` must be provided, containing
    /// sibling hashes on the branch from the leaf to the root of the tree. Each
    /// pair of leaves and each pair of pre-images are assumed to be sorted.

    pub fn verify(env: Env, proof: Vec<Bytes>, root: Bytes, leaf: Bytes) -> bool {
        return Self::processprf(env, proof, leaf) == root;
    }

    /// Returns the rebuilt hash obtained by traversing a Merkle tree up
    /// from `leaf` using `proof`. A `proof` is valid if and only if the rebuilt
    /// hash matches the root of the tree. When processing the proof, the pairs
    /// of leafs & pre-images are assumed to be sorted.

    pub fn processprf(env: Env, proof: Vec<Bytes>, leaf: Bytes) -> Bytes {
        let mut computedhash: Bytes = leaf;
        for i in 0..proof.len() {
            computedhash =
                Self::hashpair(env.clone(), computedhash, proof.get_unchecked(i).unwrap());
        }
        return computedhash;
    }

    fn hashpair(env: Env, a: Bytes, b: Bytes) -> Bytes {
        match a < b {
            true => return Self::effhash(env, a, b),
            false => return Self::effhash(env, b, a),
        }
    }

    fn effhash(env: Env, a: Bytes, b: Bytes) -> Bytes {
        let mut k256 = Keccak::v256();
        let mut byte_arr = [0u8; 32];
        let mut a_slice = [0u8; 32];
        let mut b_slice = [0u8; 32];

        a.copy_into_slice(&mut a_slice);
        b.copy_into_slice(&mut b_slice);
        k256.update(&a_slice);
        k256.update(&b_slice);

        k256.finalize(&mut byte_arr);
        let res = Bytes::from_array(&env, &byte_arr);
        return res;
    }
}

mod test;
