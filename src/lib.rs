#![no_std]
use tiny_keccak::{Hasher, Keccak};
use soroban_sdk::{assert_with_error, contracterror, contractimpl, vec, Bytes, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidMultiProof = 1,
}

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
        let mut i = 0;
        while i < proof.len() {
            computedhash = Self::hashpair(env.clone(), computedhash, proof[i]);
            i += 1;
        }
        return computedhash;
    }

    /// Returns true if the `leaves` can be simultaneously proven to be a part of a merkle tree defined by
    /// `root`, according to `proof` and `proofFlags` as described in {processMultiProof}.
    ///
    /// CAUTION: Not all merkle trees admit multiproofs. See {processMultiProof} for details.

    pub fn multiprfvr(
        env: Env,
        proof: Vec<Bytes>,
        proofflags: Vec<bool>,
        root: Bytes,
        leaves: Vec<Bytes>,
    ) -> bool {
        return Self::prcsmp(env, proof, proofflags, leaves) == root;
    }

    /// Returns the root of a tree reconstructed from `leaves` and sibling nodes in `proof`. The reconstruction
    /// proceeds by incrementally reconstructing all inner nodes by combining a leaf/inner node with either another
    /// leaf/inner node or a proof sibling node, depending on whether each `proofFlags` item is true or false
    /// respectively.
    ///
    /// CAUTION: Not all merkle trees admit multiproofs. To use multiproofs, it is sufficient to ensure that:
    /// 1) the tree is complete (but not necessarily perfect),
    /// 2) the leaves to be proven are in the opposite order they are in the tree (i.e., as seen from right to
    /// left starting at the deepest layer and continuing at the next layer).

    pub fn prcsmp(env: Env, proof: Vec<Bytes>, proofflags: Vec<bool>, leaves: Vec<Bytes>) -> Bytes {
        // This function rebuild the root hash by traversing the tree up from the leaves. The root is rebuilt by
        // consuming and producing values on a queue. The queue starts with the `leaves` array, then goes onto the
        // `hashes` array. At the end of the process, the last hash in the `hashes` array should contain the root of
        // the merkle tree.
        let leaves_len: u32 = leaves.len();
        let total_hash: u32 = proofflags.len();

        // Check proof validity.
        assert_with_error!(
            &env,
            leaves_len + proof.len() - 1 == total_hash,
            Error::InvalidMultiProof
        );

        // The xxxPos values are "pointers" to the next value to consume in each array. All accesses are done using
        // `xxx[xxxPos++]`, which return the current value and increment the pointer, thus mimicking a queue's "pop".
        let mut hashes: Vec<Bytes> = vec![&env, Bytes::from_slice(&env, &[total_hash as u8])]; 
        let mut leaf_pos: u32 = 0;
        let mut hash_pos: u32 = 0;
        let mut proof_pos: u32 = 0;
        // At each step, we compute the next hash using two values:
        // - a value from the "main queue". If not all leaves have been consumed, we get the next leaf, otherwise we
        //   get the next hash.
        // - depending on the flag, either another value for the "main queue" (merging branches) or an element from the
        //   `proof` array.
        let mut i = 0;
        while i < total_hash {
            let a: Bytes = if leaf_pos < leaves_len {
                leaves[leaf_pos += 1]
            } else {
                hashes[hash_pos += 1]
            };

            let b: Bytes = if proofflags[i] == true && leaf_pos < leaves_len {
                leaves[leaf_pos += 1]
            } else if proofflags[i] == true && leaf_pos >= leaves_len {
                hashes[hash_pos += 1]
            } else {
                proof[proof_pos += 1]
            };

            hashes.insert(i, Self::hashpair(env.clone(), a, b));
            i += 1;
        }

        if total_hash > 0 {
            return hashes[total_hash - 1];
        } else if leaves_len > 0 {
            return leaves[0];
        } else {
            return proof[0];
        }
    }

    fn hashpair(env: Env, a: Bytes, b: Bytes) -> Bytes {
        match a < b {
            true => return Self::effhash(env, a, b),
            false => return Self::effhash(env, b, a),
        }
    }

    fn effhash(env: Env, a: Bytes, b: Bytes) -> Bytes {
        let mut k256 = Keccak::v256();
        let mut byte_arr = [0u8;32];
        let mut a_slice = [0u8;32];
        let mut b_slice = [0u8;32];

        a.copy_into_slice(&mut a_slice);
        b.copy_into_slice(&mut b_slice);
        k256.update(&a_slice);
        k256.update(&b_slice);
        
        k256.finalize(&mut byte_arr);
        let mut res = Bytes::from_array(&env, &byte_arr);
        return res;
    }
}