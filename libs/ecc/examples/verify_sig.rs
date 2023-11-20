//! How to verify a message without recovery id?
//!
//! Unlike Ethereum, there is no `v` component out of (r, s, v) for a digital signature.
//! And this is because we use the signing key to sign the message (in bytes), not the message digest.
//!
//! [msg] --sign--> [signature] --`verify` method of `verifying_key`--> Verified (Yes/No)

use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand_core::OsRng;

pub fn main() {
    // create a private/signing key
    // 32 bytes
    let signing_key = SigningKey::random(&mut OsRng);

    // generate public/verification key from signing key
    // 33 bytes
    let verifying_key = VerifyingKey::from(&signing_key);

    // === 1. Signing
    let msg = b"Book a Cab from Home to Airport at 10 am tomorrow";
    let signature: Signature = signing_key.sign(msg);
    //  64 bytes
    println!(
        "Signature: {} with components \nr: {},\ns: {}",
        signature.to_string(),     // 64 bytes
        signature.r().to_string(), // 32 bytes
        signature.s().to_string(), // 32 bytes
    );

    // === 2. Verification
    assert!(verifying_key.verify(msg, &signature).is_ok());
    println!("Successfully verified the signer!");
}
