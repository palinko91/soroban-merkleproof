## Soroban Merkleproof
In this crate I'm aiming to create a Soroban version of the Merkleproof contract, and also provide a whitelist smart contract template later. Right now the main focus are on to make something similar than this on EVM chains:
https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol

## Environment Setup
The best and easiest would be if you following this:
https://soroban.stellar.org/docs/getting-started/setup

## Merkle Tree Generator
At the path `src/bin/merkletree_generator.rs` you can find a simple example of a Markle Tree generation. The root is important we going to need that later in the whitelist contract. It's important to note the `wallet_addresses_to_whitelist` should have a vector of 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024...etc long so numbers power of 2 works. In order to run this file type:

    cargo run --bin merkletree_generator

## Building the library
To build the library into Soroban smart contract use the command:

    cargo build --target wasm32-unknown-unknown --release

After building can find the wasm file at:

    ./target/wasm32-unknown-unknown/release/soroban_merkleproof.wasm