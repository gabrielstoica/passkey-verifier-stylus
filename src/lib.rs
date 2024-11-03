// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import odin-rs/secp256r1 lib as p256
use p256;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;

// Define some dummy persistent storage for future implementations.
sol_storage! {
    #[entrypoint]
    pub struct P256Verifier {
    }
}

/// Declare that `P256Verifier` is a contract with the following external methods.
#[public]
impl P256Verifier {
    /// Verifies the provided signature
    pub fn verify_p256_signature(
        &self,
        message: String,
        pub_x: String,
        pub_y: String,
        r: String,
        s: String,
    ) -> bool {
        let result = p256::verify_signature_secp256r1(&pub_x, &pub_y, &message, &r, &s);
        result
    }
}
