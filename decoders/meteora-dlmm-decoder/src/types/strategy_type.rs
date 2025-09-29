use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum StrategyType {
    SpotOneSide,
    CurveOneSide,
    BidAskOneSide,
    SpotBalanced,
    CurveBalanced,
    BidAskBalanced,
    SpotImBalanced,
    CurveImBalanced,
    BidAskImBalanced,
}
