use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8bc283b38cb3e5f4")]
pub struct Oracle {
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
}
