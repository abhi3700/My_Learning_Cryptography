//! Create Merkle tree from given leaf values
//!
//! The leaves are to be stored somewhere (in DB) so as to form the merkle proof
//!
//! Here, merkle proof is generated from leaves' indices.
//!
//! Output:
//! ```
//! Leaves: ["a", "b", "c", "d", "e", "f", "g"]
//! Hashes of leaves: [
//!     "0xca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
//!     "0x3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d",
//!     "0x2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6",
//!     "0x18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4",
//!     "0x3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea",
//!     "0x252f10c83610ebca1a059c0bae8255eba2f95be4d1d7bcfa89d7248a82d9f111",
//!     "0xcd0aa9856147b6c5b4ff2b7dfee5da20aa38253099ef1b4a64aced233c9afe29",
//! ]
//! Merkle root: 0xe2a80e0e872a6c6eaed37b4c1f220e1935004805585b5f99617e48e9c8fe4034
//! ```

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let leaf_values = ["a", "b", "c", "d", "e", "f", "g"];
    println!("Leaves: {:?}", leaf_values.to_vec());
    let leaves_bytes: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    dbg!("Hashes (in bytes) of leaves: {:?}", &leaves_bytes);
    let leaves_hashes: Vec<String> = leaves_bytes
        .iter()
        .map(|x| format!("0x{}", hex::encode(x)))
        .collect();
    println!("Hashes of leaves: {:#?}", leaves_hashes);

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves_bytes);
    println!("Merkle root hash: 0x{}", merkle_tree.root_hex().unwrap());

    Ok(())
}
