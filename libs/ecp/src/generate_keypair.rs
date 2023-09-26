//! How to generate a keypair using the k256 elliptic curve cryptography library. The program starts by importing the necessary modules,
//! which are `SigningKey` and `VerifyingKey` from the `ecdsa` module of the `k256` library, and `OsRng` from the `rand_core` library.
//!
//! The `main` function is then defined, which is the entry point of the program. Inside the `main` function, the first step is
//! to generate a private key using the `SigningKey::random` method, which takes a mutable reference to an instance of `OsRng`
//! as an argument. This method generates a random 32-byte private key using the operating system's random number generator.
//! The private key is then printed to the console using the `to_bytes` method of the `SigningKey` struct.
//!
//! Next, the public key is derived from the private key using the `VerifyingKey::from` method, which takes a reference
//! to the private key as an argument. The `to_sec1_bytes` method of the `VerifyingKey` struct is then used to convert
//! the public key to its compressed SEC1 representation, which is a 33-byte array. The compressed public key is then printed to the console.
//!
//! To improve the readability of the code, the comments could be expanded to provide more context on what each step is doing.
//! Additionally, the code could be split into separate functions for generating the private and public keys, which would
//! make it easier to reuse these functions in other parts of the program. Finally, the code could be optimized for
//! performance by using a more efficient random number generator, such as the `ThreadRng` generator from the `rand` crate, instead of `OsRng`.
//!
//! Example:
//! ```sh
//! $ cargo run
//! Private/Secret key: [43, 188, 69, 27, 157, 184, 226, 205, 66, 70, 201, 30, 4, 194, 84, 11, 238, 48, 188, 213, 132, 41, 214, 110, 242, 68, 141, 110, 129, 61, 133, 36]
//! Public key: [3, 242, 211, 81, 35, 243, 126, 29, 154, 73, 140, 164, 56, 233, 111, 50, 143, 31, 76, 6, 212, 20, 178, 36, 100, 45, 211, 94, 119, 219, 20, 113, 132]
//! ```

use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;

pub fn main() {
    // 1a. private key from random no. generator of the OS
    let secret_key = SigningKey::random(&mut OsRng);
    // 32 bytes
    println!(
        "Private/Secret/Signing key (in bytes array): {:?}",
        secret_key.to_bytes()
    );
    println!(
        "Private/Secret/Signing key (in hex string): {:?}",
        hex::encode(secret_key.to_bytes())
    );

    // 1b. public key from private key
    let public_key = VerifyingKey::from(&secret_key);
    // 33 bytes
    println!(
        "Public/Verifying key (in bytes array): {:?}",
        public_key.to_sec1_bytes()
    );
    println!(
        "Public key/Verifying (in hex string): {:?}",
        hex::encode(public_key.to_sec1_bytes())
    );
}
