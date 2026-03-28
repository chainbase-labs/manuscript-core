use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SignatureThresholdUpdated {
    pub old_signature_threshold: u32,
    pub new_signature_threshold: u32,
}
