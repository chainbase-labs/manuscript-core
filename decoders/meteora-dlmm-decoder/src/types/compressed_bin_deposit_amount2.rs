use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CompressedBinDepositAmount2 {
    pub bin_id: i32,
    pub amount: u32,
}
