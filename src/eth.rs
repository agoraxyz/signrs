use ethers::core::k256::ecdsa::SigningKey as SecpSk;
use ethers::signers::{LocalWallet, Signer as SignerT};
pub use ethers::utils::hash_message as hash_eth_message;

pub struct EthSigner(LocalWallet);

impl EthSigner {
    pub fn new(seed: &[u8]) -> Self {
        let signing_key = SecpSk::from_bytes(seed.into()).unwrap();
        Self(LocalWallet::from(signing_key))
    }

    pub fn address(&self) -> [u8; 20] {
        self.0.address().to_fixed_bytes()
    }

    pub fn sign(&self, msg: &[u8]) -> [u8; 65] {
        futures::executor::block_on(async move {
            self.0
                .sign_message(msg)
                .await
                .unwrap()
                .to_vec()
                .try_into()
                .unwrap()
        })
    }
}
