//! How to verify a message with recovery id?
//!
//! Unlike Ethereum, there is no `v` component out of (r, s, v) for a digital signature.
//! And this is because we use the signing key to sign the message (in bytes), not the message digest.
//!
//! [msg] --hash--> [msg digest] --sign--> [signature, rec_id] --`recover_from_digest` method of `VerifyingKey`--> recovered_key
//! check if recovered_key == expected_key

use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha3::{Digest, Keccak256};

pub fn main() {
    // create a private/signing key
    // 32 bytes
    let signing_key = SigningKey::random(&mut OsRng);

    // === 1. Signing
    // message
    let msg = b"Book a Cab from Home to Airport at 10 am tomorrow";

    // message digest formed via hashing the original message bytes
    let msg_digest = Keccak256::new_with_prefix(msg);

    // get the signature & recovery id by signing the message digest
    let (signature, rec_id) = signing_key.sign_digest_recoverable(msg_digest).unwrap();
    //  64 bytes
    println!(
        "Signature: {} with components \nr: {},\ns: {} with recovery id \nv: {:?}",
        signature.to_string(),     // 64 bytes
        signature.r().to_string(), // 32 bytes
        signature.s().to_string(), // 32 bytes
        rec_id                     // 1 byte
    );

    // === 2. Verification
    // generate verification (public) key from the signature
    // 33 bytes
    let recovered_key =
        VerifyingKey::recover_from_digest(Keccak256::new_with_prefix(msg), &signature, rec_id)
            .unwrap();
    println!("Recovered verifying key: {:?}", recovered_key);

    // get the expected key from signing key
    let expected_key = VerifyingKey::from(&signing_key);
    println!("Expected verifying key: {:?}", expected_key.to_sec1_bytes());

    // assert
    assert_eq!(recovered_key, expected_key);
    println!("Successfully verified the signer!");
}
