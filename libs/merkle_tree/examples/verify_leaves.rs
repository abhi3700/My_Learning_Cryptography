//! Verify leaves (single/multiple) via merkle proof
//!
//! NOTE: It is observed that the proof size increases as the no. of leaves increase.

use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // a vec of leaves stored in DB.
    let leaf_values = ["a", "b", "c", "d", "e", "f", "g"];
    println!("Leaves: {:?}", leaf_values.to_vec());

    // vec of 32 bytes i.e. effectively each bytes is a Hash
    // which is represented as bytes array like [128, 56, 89, ..]
    let leaves_bytes: Vec<[u8; 32]> =
        leaf_values.iter().map(|x| Sha256::hash(x.as_bytes())).collect();
    dbg!("Hashes (in bytes) of leaves: {:#?}", &leaves_bytes);
    let leaves_hashes: Vec<String> =
        leaves_bytes.iter().map(|x| format!("0x{}", hex::encode(x))).collect();
    println!("Hashes of leaves: {:#?}", leaves_hashes);

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves_bytes);
    let merkle_root_bytes = merkle_tree.root().ok_or("couldn't get the merkle root")?;
    println!("Merkle root hash: 0x{}", merkle_tree.root_hex().unwrap());

    let indices_to_prove = vec![3, 4];
    let leaves_to_prove = leaves_bytes.get(3..5).ok_or("can't get leaves to prove")?;
    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    // Serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();
    println!("Proof size: {:?} bytes", proof_bytes.len());
    // This has to be parsed in contract's function after generating this offline
    // (via cloud based setup)
    let proof_hashes = merkle_proof.proof_hashes_hex();
    println!("Proof array (in hashes): {:?}", proof_hashes);
    println!("Proof array len: {:?}", proof_hashes.len());

    // Parse proof back on the client
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes)?;

    assert!(proof.verify(
        merkle_root_bytes,
        &indices_to_prove,
        leaves_to_prove,
        leaves_bytes.len()
    ));

    Ok(())
}
