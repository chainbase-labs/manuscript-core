use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapInstructionBaseOut {
    pub max_amount_in: u64,
    pub amount_out: u64,
}
