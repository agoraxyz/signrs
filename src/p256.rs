use p256::ecdsa::{SigningKey as NistSk, VerifyingKey as NistPk};

pub struct NistSigner(NistSk);

impl NistSigner {
    pub fn new(bytes: &[u8]) -> Self {
        let signing_key = NistSk::from_slice(bytes).unwrap();
        Self(signing_key)
    }

    pub fn pubkey(&self) -> NistPk {
        NistPk::from(&self.0)
    }

    pub fn sign_prehashed(&self, prehash: &[u8]) -> [u8; 65] {
        let (sig, recid) = self.0.sign_prehash_recoverable(prehash).unwrap();
        let mut sig_bytes = [0u8; 65];
        sig_bytes[0..64].copy_from_slice(&sig.to_bytes());
        sig_bytes[64] = recid.to_byte();
        sig_bytes
    }
}
