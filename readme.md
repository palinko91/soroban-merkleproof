## Soroban Merkleproof
In this crate I'm aiming to create a Soroban version of the Merkleproof contract, and also provide a whitelist smart contract template later. Right now the main focus are on to make something similar than this on EVM chains:
https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol

## Environment Setup
The best and easiest would be if you following this:
https://soroban.stellar.org/docs/getting-started/setup

## Merkle Tree Generator
At the path `src/bin/merkletree_generator.rs` you can find a simple example of a Markle Tree generation. The root is important we going to need that later in the whitelist contract. It's important to note the `wallet_addresses_to_whitelist` should have a vector of 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024...etc long so numbers power of 2 works. In order to run this file type:

    cargo run --bin merkletree_generator

This output gives us a lot of results what we can utilize later on, like the `Root in bytes` and `Proof in bytes` are the 2 most important. The first is hard coded into the `whitelist` example, the second we have to send during the contract call.
## Building the library
To build the library into Soroban smart contract use the command:

    cargo build --target wasm32-unknown-unknown --release

After building can find the wasm file at:

    ./target/wasm32-unknown-unknown/release/soroban_merkleproof.wasm

## Example whitelist
To see a simple example smart contract which utilizing the `soroban_merkleproof` change directory to:

    cd whitelist

We can also build like the previous:

    cargo build --target wasm32-unknown-unknown --release

After building can find the wasm file at:

    ./target/wasm32-unknown-unknown/release/whitelist.wasm

## In Soroban CLI
After built both contracts we can deploy it. First we have to deploy the merkkleproof contract:

    soroban deploy \
    --wasm src/soroban_merkleproof.wasm \
    --id a

Then we can deploy the whitelist contract also:

    soroban deploy \
    --wasm target/wasm32-unknown-unknown/release/whitelist.wasm \
    --id b

So we can invoke the contract function to test:

    soroban invoke \
            --id b \
            --account GBADIKRAYTCM5TL34XIFBY6JN5JDDU23OC6TCG2HJIYVNJMCY5WQARPI \
            --fn is_wled \
            --arg a \
            --arg [[227,20,180,34,202,220,222,70,162,168,25,24,156,109,64,246,59,199,142,119,215,6,144,242,90,128,109,189,145,169,198,18],[85,16,51,17,218,113,23,115,85,17,78,73,214,193,105,141,236,173,135,120,141,109,170,217,131,186,159,161,87,246,163,219]]

We should get true because this address is whitelisted and we sent the proof route for the contract to check with the merkletree :)
Let try 2 ways to fail. First is to change the proof argument:

    soroban invoke \
            --id b \
            --account GBADIKRAYTCM5TL34XIFBY6JN5JDDU23OC6TCG2HJIYVNJMCY5WQARPI \
            --fn is_wled \
            --arg a \
            --arg [[228,19,181,35,203,220,222,70,162,168,25,24,156,109,64,246,59,199,142,119,215,6,144,242,90,128,109,189,145,169,198,18],[85,16,51,17,218,113,23,115,85,17,78,73,214,193,105,141,236,173,135,120,141,109,170,217,131,186,159,161,87,246,163,219]]

We can try to change the address also:

    soroban invoke \
            --id b \
            --account GDQHNBKFCO666SPX4RS62VTDY7H5W2QXHVVVQCDTADTOI3IYZGEOZL6V \
            --fn is_wled \
            --arg a \
            --arg [[227,20,180,34,202,220,222,70,162,168,25,24,156,109,64,246,59,199,142,119,215,6,144,242,90,128,109,189,145,169,198,18],[85,16,51,17,218,113,23,115,85,17,78,73,214,193,105,141,236,173,135,120,141,109,170,217,131,186,159,161,87,246,163,219]]

Both should return is `false` which means we are not whitelisted. The correct proof should be generated by the website the contract call coming from.


