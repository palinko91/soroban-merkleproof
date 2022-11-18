use merkletreers::merkletree::tree::MerkleTree;
use soroban_sdk::Env;
mod merkletree_soroban_types;

fn main() {
    // The wallets should be whitelisted. The list should contain power of 2 element, so it can be 2,4,8,16,32,64,128,256,512,1024...etc long
    let wallet_addresses_to_whitelist = vec![
        "GBADIKRAYTCM5TL34XIFBY6JN5JDDU23OC6TCG2HJIYVNJMCY5WQARPI",
        "GB2NRKJVDMXKJQA7JMQ27FB2HX6CSLCKRFQQVPNXK6QEDOMN2H2N6PSH",
        "GDIKL43MU7AM2MVJNDD63MAKSCOFGCEGRP4CXKXHTV6WR2FFRJHCNWH2",
        "GCUL2IG2KKV6M65Z5ZBW6GD5SQZGBMNI5TKQP5UM6XPRHQM7VETL3WVX",
        "GBEL4SDQL4NFDSKLZESRESQCKTCWRBGDROAMFRR4BTQVBC72JN7TVZOM",
        "GBLFJ7MQCE4CF7WYNQBY37FYS2LDUM3GDYZLQN26HAFAH5QZ5CO2TEQI",
        "GAMBM4OBANJ4DTY3LO4MRNFTVRVJLHROCZ7ZXXMDUUH2PDUMRYSL3VXN",
        "GDTA4ADTTPHM42ORRW6J533V5WNABUPNZCARMM7PUJ7GLBLK7FO57CBY",
        "GA3FJNSI5YUFBGADYYZGYPOH5RW7LI74GPXVJ7QCBRRKSPCPOOMHEPVZ",
        "GDWQKDEUUUSJRKHOHKHZDYIXCMCKYZT7Q74M7GM3EJJLMSHXD4IFZWZA",
        "GAYD3HMA5X6E3PDODZEXJL7HVGTARVYKTR7LNY63E5XNY4YNSHYKE5GO",
        "GCLLWN6TAPKLPCBMHGWPFE2BFKUGMYDTUHLDPHDTCLKGZQWRPWNM7K23",
        "GCBKRHRFLCJXLDQUCCNLQWCQ22TUWN6OYU4NU362FFX3VFWXNK7IBZVM",
        "GAC2TCTSYOHU6W6GT2KRZE2VSWIIVRU7DLTQI7TR22P5CE4Z5D7IPD5Z",
        "GAEV2UDWTA7A6RAQFFOFIFKGUHN2JUNHVMMJVMSK4FYDTXZLCO43QNZX",
        "GDL4OY7PC4FSH22XV3TEDUBVP4B3DUO3K2CWY4I45P2M5LDYJV3V5H6G",
        "GBGPJKREV6MDO53QMKOOUSSRZOGRKI5F26FYFL7JYDGFQ3SM37EQKWBQ",
        "GCD4OH3HXXM3CBXRDA7ALULI5QETLSF4NGZUKUYKG4CK3CGTJ24GF7XZ",
        "GCBXVKCGMMIM5PYYC65Z4K42AFFPDHTCNBKMSO23SEM65KIU2BNEKJVX",
        "GDPLYZX4JL6LQT6YMJK6S56YRQ74EIYS7AEVUT5Y3VVZGQJBXZQKK7BT",
        "GBEI2C2Y5UCKA5SLO62FQIQ6BX3G3W5H7FD3PGIW5F72B2VZCZUHD5NO",
        "GD532HCAZMSJVJHNCVYY2ML65IS42NQ5Q4E7OBKS3SB25B5DID3YYGR3",
        "GCBIVJBAJDLJ2UO7ZT2NYVZWT2TV6P2TUB6RWXZUIKSX7EPTQA4EWHYU",
        "GDG2ITIOPGEKJNZSFNX3TAOS6YQZZ253SGS5CXLBOTSVJVJWSEVIWXW5",
        "GCN4QHQDR3UXYBUVMO33NUFUYA5WVFGID6CS3HLC2SQRDNYLGWRMPV43",
        "GBIPSW3DLEFWIPCXCZW3I3D5YZRB2PHO3GWC22TIWINIBUWFWN6F53FI",
        "GALI7NOQ2J7XH2B2TPWALQV66ARW2TDP52YUMZIEJE3D3DQD2QZ3Y7HI",
        "GDAAMAPO5KNGP4BT2M3FDO4HNZZ2DQLXVNAGM6VHDV2TWIWHF6OVFNCP",
        "GA52JNXWFJ3NUMFA5R75SYL4QWDYZRX2RYHLZKUGYZA7FTXWGT7GNMXZ",
        "GCGGCUDXW3YLVT5U267DZQHA4KTM6ZK5RYRHSRILZTN2BHT7AVHSJL63",
        "GDZCTYOFXC5O4CWOF6WD6XAZFQFUCNGBIO3AYQFIMCPICPXALR2RY2MU",
        "GC7NIXKDPNXY4WLJ7AMN5E3TQELZSKGITIYXA5LLX3I3VGID7SKVSE43",
    ];

    let tree = MerkleTree::new(wallet_addresses_to_whitelist.clone());
    let root_hash = &tree.root;
    let leafs = &tree.leafs;
    assert_eq!(
        &wallet_addresses_to_whitelist.len(),
        &leafs.len(),
        "The wallet address and the leaf nodeas amount not matching!"
    );

    println!("The whole tree: \n{:?}\n\n", &tree);
    println!("The root is: \n{:?}\n\n", &root_hash);

    // https://medium.com/@ItsCuzzo/using-merkle-trees-for-nft-whitelists-523b58ada3f9
    // Under Website Implementation part, but this time did in Rust
    let claiming_address = &wallet_addresses_to_whitelist[0];
    let proof = &tree.proof(claiming_address);
    println!("The proof is: \n{:?}\n\n\n", &proof);

    // Let's convert into Soroban Bytes and Vec<Bytes> types
    let env = Env::default();
    let tree_copy = MerkleTree::new(wallet_addresses_to_whitelist.clone());
    let root_bytes = merkletree_soroban_types::str_to_bytes(&env, &tree_copy.root);
    println!("Root in bytes: \n{:?}\n\n\n", root_bytes);
    let leaf_bytes = merkletree_soroban_types::str_to_bytes(&env, &tree_copy.leafs[0]);
    println!("Leaf in bytes: \n{:?}\n\n\n", leaf_bytes);
    let mut proof_array: Vec<&str> = Vec::new();
    let proof_len = proof.len();
    for i in 0..proof_len {
        let r = match &proof[i].r {
            None => "None",
            Some(string) => string,
        };

        let l = match &proof[i].l {
            None => "None",
            Some(string) => string,
        };

        if r != "None" {
            proof_array.push(r);
        }

        if l != "None" {
            proof_array.push(l);
        }
    }

    let proof_bytes = merkletree_soroban_types::str_arr_to_bytes(&env, &proof_array);
    println!("Proof in bytes vec: \n{:?}", proof_bytes);
}
