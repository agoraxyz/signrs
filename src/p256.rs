use crate::Signer;
use p256::ecdsa::{SigningKey as NistSk, VerifyingKey as NistPk};

pub struct NistSigner(NistSk);

impl Signer for NistSigner {
    type Address = ();
    type Pubkey = NistPk;
    type Signature = [u8; 65];
    fn new(seed: &[u8]) -> Self {
        let signing_key = NistSk::from_slice(seed).expect("32 bytes");
        Self(signing_key)
    }

    fn address(&self) -> Self::Address {}

    fn pubkey(&self) -> Self::Pubkey {
        NistPk::from(&self.0)
    }

    fn sign(&self, prehash: &[u8]) -> Self::Signature {
        let (sig, recid) = self.0.sign_prehash_recoverable(prehash).unwrap();
        let mut sig_bytes = [0u8; 65];
        sig_bytes[0..64].copy_from_slice(&sig.to_bytes());
        sig_bytes[64] = recid.to_byte();
        sig_bytes
    }
}
