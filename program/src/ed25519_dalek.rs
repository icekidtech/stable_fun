use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};

pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut rand::thread_rng())
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify_signature(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}