//! Merkle tree + inclusion proofs (RFC 6962-style via `rs_merkle`).

use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleTree};

/// Build a Merkle tree from a slice of leaf hashes.
pub fn build_tree(leaves: &[[u8; 32]]) -> MerkleTree<Sha256> {
    MerkleTree::<Sha256>::from_leaves(leaves)
}

/// Compute the Merkle root. Returns the canonical empty-tree marker
/// (all-zero 32 bytes) if the tree is empty.
pub fn root(tree: &MerkleTree<Sha256>) -> [u8; 32] {
    tree.root().unwrap_or([0u8; 32])
}

/// Serializable inclusion proof for one leaf.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofData {
    pub leaf_index: usize,
    pub leaf_count: usize,
    /// Hex-encoded sibling hashes, ordered as required by RFC 6962.
    pub siblings: Vec<String>,
}

pub fn inclusion_proof(tree: &MerkleTree<Sha256>, leaf_index: usize) -> ProofData {
    let proof = tree.proof(&[leaf_index]);
    ProofData {
        leaf_index,
        leaf_count: tree.leaves_len(),
        siblings: proof.proof_hashes().iter().map(hex::encode).collect(),
    }
}

pub fn verify_inclusion(proof: &ProofData, leaf: [u8; 32], root: [u8; 32]) -> bool {
    let siblings: Result<Vec<[u8; 32]>, _> = proof
        .siblings
        .iter()
        .map(|hex_str| {
            let bytes = hex::decode(hex_str).map_err(|e| e.to_string())?;
            let arr: [u8; 32] = bytes
                .try_into()
                .map_err(|_| "sibling not 32 bytes".to_string())?;
            Ok::<_, String>(arr)
        })
        .collect();
    let Ok(siblings) = siblings else { return false };

    rs_merkle::MerkleProof::<Sha256>::new(siblings).verify(
        root,
        &[proof.leaf_index],
        &[leaf],
        proof.leaf_count,
    )
}

/// Convenience: compute the hash of an arbitrary byte sequence with the
/// same algorithm the tree uses.
pub fn hash(bytes: &[u8]) -> [u8; 32] {
    Sha256::hash(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaves(n: u32) -> Vec<[u8; 32]> {
        (0..n)
            .map(|i| {
                let mut buf = [0u8; 32];
                buf[..4].copy_from_slice(&i.to_be_bytes());
                buf
            })
            .collect()
    }

    #[test]
    fn root_deterministic_same_order() {
        let l = leaves(10);
        assert_eq!(root(&build_tree(&l)), root(&build_tree(&l)));
    }

    #[test]
    fn root_changes_with_order() {
        let mut l1 = leaves(10);
        let mut l2 = l1.clone();
        l2.swap(0, 5);
        assert_ne!(root(&build_tree(&l1)), root(&build_tree(&l2)));
        l1.swap(0, 5);
        assert_eq!(root(&build_tree(&l1)), root(&build_tree(&l2)));
    }

    #[test]
    fn every_leaf_has_valid_proof() {
        let l = leaves(20);
        let tree = build_tree(&l);
        let r = root(&tree);
        for (i, leaf) in l.iter().enumerate() {
            let proof = inclusion_proof(&tree, i);
            assert!(
                verify_inclusion(&proof, *leaf, r),
                "proof for index {i} failed"
            );
        }
    }

    #[test]
    fn tampered_leaf_fails_verification() {
        let l = leaves(20);
        let tree = build_tree(&l);
        let r = root(&tree);
        let proof = inclusion_proof(&tree, 7);
        let mut tampered = l[7];
        tampered[0] ^= 0xff;
        assert!(!verify_inclusion(&proof, tampered, r));
    }
}
