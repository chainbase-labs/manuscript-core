use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}
