mod entries;
mod node_extensions;
mod storage_layout;

use std::fmt;

use sha3::{Digest, Keccak256};

pub use self::entries::{
    AbiConstructor, AbiEntries, AbiEntry, AbiError, AbiEvent, AbiFallback, AbiFunction,
    AbiMutability, AbiParameter, AbiParameters, AbiReceive, ContractAbi,
};
pub(crate) use self::storage_layout::storage_type_identifier;
pub use self::storage_layout::{
    StorageLayout, StoragePosition, StorageSize, StorageType, StorageTypeKind, StorageVariable,
};

pub fn hash_from_signature(signature: &str) -> [u8; 32] {
    Keccak256::digest(signature).into()
}

/// Keccak-256 over a signature written piece by piece, so a selector needs no signature string.
#[derive(Default)]
pub(crate) struct SignatureHasher(Keccak256);

impl fmt::Write for SignatureHasher {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.update(s.as_bytes());
        Ok(())
    }
}

impl SignatureHasher {
    pub(crate) fn selector(self) -> u32 {
        let hash: [u8; 32] = self.0.finalize().into();
        u32::from_be_bytes(hash[0..4].try_into().unwrap())
    }
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = SignatureHasher::default();
    hasher.0.update(signature.as_bytes());
    hasher.selector()
}
