use crate::PROGRAM_ID;

use super::MeteoraDammV2Decoder;
pub mod add_liquidity;
pub mod claim_partner_fee;
pub mod claim_position_fee;
pub mod claim_protocol_fee;
pub mod claim_reward;
pub mod close_claim_fee_operator;
pub mod close_config;
pub mod close_position;
pub mod create_claim_fee_operator;
pub mod create_config;
pub mod create_dynamic_config;
pub mod create_position;
pub mod create_token_badge;
pub mod evt_add_liquidity_event;
pub mod evt_claim_partner_fee_event;
pub mod evt_claim_position_fee_event;
pub mod evt_claim_protocol_fee_event;
pub mod evt_claim_reward_event;
pub mod evt_close_claim_fee_operator_event;
pub mod evt_close_config_event;
pub mod evt_close_position_event;
pub mod evt_create_claim_fee_operator_event;
pub mod evt_create_config_event;
pub mod evt_create_dynamic_config_event;
pub mod evt_create_position_event;
pub mod evt_create_token_badge_event;
pub mod evt_fund_reward_event;
pub mod evt_initialize_pool_event;
pub mod evt_initialize_reward_event;
pub mod evt_lock_position_event;
pub mod evt_permanent_lock_position_event;
pub mod evt_remove_liquidity_event;
pub mod evt_set_pool_status_event;
pub mod evt_swap_event;
pub mod evt_update_reward_duration_event;
pub mod evt_update_reward_funder_event;
pub mod evt_withdraw_ineligible_reward_event;
pub mod fund_reward;
pub mod initialize_customizable_pool;
pub mod initialize_pool;
pub mod initialize_pool_with_dynamic_config;
pub mod initialize_reward;
pub mod lock_position;
pub mod permanent_lock_position;
pub mod refresh_vesting;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod set_pool_status;
pub mod swap;
pub mod update_reward_duration;
pub mod update_reward_funder;
pub mod withdraw_ineligible_reward;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MeteoraDammV2Instruction {
    AddLiquidity(add_liquidity::AddLiquidity),
    ClaimPartnerFee(claim_partner_fee::ClaimPartnerFee),
    ClaimPositionFee(claim_position_fee::ClaimPositionFee),
    ClaimProtocolFee(claim_protocol_fee::ClaimProtocolFee),
    ClaimReward(claim_reward::ClaimReward),
    CloseClaimFeeOperator(close_claim_fee_operator::CloseClaimFeeOperator),
    CloseConfig(close_config::CloseConfig),
    ClosePosition(close_position::ClosePosition),
    CreateClaimFeeOperator(create_claim_fee_operator::CreateClaimFeeOperator),
    CreateConfig(create_config::CreateConfig),
    CreateDynamicConfig(create_dynamic_config::CreateDynamicConfig),
    CreatePosition(create_position::CreatePosition),
    CreateTokenBadge(create_token_badge::CreateTokenBadge),
    FundReward(fund_reward::FundReward),
    InitializeCustomizablePool(initialize_customizable_pool::InitializeCustomizablePool),
    InitializePool(initialize_pool::InitializePool),
    InitializePoolWithDynamicConfig(
        initialize_pool_with_dynamic_config::InitializePoolWithDynamicConfig,
    ),
    InitializeReward(initialize_reward::InitializeReward),
    LockPosition(lock_position::LockPosition),
    PermanentLockPosition(permanent_lock_position::PermanentLockPosition),
    RefreshVesting(refresh_vesting::RefreshVesting),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    SetPoolStatus(set_pool_status::SetPoolStatus),
    Swap(swap::Swap),
    UpdateRewardDuration(update_reward_duration::UpdateRewardDuration),
    UpdateRewardFunder(update_reward_funder::UpdateRewardFunder),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    EvtAddLiquidityEvent(evt_add_liquidity_event::EvtAddLiquidityEvent),
    EvtClaimPartnerFeeEvent(evt_claim_partner_fee_event::EvtClaimPartnerFeeEvent),
    EvtClaimPositionFeeEvent(evt_claim_position_fee_event::EvtClaimPositionFeeEvent),
    EvtClaimProtocolFeeEvent(evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent),
    EvtClaimRewardEvent(evt_claim_reward_event::EvtClaimRewardEvent),
    EvtCloseClaimFeeOperatorEvent(
        evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
    ),
    EvtCloseConfigEvent(evt_close_config_event::EvtCloseConfigEvent),
    EvtClosePositionEvent(evt_close_position_event::EvtClosePositionEvent),
    EvtCreateClaimFeeOperatorEvent(
        evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
    ),
    EvtCreateConfigEvent(evt_create_config_event::EvtCreateConfigEvent),
    EvtCreateDynamicConfigEvent(evt_create_dynamic_config_event::EvtCreateDynamicConfigEvent),
    EvtCreatePositionEvent(evt_create_position_event::EvtCreatePositionEvent),
    EvtCreateTokenBadgeEvent(evt_create_token_badge_event::EvtCreateTokenBadgeEvent),
    EvtFundRewardEvent(evt_fund_reward_event::EvtFundRewardEvent),
    EvtInitializePoolEvent(evt_initialize_pool_event::EvtInitializePoolEvent),
    EvtInitializeRewardEvent(evt_initialize_reward_event::EvtInitializeRewardEvent),
    EvtLockPositionEvent(evt_lock_position_event::EvtLockPositionEvent),
    EvtPermanentLockPositionEvent(evt_permanent_lock_position_event::EvtPermanentLockPositionEvent),
    EvtRemoveLiquidityEvent(evt_remove_liquidity_event::EvtRemoveLiquidityEvent),
    EvtSetPoolStatusEvent(evt_set_pool_status_event::EvtSetPoolStatusEvent),
    EvtSwapEvent(evt_swap_event::EvtSwapEvent),
    EvtUpdateRewardDurationEvent(evt_update_reward_duration_event::EvtUpdateRewardDurationEvent),
    EvtUpdateRewardFunderEvent(evt_update_reward_funder_event::EvtUpdateRewardFunderEvent),
    EvtWithdrawIneligibleRewardEvent(
        evt_withdraw_ineligible_reward_event::EvtWithdrawIneligibleRewardEvent,
    ),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MeteoraDammV2Decoder {
    type InstructionType = MeteoraDammV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            MeteoraDammV2Instruction::AddLiquidity => add_liquidity::AddLiquidity,
            MeteoraDammV2Instruction::ClaimPartnerFee => claim_partner_fee::ClaimPartnerFee,
            MeteoraDammV2Instruction::ClaimPositionFee => claim_position_fee::ClaimPositionFee,
            MeteoraDammV2Instruction::ClaimProtocolFee => claim_protocol_fee::ClaimProtocolFee,
            MeteoraDammV2Instruction::ClaimReward => claim_reward::ClaimReward,
            MeteoraDammV2Instruction::CloseClaimFeeOperator => close_claim_fee_operator::CloseClaimFeeOperator,
            MeteoraDammV2Instruction::CloseConfig => close_config::CloseConfig,
            MeteoraDammV2Instruction::ClosePosition => close_position::ClosePosition,
            MeteoraDammV2Instruction::CreateClaimFeeOperator => create_claim_fee_operator::CreateClaimFeeOperator,
            MeteoraDammV2Instruction::CreateConfig => create_config::CreateConfig,
            MeteoraDammV2Instruction::CreateDynamicConfig => create_dynamic_config::CreateDynamicConfig,
            MeteoraDammV2Instruction::CreatePosition => create_position::CreatePosition,
            MeteoraDammV2Instruction::CreateTokenBadge => create_token_badge::CreateTokenBadge,
            MeteoraDammV2Instruction::FundReward => fund_reward::FundReward,
            MeteoraDammV2Instruction::InitializeCustomizablePool => initialize_customizable_pool::InitializeCustomizablePool,
            MeteoraDammV2Instruction::InitializePool => initialize_pool::InitializePool,
            MeteoraDammV2Instruction::InitializePoolWithDynamicConfig => initialize_pool_with_dynamic_config::InitializePoolWithDynamicConfig,
            MeteoraDammV2Instruction::InitializeReward => initialize_reward::InitializeReward,
            MeteoraDammV2Instruction::LockPosition => lock_position::LockPosition,
            MeteoraDammV2Instruction::PermanentLockPosition => permanent_lock_position::PermanentLockPosition,
            MeteoraDammV2Instruction::RefreshVesting => refresh_vesting::RefreshVesting,
            MeteoraDammV2Instruction::RemoveAllLiquidity => remove_all_liquidity::RemoveAllLiquidity,
            MeteoraDammV2Instruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            MeteoraDammV2Instruction::SetPoolStatus => set_pool_status::SetPoolStatus,
            MeteoraDammV2Instruction::Swap => swap::Swap,
            MeteoraDammV2Instruction::UpdateRewardDuration => update_reward_duration::UpdateRewardDuration,
            MeteoraDammV2Instruction::UpdateRewardFunder => update_reward_funder::UpdateRewardFunder,
            MeteoraDammV2Instruction::WithdrawIneligibleReward => withdraw_ineligible_reward::WithdrawIneligibleReward,
            MeteoraDammV2Instruction::EvtAddLiquidityEvent => evt_add_liquidity_event::EvtAddLiquidityEvent,
            MeteoraDammV2Instruction::EvtClaimPartnerFeeEvent => evt_claim_partner_fee_event::EvtClaimPartnerFeeEvent,
            MeteoraDammV2Instruction::EvtClaimPositionFeeEvent => evt_claim_position_fee_event::EvtClaimPositionFeeEvent,
            MeteoraDammV2Instruction::EvtClaimProtocolFeeEvent => evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent,
            MeteoraDammV2Instruction::EvtClaimRewardEvent => evt_claim_reward_event::EvtClaimRewardEvent,
            MeteoraDammV2Instruction::EvtCloseClaimFeeOperatorEvent => evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
            MeteoraDammV2Instruction::EvtCloseConfigEvent => evt_close_config_event::EvtCloseConfigEvent,
            MeteoraDammV2Instruction::EvtClosePositionEvent => evt_close_position_event::EvtClosePositionEvent,
            MeteoraDammV2Instruction::EvtCreateClaimFeeOperatorEvent => evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
            MeteoraDammV2Instruction::EvtCreateConfigEvent => evt_create_config_event::EvtCreateConfigEvent,
            MeteoraDammV2Instruction::EvtCreateDynamicConfigEvent => evt_create_dynamic_config_event::EvtCreateDynamicConfigEvent,
            MeteoraDammV2Instruction::EvtCreatePositionEvent => evt_create_position_event::EvtCreatePositionEvent,
            MeteoraDammV2Instruction::EvtCreateTokenBadgeEvent => evt_create_token_badge_event::EvtCreateTokenBadgeEvent,
            MeteoraDammV2Instruction::EvtFundRewardEvent => evt_fund_reward_event::EvtFundRewardEvent,
            MeteoraDammV2Instruction::EvtInitializePoolEvent => evt_initialize_pool_event::EvtInitializePoolEvent,
            MeteoraDammV2Instruction::EvtInitializeRewardEvent => evt_initialize_reward_event::EvtInitializeRewardEvent,
            MeteoraDammV2Instruction::EvtLockPositionEvent => evt_lock_position_event::EvtLockPositionEvent,
            MeteoraDammV2Instruction::EvtPermanentLockPositionEvent => evt_permanent_lock_position_event::EvtPermanentLockPositionEvent,
            MeteoraDammV2Instruction::EvtRemoveLiquidityEvent => evt_remove_liquidity_event::EvtRemoveLiquidityEvent,
            MeteoraDammV2Instruction::EvtSetPoolStatusEvent => evt_set_pool_status_event::EvtSetPoolStatusEvent,
            MeteoraDammV2Instruction::EvtSwapEvent => evt_swap_event::EvtSwapEvent,
            MeteoraDammV2Instruction::EvtUpdateRewardDurationEvent => evt_update_reward_duration_event::EvtUpdateRewardDurationEvent,
            MeteoraDammV2Instruction::EvtUpdateRewardFunderEvent => evt_update_reward_funder_event::EvtUpdateRewardFunderEvent,
            MeteoraDammV2Instruction::EvtWithdrawIneligibleRewardEvent => evt_withdraw_ineligible_reward_event::EvtWithdrawIneligibleRewardEvent,
        )
    }
}
