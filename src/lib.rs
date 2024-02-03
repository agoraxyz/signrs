#![deny(clippy::all)]
#![deny(clippy::dbg_macro)]
#![deny(unused_crate_dependencies)]

#[cfg(feature = "eth")]
pub mod eth;
#[cfg(feature = "secp")]
pub mod k256;
#[cfg(feature = "nist")]
pub mod p256;

pub trait Signer {
    type Address;
    type Pubkey;
    type Signature;
    fn new(seed: &[u8]) -> Self
    where
        Self: Sized;
    fn address(&self) -> Self::Address;
    fn pubkey(&self) -> Self::Pubkey;
    fn sign(&self, msg: &[u8]) -> Self::Signature;
}
