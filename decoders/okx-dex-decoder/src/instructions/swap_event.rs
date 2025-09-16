use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d40c6cde8260871e2")]
pub struct SwapEvent {
    pub dex: Dex,
    pub amount_in: u64,
    pub amount_out: u64,
}
