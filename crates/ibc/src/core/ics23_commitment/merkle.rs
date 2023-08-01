//! Merkle proof utilities

use crate::prelude::*;

use ibc_proto::ibc::core::commitment::v1::MerklePath;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleRoot;

use crate::core::ics23_commitment::commitment::{CommitmentPrefix, CommitmentRoot};
use crate::core::ics23_commitment::error::CommitmentError;
use crate::core::ics23_commitment::specs::ProofSpecs;

pub fn apply_prefix(prefix: &CommitmentPrefix, mut path: Vec<String>) -> MerklePath {
    let mut key_path: Vec<String> = vec![format!("{prefix:?}")];
    key_path.append(&mut path);
    MerklePath { key_path }
}

impl From<CommitmentRoot> for MerkleRoot {
    fn from(root: CommitmentRoot) -> Self {
        Self {
            hash: root.into_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MerkleProof {
}

/// Convert to ics23::CommitmentProof
/// The encoding and decoding shouldn't fail since ics23::CommitmentProof and ibc_proto::ics23::CommitmentProof should be the same
/// Ref. <https://github.com/informalsystems/ibc-rs/issues/853>
impl From<RawMerkleProof> for MerkleProof {
    fn from(_proof: RawMerkleProof) -> Self {
        panic!()
    }
}

impl From<MerkleProof> for RawMerkleProof {
    fn from(_proof: MerkleProof) -> Self {
        panic!()
    }
}

impl MerkleProof {
    pub fn verify_membership(
        &self,
        _specs: &ProofSpecs,
        _root: MerkleRoot,
        _keys: MerklePath,
        _value: Vec<u8>,
        _start_index: usize,
    ) -> Result<(), CommitmentError> {
        panic!()
    }

    pub fn verify_non_membership(
        &self,
        _specs: &ProofSpecs,
        _root: MerkleRoot,
        _keys: MerklePath,
    ) -> Result<(), CommitmentError> {
        panic!()
    }
}