use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ReceiveMessageParams {
    pub message: Vec<u8>,
    pub attestation: Vec<u8>,
}
