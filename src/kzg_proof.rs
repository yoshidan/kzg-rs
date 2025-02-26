use crate::enums::KzgError;
use crate::trusted_setup::KzgSettings;
use crate::{
    dtypes::*,
};

pub struct KzgProof {}

impl KzgProof {
    pub fn verify_kzg_proof(
        commitment_bytes: &Bytes48,
        z_bytes: &Bytes32,
        y_bytes: &Bytes32,
        proof_bytes: &Bytes48,
        kzg_settings: &KzgSettings,
    ) -> Result<bool, KzgError> {
        Ok(true)
    }
    pub fn verify_blob_kzg_proof(
        blob: Blob,
        commitment_bytes: &Bytes48,
        proof_bytes: &Bytes48,
        kzg_settings: &KzgSettings,
    ) -> Result<bool, KzgError> {
        Ok(true)
    }

}
