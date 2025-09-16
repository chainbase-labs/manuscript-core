use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Dex {
    SplTokenSwap,
    StableSwap,
    Whirlpool,
    MeteoraDynamicpool,
    RaydiumSwap,
    RaydiumStableSwap,
    RaydiumClmmSwap,
    AldrinExchangeV1,
    AldrinExchangeV2,
    LifinityV1,
    LifinityV2,
    RaydiumClmmSwapV2,
    FluxBeam,
    MeteoraDlmm,
    RaydiumCpmmSwap,
    OpenBookV2,
    WhirlpoolV2,
    Phoenix,
    ObricV2,
    SanctumAddLiq,
    SanctumRemoveLiq,
    SanctumNonWsolSwap,
    SanctumWsolSwap,
}
