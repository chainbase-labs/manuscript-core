use crate::PROGRAM_ID;

use super::MeteoraDlmmDecoder;
pub mod add_liquidity;
pub mod add_liquidity2;
pub mod add_liquidity_by_strategy;
pub mod add_liquidity_by_strategy2;
pub mod add_liquidity_by_strategy_one_side;
pub mod add_liquidity_by_weight;
pub mod add_liquidity_event;
pub mod add_liquidity_one_side;
pub mod add_liquidity_one_side_precise;
pub mod add_liquidity_one_side_precise2;
pub mod claim_fee;
pub mod claim_fee2;
pub mod claim_fee_event;
pub mod claim_reward;
pub mod claim_reward2;
pub mod claim_reward_event;
pub mod close_claim_protocol_fee_operator;
pub mod close_position;
pub mod close_position2;
pub mod close_position_if_empty;
pub mod close_preset_parameter;
pub mod close_preset_parameter2;
pub mod composition_fee_event;
pub mod create_claim_protocol_fee_operator;
pub mod decrease_position_length_event;
pub mod dynamic_fee_parameter_update_event;
pub mod fee_parameter_update_event;
pub mod fund_reward;
pub mod fund_reward_event;
pub mod go_to_a_bin;
pub mod go_to_a_bin_event;
pub mod increase_observation_event;
pub mod increase_oracle_length;
pub mod increase_position_length_event;
pub mod initialize_bin_array;
pub mod initialize_bin_array_bitmap_extension;
pub mod initialize_customizable_permissionless_lb_pair;
pub mod initialize_customizable_permissionless_lb_pair2;
pub mod initialize_lb_pair;
pub mod initialize_lb_pair2;
pub mod initialize_permission_lb_pair;
pub mod initialize_position;
pub mod initialize_position_by_operator;
pub mod initialize_position_pda;
pub mod initialize_preset_parameter;
pub mod initialize_preset_parameter2;
pub mod initialize_reward;
pub mod initialize_reward_event;
pub mod initialize_token_badge;
pub mod lb_pair_create_event;
pub mod migrate_bin_array;
pub mod migrate_position;
pub mod position_close_event;
pub mod position_create_event;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod remove_liquidity2;
pub mod remove_liquidity_by_range;
pub mod remove_liquidity_by_range2;
pub mod remove_liquidity_event;
pub mod set_activation_point;
pub mod set_pair_status;
pub mod set_pair_status_permissionless;
pub mod set_pre_activation_duration;
pub mod set_pre_activation_swap_address;
pub mod swap;
pub mod swap2;
pub mod swap_event;
pub mod swap_exact_out;
pub mod swap_exact_out2;
pub mod swap_with_price_impact;
pub mod swap_with_price_impact2;
pub mod update_base_fee_parameters;
pub mod update_dynamic_fee_parameters;
pub mod update_fees_and_reward2;
pub mod update_fees_and_rewards;
pub mod update_position_lock_release_point_event;
pub mod update_position_operator;
pub mod update_position_operator_event;
pub mod update_reward_duration;
pub mod update_reward_duration_event;
pub mod update_reward_funder;
pub mod update_reward_funder_event;
pub mod withdraw_ineligible_reward;
pub mod withdraw_ineligible_reward_event;
pub mod withdraw_protocol_fee;

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
pub enum MeteoraDlmmInstruction {
    InitializeLbPair(initialize_lb_pair::InitializeLbPair),
    InitializePermissionLbPair(initialize_permission_lb_pair::InitializePermissionLbPair),
    InitializeCustomizablePermissionlessLbPair(initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair),
    InitializeBinArrayBitmapExtension(initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension),
    InitializeBinArray(initialize_bin_array::InitializeBinArray),
    AddLiquidity(add_liquidity::AddLiquidity),
    AddLiquidityByWeight(add_liquidity_by_weight::AddLiquidityByWeight),
    AddLiquidityByStrategy(add_liquidity_by_strategy::AddLiquidityByStrategy),
    AddLiquidityByStrategyOneSide(add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide),
    AddLiquidityOneSide(add_liquidity_one_side::AddLiquidityOneSide),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    InitializePosition(initialize_position::InitializePosition),
    InitializePositionPda(initialize_position_pda::InitializePositionPda),
    InitializePositionByOperator(initialize_position_by_operator::InitializePositionByOperator),
    UpdatePositionOperator(update_position_operator::UpdatePositionOperator),
    Swap(swap::Swap),
    SwapExactOut(swap_exact_out::SwapExactOut),
    SwapWithPriceImpact(swap_with_price_impact::SwapWithPriceImpact),
    WithdrawProtocolFee(withdraw_protocol_fee::WithdrawProtocolFee),
    InitializeReward(initialize_reward::InitializeReward),
    FundReward(fund_reward::FundReward),
    UpdateRewardFunder(update_reward_funder::UpdateRewardFunder),
    UpdateRewardDuration(update_reward_duration::UpdateRewardDuration),
    ClaimReward(claim_reward::ClaimReward),
    ClaimFee(claim_fee::ClaimFee),
    ClosePosition(close_position::ClosePosition),
    UpdateBaseFeeParameters(update_base_fee_parameters::UpdateBaseFeeParameters),
    UpdateDynamicFeeParameters(update_dynamic_fee_parameters::UpdateDynamicFeeParameters),
    IncreaseOracleLength(increase_oracle_length::IncreaseOracleLength),
    InitializePresetParameter(initialize_preset_parameter::InitializePresetParameter),
    ClosePresetParameter(close_preset_parameter::ClosePresetParameter),
    ClosePresetParameter2(close_preset_parameter2::ClosePresetParameter2),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    SetPairStatus(set_pair_status::SetPairStatus),
    MigratePosition(migrate_position::MigratePosition),
    MigrateBinArray(migrate_bin_array::MigrateBinArray),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    SetActivationPoint(set_activation_point::SetActivationPoint),
    RemoveLiquidityByRange(remove_liquidity_by_range::RemoveLiquidityByRange),
    AddLiquidityOneSidePrecise(add_liquidity_one_side_precise::AddLiquidityOneSidePrecise),
    GoToABin(go_to_a_bin::GoToABin),
    SetPreActivationDuration(set_pre_activation_duration::SetPreActivationDuration),
    SetPreActivationSwapAddress(set_pre_activation_swap_address::SetPreActivationSwapAddress),
    SetPairStatusPermissionless(set_pair_status_permissionless::SetPairStatusPermissionless),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    CreateClaimProtocolFeeOperator(create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator),
    CloseClaimProtocolFeeOperator(close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator),
    InitializePresetParameter2(initialize_preset_parameter2::InitializePresetParameter2),
    InitializeLbPair2(initialize_lb_pair2::InitializeLbPair2),
    InitializeCustomizablePermissionlessLbPair2(initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2),
    ClaimFee2(claim_fee2::ClaimFee2),
    ClaimReward2(claim_reward2::ClaimReward2),
    AddLiquidity2(add_liquidity2::AddLiquidity2),
    AddLiquidityByStrategy2(add_liquidity_by_strategy2::AddLiquidityByStrategy2),
    AddLiquidityOneSidePrecise2(add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2),
    RemoveLiquidity2(remove_liquidity2::RemoveLiquidity2),
    RemoveLiquidityByRange2(remove_liquidity_by_range2::RemoveLiquidityByRange2),
    Swap2(swap2::Swap2),
    SwapExactOut2(swap_exact_out2::SwapExactOut2),
    SwapWithPriceImpact2(swap_with_price_impact2::SwapWithPriceImpact2),
    ClosePosition2(close_position2::ClosePosition2),
    UpdateFeesAndReward2(update_fees_and_reward2::UpdateFeesAndReward2),
    ClosePositionIfEmpty(close_position_if_empty::ClosePositionIfEmpty),
    CompositionFeeEvent(composition_fee_event::CompositionFeeEvent),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    SwapEvent(swap_event::SwapEvent),
    ClaimRewardEvent(claim_reward_event::ClaimRewardEvent),
    FundRewardEvent(fund_reward_event::FundRewardEvent),
    InitializeRewardEvent(initialize_reward_event::InitializeRewardEvent),
    UpdateRewardDurationEvent(update_reward_duration_event::UpdateRewardDurationEvent),
    UpdateRewardFunderEvent(update_reward_funder_event::UpdateRewardFunderEvent),
    PositionCloseEvent(position_close_event::PositionCloseEvent),
    ClaimFeeEvent(claim_fee_event::ClaimFeeEvent),
    LbPairCreateEvent(lb_pair_create_event::LbPairCreateEvent),
    PositionCreateEvent(position_create_event::PositionCreateEvent),
    IncreasePositionLengthEvent(increase_position_length_event::IncreasePositionLengthEvent),
    DecreasePositionLengthEvent(decrease_position_length_event::DecreasePositionLengthEvent),
    FeeParameterUpdateEvent(fee_parameter_update_event::FeeParameterUpdateEvent),
    DynamicFeeParameterUpdateEvent(dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent),
    IncreaseObservationEvent(increase_observation_event::IncreaseObservationEvent),
    WithdrawIneligibleRewardEvent(withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent),
    UpdatePositionOperatorEvent(update_position_operator_event::UpdatePositionOperatorEvent),
    UpdatePositionLockReleasePointEvent(update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent),
    GoToABinEvent(go_to_a_bin_event::GoToABinEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MeteoraDlmmDecoder {
    type InstructionType = MeteoraDlmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MeteoraDlmmInstruction::InitializeLbPair => initialize_lb_pair::InitializeLbPair,
            MeteoraDlmmInstruction::InitializePermissionLbPair => initialize_permission_lb_pair::InitializePermissionLbPair,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair => initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair,
            MeteoraDlmmInstruction::InitializeBinArrayBitmapExtension => initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension,
            MeteoraDlmmInstruction::InitializeBinArray => initialize_bin_array::InitializeBinArray,
            MeteoraDlmmInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            MeteoraDlmmInstruction::AddLiquidityByWeight => add_liquidity_by_weight::AddLiquidityByWeight,
            MeteoraDlmmInstruction::AddLiquidityByStrategy => add_liquidity_by_strategy::AddLiquidityByStrategy,
            MeteoraDlmmInstruction::AddLiquidityByStrategyOneSide => add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide,
            MeteoraDlmmInstruction::AddLiquidityOneSide => add_liquidity_one_side::AddLiquidityOneSide,
            MeteoraDlmmInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            MeteoraDlmmInstruction::InitializePosition => initialize_position::InitializePosition,
            MeteoraDlmmInstruction::InitializePositionPda => initialize_position_pda::InitializePositionPda,
            MeteoraDlmmInstruction::InitializePositionByOperator => initialize_position_by_operator::InitializePositionByOperator,
            MeteoraDlmmInstruction::UpdatePositionOperator => update_position_operator::UpdatePositionOperator,
            MeteoraDlmmInstruction::Swap => swap::Swap,
            MeteoraDlmmInstruction::SwapExactOut => swap_exact_out::SwapExactOut,
            MeteoraDlmmInstruction::SwapWithPriceImpact => swap_with_price_impact::SwapWithPriceImpact,
            MeteoraDlmmInstruction::WithdrawProtocolFee => withdraw_protocol_fee::WithdrawProtocolFee,
            MeteoraDlmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            MeteoraDlmmInstruction::FundReward => fund_reward::FundReward,
            MeteoraDlmmInstruction::UpdateRewardFunder => update_reward_funder::UpdateRewardFunder,
            MeteoraDlmmInstruction::UpdateRewardDuration => update_reward_duration::UpdateRewardDuration,
            MeteoraDlmmInstruction::ClaimReward => claim_reward::ClaimReward,
            MeteoraDlmmInstruction::ClaimFee => claim_fee::ClaimFee,
            MeteoraDlmmInstruction::ClosePosition => close_position::ClosePosition,
            MeteoraDlmmInstruction::UpdateBaseFeeParameters => update_base_fee_parameters::UpdateBaseFeeParameters,
            MeteoraDlmmInstruction::UpdateDynamicFeeParameters => update_dynamic_fee_parameters::UpdateDynamicFeeParameters,
            MeteoraDlmmInstruction::IncreaseOracleLength => increase_oracle_length::IncreaseOracleLength,
            MeteoraDlmmInstruction::InitializePresetParameter => initialize_preset_parameter::InitializePresetParameter,
            MeteoraDlmmInstruction::ClosePresetParameter => close_preset_parameter::ClosePresetParameter,
            MeteoraDlmmInstruction::ClosePresetParameter2 => close_preset_parameter2::ClosePresetParameter2,
            MeteoraDlmmInstruction::RemoveAllLiquidity => remove_all_liquidity::RemoveAllLiquidity,
            MeteoraDlmmInstruction::SetPairStatus => set_pair_status::SetPairStatus,
            MeteoraDlmmInstruction::MigratePosition => migrate_position::MigratePosition,
            MeteoraDlmmInstruction::MigrateBinArray => migrate_bin_array::MigrateBinArray,
            MeteoraDlmmInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            MeteoraDlmmInstruction::WithdrawIneligibleReward => withdraw_ineligible_reward::WithdrawIneligibleReward,
            MeteoraDlmmInstruction::SetActivationPoint => set_activation_point::SetActivationPoint,
            MeteoraDlmmInstruction::RemoveLiquidityByRange => remove_liquidity_by_range::RemoveLiquidityByRange,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise => add_liquidity_one_side_precise::AddLiquidityOneSidePrecise,
            MeteoraDlmmInstruction::GoToABin => go_to_a_bin::GoToABin,
            MeteoraDlmmInstruction::SetPreActivationDuration => set_pre_activation_duration::SetPreActivationDuration,
            MeteoraDlmmInstruction::SetPreActivationSwapAddress => set_pre_activation_swap_address::SetPreActivationSwapAddress,
            MeteoraDlmmInstruction::SetPairStatusPermissionless => set_pair_status_permissionless::SetPairStatusPermissionless,
            MeteoraDlmmInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            MeteoraDlmmInstruction::CreateClaimProtocolFeeOperator => create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::CloseClaimProtocolFeeOperator => close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::InitializePresetParameter2 => initialize_preset_parameter2::InitializePresetParameter2,
            MeteoraDlmmInstruction::InitializeLbPair2 => initialize_lb_pair2::InitializeLbPair2,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair2 => initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2,
            MeteoraDlmmInstruction::ClaimFee2 => claim_fee2::ClaimFee2,
            MeteoraDlmmInstruction::ClaimReward2 => claim_reward2::ClaimReward2,
            MeteoraDlmmInstruction::AddLiquidity2 => add_liquidity2::AddLiquidity2,
            MeteoraDlmmInstruction::AddLiquidityByStrategy2 => add_liquidity_by_strategy2::AddLiquidityByStrategy2,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise2 => add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2,
            MeteoraDlmmInstruction::RemoveLiquidity2 => remove_liquidity2::RemoveLiquidity2,
            MeteoraDlmmInstruction::RemoveLiquidityByRange2 => remove_liquidity_by_range2::RemoveLiquidityByRange2,
            MeteoraDlmmInstruction::Swap2 => swap2::Swap2,
            MeteoraDlmmInstruction::SwapExactOut2 => swap_exact_out2::SwapExactOut2,
            MeteoraDlmmInstruction::SwapWithPriceImpact2 => swap_with_price_impact2::SwapWithPriceImpact2,
            MeteoraDlmmInstruction::ClosePosition2 => close_position2::ClosePosition2,
            MeteoraDlmmInstruction::UpdateFeesAndReward2 => update_fees_and_reward2::UpdateFeesAndReward2,
            MeteoraDlmmInstruction::ClosePositionIfEmpty => close_position_if_empty::ClosePositionIfEmpty,
            MeteoraDlmmInstruction::CompositionFeeEvent => composition_fee_event::CompositionFeeEvent,
            MeteoraDlmmInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MeteoraDlmmInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MeteoraDlmmInstruction::SwapEvent => swap_event::SwapEvent,
            MeteoraDlmmInstruction::ClaimRewardEvent => claim_reward_event::ClaimRewardEvent,
            MeteoraDlmmInstruction::FundRewardEvent => fund_reward_event::FundRewardEvent,
            MeteoraDlmmInstruction::InitializeRewardEvent => initialize_reward_event::InitializeRewardEvent,
            MeteoraDlmmInstruction::UpdateRewardDurationEvent => update_reward_duration_event::UpdateRewardDurationEvent,
            MeteoraDlmmInstruction::UpdateRewardFunderEvent => update_reward_funder_event::UpdateRewardFunderEvent,
            MeteoraDlmmInstruction::PositionCloseEvent => position_close_event::PositionCloseEvent,
            MeteoraDlmmInstruction::ClaimFeeEvent => claim_fee_event::ClaimFeeEvent,
            MeteoraDlmmInstruction::LbPairCreateEvent => lb_pair_create_event::LbPairCreateEvent,
            MeteoraDlmmInstruction::PositionCreateEvent => position_create_event::PositionCreateEvent,
            MeteoraDlmmInstruction::IncreasePositionLengthEvent => increase_position_length_event::IncreasePositionLengthEvent,
            MeteoraDlmmInstruction::DecreasePositionLengthEvent => decrease_position_length_event::DecreasePositionLengthEvent,
            MeteoraDlmmInstruction::FeeParameterUpdateEvent => fee_parameter_update_event::FeeParameterUpdateEvent,
            MeteoraDlmmInstruction::DynamicFeeParameterUpdateEvent => dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent,
            MeteoraDlmmInstruction::IncreaseObservationEvent => increase_observation_event::IncreaseObservationEvent,
            MeteoraDlmmInstruction::WithdrawIneligibleRewardEvent => withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent,
            MeteoraDlmmInstruction::UpdatePositionOperatorEvent => update_position_operator_event::UpdatePositionOperatorEvent,
            MeteoraDlmmInstruction::UpdatePositionLockReleasePointEvent => update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent,
            MeteoraDlmmInstruction::GoToABinEvent => go_to_a_bin_event::GoToABinEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    use crate::{
        instructions::{
            add_liquidity::{AddLiquidity, AddLiquidityInstructionAccounts},
            add_liquidity_by_strategy::{
                AddLiquidityByStrategy, AddLiquidityByStrategyInstructionAccounts,
            },
            add_liquidity_by_strategy_one_side::{
                AddLiquidityByStrategyOneSide, AddLiquidityByStrategyOneSideInstructionAccounts,
            },
            add_liquidity_by_weight::{
                AddLiquidityByWeight, AddLiquidityByWeightInstructionAccounts,
            },
            add_liquidity_one_side::{AddLiquidityOneSide, AddLiquidityOneSideInstructionAccounts},
            add_liquidity_one_side_precise::{
                AddLiquidityOneSidePrecise, AddLiquidityOneSidePreciseInstructionAccounts,
            },
            claim_fee::{ClaimFee, ClaimFeeInstructionAccounts},
            claim_fee2::{ClaimFee2, ClaimFee2InstructionAccounts},
            claim_reward::{ClaimReward, ClaimRewardInstructionAccounts},
            claim_reward2::{ClaimReward2, ClaimReward2InstructionAccounts},
            close_position::{ClosePosition, ClosePositionInstructionAccounts},
            close_position2::{ClosePosition2, ClosePosition2InstructionAccounts},
            close_position_if_empty::{
                ClosePositionIfEmpty, ClosePositionIfEmptyInstructionAccounts,
            },
            close_preset_parameter::{
                ClosePresetParameter, ClosePresetParameterInstructionAccounts,
            },
            go_to_a_bin::{GoToABin, GoToABinInstructionAccounts},
            initialize_bin_array::{InitializeBinArray, InitializeBinArrayInstructionAccounts},
            initialize_bin_array_bitmap_extension::{
                InitializeBinArrayBitmapExtension,
                InitializeBinArrayBitmapExtensionInstructionAccounts,
            },
            initialize_customizable_permissionless_lb_pair::{
                InitializeCustomizablePermissionlessLbPair,
                InitializeCustomizablePermissionlessLbPairInstructionAccounts,
            },
            initialize_customizable_permissionless_lb_pair2::{
                InitializeCustomizablePermissionlessLbPair2,
                InitializeCustomizablePermissionlessLbPair2InstructionAccounts,
            },
            initialize_lb_pair::{InitializeLbPair, InitializeLbPairInstructionAccounts},
            initialize_lb_pair2::{InitializeLbPair2, InitializeLbPair2InstructionAccounts},
            initialize_permission_lb_pair::{
                InitializePermissionLbPair, InitializePermissionLbPairInstructionAccounts,
            },
            initialize_position::{InitializePosition, InitializePositionInstructionAccounts},
            initialize_position_by_operator::{
                InitializePositionByOperator, InitializePositionByOperatorInstructionAccounts,
            },
            initialize_position_pda::{
                InitializePositionPda, InitializePositionPdaInstructionAccounts,
            },
            migrate_bin_array::{MigrateBinArray, MigrateBinArrayInstructionAccounts},
            migrate_position::{MigratePosition, MigratePositionInstructionAccounts},
            remove_all_liquidity::{RemoveAllLiquidity, RemoveAllLiquidityInstructionAccounts},
            remove_liquidity::{RemoveLiquidity, RemoveLiquidityInstructionAccounts},
            remove_liquidity2::{RemoveLiquidity2, RemoveLiquidity2InstructionAccounts},
            remove_liquidity_by_range::{
                RemoveLiquidityByRange, RemoveLiquidityByRangeInstructionAccounts,
            },
            remove_liquidity_by_range2::{
                RemoveLiquidityByRange2, RemoveLiquidityByRange2InstructionAccounts,
            },
            set_activation_point::{SetActivationPoint, SetActivationPointInstructionAccounts},
            set_pair_status::{SetPairStatus, SetPairStatusInstructionAccounts},
            set_pair_status_permissionless::{
                SetPairStatusPermissionless, SetPairStatusPermissionlessInstructionAccounts,
            },
            set_pre_activation_duration::{
                SetPreActivationDuration, SetPreActivationDurationInstructionAccounts,
            },
            set_pre_activation_swap_address::{
                SetPreActivationSwapAddress, SetPreActivationSwapAddressInstructionAccounts,
            },
            swap::{Swap, SwapInstructionAccounts},
            swap2::{Swap2, Swap2InstructionAccounts},
            swap_exact_out::{SwapExactOut, SwapExactOutInstructionAccounts},
            swap_exact_out2::{SwapExactOut2, SwapExactOut2InstructionAccounts},
            swap_with_price_impact2::{
                SwapWithPriceImpact2, SwapWithPriceImpact2InstructionAccounts,
            },
            update_base_fee_parameters::{
                UpdateBaseFeeParameters, UpdateBaseFeeParametersInstructionAccounts,
            },
            update_fees_and_rewards::{
                UpdateFeesAndRewards, UpdateFeesAndRewardsInstructionAccounts,
            },
        },
        types::{
            AccountsType, AddLiquiditySingleSidePreciseParameter, BaseFeeParameter,
            BinLiquidityDistribution, BinLiquidityDistributionByWeight, BinLiquidityReduction,
            CompressedBinDepositAmount, CustomizableParams, InitPermissionPairIx,
            InitializeLbPair2Params, LiquidityOneSideParameter, LiquidityParameter,
            LiquidityParameterByStrategy, LiquidityParameterByStrategyOneSide,
            LiquidityParameterByWeight, RemainingAccountsInfo, RemainingAccountsSlice,
            StrategyParameters, StrategyType,
        },
    };

    use super::*;

    #[test]
    fn test_decode_add_liquidity_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::AddLiquidity(AddLiquidity {
            liquidity_parameter: LiquidityParameter {
                amount_x: 300000000000000000,
                amount_y: 0,
                bin_liquidity_dist: vec![BinLiquidityDistribution {
                    bin_id: -2022,
                    distribution_x: 10000,
                    distribution_y: 0,
                }],
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("HmgRf3Pp6ZVzJ819Q7mghHF1u1FwExPPUW4wCLzBnuaa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DBS3T76RM6Ca1LuR5Rq5HSDJaRhFR1njskNpwAj2KJ1j"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5Dng6CfUnmB5c5uHF47ZLSvw5RJqXvJotAUfhsk4DzUe"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EZ52s5QnBB2RPzJWhEKKRq1qgXCrbPDAMUD6CpST1hyj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EgPtKDncu4xK7ZBrfqvoZ7SUBzFWUJ2UD2TiruZ7P6o2"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("A8CDf2UNKR8hDLDXiWnNJRUYCNiqSsRMnK5gScxpRjnL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("78fSXwNfQ1dt3Yo5UoX1c6HeT8JaQ1XSsenEh5gSoSt7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5X5bqEcaGmNp9bMC44JRmVrNFzckBcsefYSHTQB5zjYx"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityInstructionAccounts {
            position: pubkey!("HmgRf3Pp6ZVzJ819Q7mghHF1u1FwExPPUW4wCLzBnuaa"),
            lb_pair: pubkey!("DBS3T76RM6Ca1LuR5Rq5HSDJaRhFR1njskNpwAj2KJ1j"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
            user_token_y: pubkey!("5Dng6CfUnmB5c5uHF47ZLSvw5RJqXvJotAUfhsk4DzUe"),
            reserve_x: pubkey!("EZ52s5QnBB2RPzJWhEKKRq1qgXCrbPDAMUD6CpST1hyj"),
            reserve_y: pubkey!("EgPtKDncu4xK7ZBrfqvoZ7SUBzFWUJ2UD2TiruZ7P6o2"),
            token_x_mint: pubkey!("A8CDf2UNKR8hDLDXiWnNJRUYCNiqSsRMnK5gScxpRjnL"),
            token_y_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            bin_array_lower: pubkey!("78fSXwNfQ1dt3Yo5UoX1c6HeT8JaQ1XSsenEh5gSoSt7"),
            bin_array_upper: pubkey!("5X5bqEcaGmNp9bMC44JRmVrNFzckBcsefYSHTQB5zjYx"),
            sender: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/add_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_add_liquidity_by_strategy_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::AddLiquidityByStrategy(AddLiquidityByStrategy {
            liquidity_parameter: LiquidityParameterByStrategy {
                active_id: -729,
                amount_x: 0,
                amount_y: 21523400000,
                max_active_bin_slippage: 10000,
                strategy_parameters: StrategyParameters {
                    max_bin_id: -730,
                    min_bin_id: -730,
                    parameteres: [0; 64],
                    strategy_type: StrategyType::SpotBalanced,
                },
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("41FpGiwCcRr3qczHts3zqYuZERjddZ2dZDoLvhKzhaKJ"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2bYjEuupzFtBwgQuzSDkpnCFX9A2iBK6oL9JA3wXwbsa"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8d13MHH2DWZfkburgMQ5v9FGgifmK6ejgZM1jJzEVT7n"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3nVErWE28y5mU6WFPpbhUm7jqmDTrAiA4hN6vuXGUMuP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("G7AsqVKQ7yW7z2VZmk3FCMCyAqdgRq28aPnAWicGuc2b"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9hPJ6E7hXEQ5UTPC7SKK6D5mDQKTdZ87uQtG5DDaYrLB"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7XAyfFUXfCWbA1x6jCHVzeFtoKN3x836Pp5zQyhqpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("F5GxuK8c9Z7JnwBhKbjFbbLgTB9N2RvMXpZbGUVxU74c"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AambQn5DuPVWquMrQdhhYeAdM7X6eG2GDc35ek7HwZzB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5h5fDq1A17dKpFQEjkNfYYaJd7CAzymUchjfPZ4Dh8pa"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityByStrategyInstructionAccounts {
            position: pubkey!("41FpGiwCcRr3qczHts3zqYuZERjddZ2dZDoLvhKzhaKJ"),
            lb_pair: pubkey!("2bYjEuupzFtBwgQuzSDkpnCFX9A2iBK6oL9JA3wXwbsa"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("8d13MHH2DWZfkburgMQ5v9FGgifmK6ejgZM1jJzEVT7n"),
            user_token_y: pubkey!("3nVErWE28y5mU6WFPpbhUm7jqmDTrAiA4hN6vuXGUMuP"),
            reserve_x: pubkey!("G7AsqVKQ7yW7z2VZmk3FCMCyAqdgRq28aPnAWicGuc2b"),
            reserve_y: pubkey!("9hPJ6E7hXEQ5UTPC7SKK6D5mDQKTdZ87uQtG5DDaYrLB"),
            token_x_mint: pubkey!("7XAyfFUXfCWbA1x6jCHVzeFtoKN3x836Pp5zQyhqpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            bin_array_lower: pubkey!("F5GxuK8c9Z7JnwBhKbjFbbLgTB9N2RvMXpZbGUVxU74c"),
            bin_array_upper: pubkey!("AambQn5DuPVWquMrQdhhYeAdM7X6eG2GDc35ek7HwZzB"),
            sender: pubkey!("5h5fDq1A17dKpFQEjkNfYYaJd7CAzymUchjfPZ4Dh8pa"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/add_liquidity_by_strategy_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidityByStrategy::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_add_liquidity_by_strategy_one_side_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSide {
                liquidity_parameter: LiquidityParameterByStrategyOneSide {
                    active_id: -2059,
                    amount: 9700000000,
                    max_active_bin_slippage: 3,
                    strategy_parameters: StrategyParameters {
                        max_bin_id: -2060,
                        min_bin_id: -2100,
                        parameteres: [0; 64],
                        strategy_type: StrategyType::SpotOneSide,
                    },
                },
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AFJHGqyNai5kYaenM8FnR4bxdacScQHAyeu3NMeMjvuC"),
                true,
            ),
            AccountMeta::new(
                pubkey!("BGm1tav58oGcsQJehL9WXBFXF7D27vZsKefj4xJKD5Y"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HVS8WdPqAia9cEbxqGpXDbgJCgFa57Ufhcfnz4UKwUc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4N22J4vW2juHocTntJNmXywSonYjkndCwahjZ2cYLDgb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FE6nEninSdoZqVKXDUEa6NtsYfkL13uPB846FpRCF1WZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9o3pWJhKowpQukQTP5d3xwrGdUZtR5wmKGJwBiMbqGoM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FSM4tqYAWuv5Tvw2wF75AwkmijTjMCD6njYkPzq7m6Cz"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityByStrategyOneSideInstructionAccounts {
            position: pubkey!("AFJHGqyNai5kYaenM8FnR4bxdacScQHAyeu3NMeMjvuC"),
            lb_pair: pubkey!("BGm1tav58oGcsQJehL9WXBFXF7D27vZsKefj4xJKD5Y"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token: pubkey!("HVS8WdPqAia9cEbxqGpXDbgJCgFa57Ufhcfnz4UKwUc"),
            reserve: pubkey!("4N22J4vW2juHocTntJNmXywSonYjkndCwahjZ2cYLDgb"),
            token_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            bin_array_lower: pubkey!("FE6nEninSdoZqVKXDUEa6NtsYfkL13uPB846FpRCF1WZ"),
            bin_array_upper: pubkey!("9o3pWJhKowpQukQTP5d3xwrGdUZtR5wmKGJwBiMbqGoM"),
            sender: pubkey!("FSM4tqYAWuv5Tvw2wF75AwkmijTjMCD6njYkPzq7m6Cz"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/add_liquidity_by_strategy_one_side_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidityByStrategyOneSide::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_add_liquidity_by_weight_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::AddLiquidityByWeight(AddLiquidityByWeight {
            liquidity_parameter: LiquidityParameterByWeight {
                active_id: 1698,
                amount_x: 191094066,
                amount_y: 227162680,
                bin_liquidity_dist: vec![
                    BinLiquidityDistributionByWeight {
                        bin_id: 1687,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1688,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1689,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1690,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1691,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1692,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1693,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1694,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1695,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1696,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1697,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1698,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1699,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1700,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1701,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1702,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1703,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1704,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1705,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1706,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1707,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1708,
                        weight: 1,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: 1709,
                        weight: 1,
                    },
                ],
                max_active_bin_slippage: 2147483647,
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("GTg4upAYuAqXnSAdSGBbcgR7mbxcLwp8pxZp5iQe1tLJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BoeMUkCLHchTD31HdXsbDExuZZfcUppSLpYtV3LZTH6U"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6MZw9Y8FiGGjnL4DVXS812w9t9p5B9tK5xcrCWu6p2Ev"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H1HwdDkL5qHM8AZR9tBWYZQDTt9wwigKqCdF4KWuX7zP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("93d6ukn24o1xMcMDip2SACKG8GbvhGUZim1e3ZEcQVm2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CodroyzrRNvc5kHRoAQYjpVSr1jA9fLcUWVFouiuWGsD"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CzeWb8k7wDfubQz6McUcYkZuM6UQ7y2XJ6hfYBRCqBea"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5Kp3m4p7QxN629DA8wcHuLrHkecxEmdGFLRcB9PD4HAa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HdZCvCH4qwUqfy5YukMyyy5gYDhtmMWK7GvqEbLVsSWj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityByWeightInstructionAccounts {
            position: pubkey!("GTg4upAYuAqXnSAdSGBbcgR7mbxcLwp8pxZp5iQe1tLJ"),
            lb_pair: pubkey!("BoeMUkCLHchTD31HdXsbDExuZZfcUppSLpYtV3LZTH6U"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("6MZw9Y8FiGGjnL4DVXS812w9t9p5B9tK5xcrCWu6p2Ev"),
            user_token_y: pubkey!("H1HwdDkL5qHM8AZR9tBWYZQDTt9wwigKqCdF4KWuX7zP"),
            reserve_x: pubkey!("93d6ukn24o1xMcMDip2SACKG8GbvhGUZim1e3ZEcQVm2"),
            reserve_y: pubkey!("CodroyzrRNvc5kHRoAQYjpVSr1jA9fLcUWVFouiuWGsD"),
            token_x_mint: pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            bin_array_lower: pubkey!("CzeWb8k7wDfubQz6McUcYkZuM6UQ7y2XJ6hfYBRCqBea"),
            bin_array_upper: pubkey!("5Kp3m4p7QxN629DA8wcHuLrHkecxEmdGFLRcB9PD4HAa"),
            sender: pubkey!("HdZCvCH4qwUqfy5YukMyyy5gYDhtmMWK7GvqEbLVsSWj"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/add_liquidity_by_weight_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidityByWeight::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_add_liquidity_one_side_precise_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise(AddLiquidityOneSidePrecise {
                parameter: AddLiquiditySingleSidePreciseParameter {
                    bins: vec![
                        CompressedBinDepositAmount {
                            amount: 92867,
                            bin_id: -800,
                        },
                        CompressedBinDepositAmount {
                            amount: 206898,
                            bin_id: -799,
                        },
                        CompressedBinDepositAmount {
                            amount: 299326,
                            bin_id: -798,
                        },
                        CompressedBinDepositAmount {
                            amount: 384861,
                            bin_id: -797,
                        },
                        CompressedBinDepositAmount {
                            amount: 467370,
                            bin_id: -796,
                        },
                        CompressedBinDepositAmount {
                            amount: 548676,
                            bin_id: -795,
                        },
                    ],
                    decompress_multiplier: 1000000000,
                },
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ce8W9ThaNsa15BtyvnGU85xTLGANgpWqyxwt5o4uaKUv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3uref9YcGnma388KKM2kqP6NAcuCdPh5NbwYpqzcYZfK"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("J75TPk5xqCR6VPN6ZvyX2A2TvPG7oE8oCggmp1kHWmq1"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HSKLo63VLJj61oNwfMMr9S761NRpm1U6s2pXfBRqGdzG"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Grk4aSvTDdeNgi5JodDTuwGCb1oXLj9dmTfn4x5o7Q9t"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Huk9wcMCsNXiDj8SocVm5axc3WZ9gpzdWaQ8waTVpRqU"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8zzYe5K8CfemwbivccZvXbZpNbMgkEijVitNNTwfpjYk"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7CNXNAdUDBDYvZPGhRx8MG6nMeCxHC9B1iBVPY4H1gTU"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityOneSidePreciseInstructionAccounts {
            position: pubkey!("Ce8W9ThaNsa15BtyvnGU85xTLGANgpWqyxwt5o4uaKUv"),
            lb_pair: pubkey!("3uref9YcGnma388KKM2kqP6NAcuCdPh5NbwYpqzcYZfK"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token: pubkey!("J75TPk5xqCR6VPN6ZvyX2A2TvPG7oE8oCggmp1kHWmq1"),
            reserve: pubkey!("HSKLo63VLJj61oNwfMMr9S761NRpm1U6s2pXfBRqGdzG"),
            token_mint: pubkey!("Grk4aSvTDdeNgi5JodDTuwGCb1oXLj9dmTfn4x5o7Q9t"),
            bin_array_lower: pubkey!("Huk9wcMCsNXiDj8SocVm5axc3WZ9gpzdWaQ8waTVpRqU"),
            bin_array_upper: pubkey!("8zzYe5K8CfemwbivccZvXbZpNbMgkEijVitNNTwfpjYk"),
            sender: pubkey!("7CNXNAdUDBDYvZPGhRx8MG6nMeCxHC9B1iBVPY4H1gTU"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/add_liquidity_one_side_precise_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidityOneSidePrecise::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_add_liquidity_one_side_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::AddLiquidityOneSide(AddLiquidityOneSide {
            liquidity_parameter: LiquidityOneSideParameter {
                active_id: -428,
                amount: 25650410760,
                bin_liquidity_dist: vec![
                    BinLiquidityDistributionByWeight {
                        bin_id: -428,
                        weight: 2252,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -427,
                        weight: 4568,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -426,
                        weight: 5738,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -425,
                        weight: 6918,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -424,
                        weight: 8088,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -423,
                        weight: 9266,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -422,
                        weight: 10439,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -421,
                        weight: 8092,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -420,
                        weight: 5739,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -419,
                        weight: 3388,
                    },
                    BinLiquidityDistributionByWeight {
                        bin_id: -418,
                        weight: 1041,
                    },
                ],
                max_active_bin_slippage: 4,
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("D6SJ8sMRou9nrU4JsWC1SdtgdQroykjf3bLV12MLjnwv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9NNAwkhZGTWbb3ASqsZcp8vAx9TSqa6QrBiNbRGvcBaN"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3yKz2iUNG2QdYkkejp9HUUtzt4PimkZumTrQpKkewepn"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7VhWRicruPXN8W8w3mTWUjrnQQAFrjB1REtGiGTH8cPz"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7Y6Rix8X2botuMaJ2mno4UqSWdQZqU4RezY6qp7zpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HXsEBdoAusbKCnixyPMAmTX1HbNJ9KEJQi32jZ7dbHQm"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3SjbLYKjDWGLqmW6xx8UnQU9ahkNUjpD1AshXxDh7ryq"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GdLm4b7x67JaieLYkPkXpzyqX9RhLGmzM6MDXhTpU7kf"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = AddLiquidityOneSideInstructionAccounts {
            position: pubkey!("D6SJ8sMRou9nrU4JsWC1SdtgdQroykjf3bLV12MLjnwv"),
            lb_pair: pubkey!("9NNAwkhZGTWbb3ASqsZcp8vAx9TSqa6QrBiNbRGvcBaN"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token: pubkey!("3yKz2iUNG2QdYkkejp9HUUtzt4PimkZumTrQpKkewepn"),
            reserve: pubkey!("7VhWRicruPXN8W8w3mTWUjrnQQAFrjB1REtGiGTH8cPz"),
            token_mint: pubkey!("7Y6Rix8X2botuMaJ2mno4UqSWdQZqU4RezY6qp7zpump"),
            bin_array_lower: pubkey!("HXsEBdoAusbKCnixyPMAmTX1HbNJ9KEJQi32jZ7dbHQm"),
            bin_array_upper: pubkey!("3SjbLYKjDWGLqmW6xx8UnQU9ahkNUjpD1AshXxDh7ryq"),
            sender: pubkey!("GdLm4b7x67JaieLYkPkXpzyqX9RhLGmzM6MDXhTpU7kf"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/add_liquidity_one_side_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            AddLiquidityOneSide::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_claim_fee_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClaimFee(ClaimFee {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CczaEikMAfA4F6gaXKBe5qfd2Mwc7pu4KpetTg88Tica"),
                false,
            ),
            AccountMeta::new(
                pubkey!("C2poDHs8dRBC8HsM8zzw4Zwe1pFMgD9LfXJd25M3Hajz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2rhbcMDNDxVqBJR8EnKWmZb8qTUD5ga67mDz4m2uPtSR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AdPnhr43oMUtNr6k9GUZd65zwKcSCLQXouPyYkjQyGsv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("71KMt1Z57CnQpgxuHU5wFvweakjpvsjGx5ucADnrGoDr"),
                true,
            ),
            AccountMeta::new(
                pubkey!("za39KwZejtyZEfLVk1PqumSKcwstQX3yqVK8k2pgmRL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6mfAFMJCqMuQUvDAoYcxhC1wmHuAey3AYa79sEHGLp6s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FsKePukyoEW14F7XVkeS38veoCR6VAxjXprm6L6C9gfK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7Mv53UFYF3b9vaFsmhVTLMsnfBMY9L2699zcQRjvBbJm"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Hjw6bEcHtbHGpQr8onG3izfJY5DJiWdt7uk2BfdSpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClaimFeeInstructionAccounts {
            lb_pair: pubkey!("CczaEikMAfA4F6gaXKBe5qfd2Mwc7pu4KpetTg88Tica"),
            position: pubkey!("C2poDHs8dRBC8HsM8zzw4Zwe1pFMgD9LfXJd25M3Hajz"),
            bin_array_lower: pubkey!("2rhbcMDNDxVqBJR8EnKWmZb8qTUD5ga67mDz4m2uPtSR"),
            bin_array_upper: pubkey!("AdPnhr43oMUtNr6k9GUZd65zwKcSCLQXouPyYkjQyGsv"),
            sender: pubkey!("71KMt1Z57CnQpgxuHU5wFvweakjpvsjGx5ucADnrGoDr"),
            reserve_x: pubkey!("za39KwZejtyZEfLVk1PqumSKcwstQX3yqVK8k2pgmRL"),
            reserve_y: pubkey!("6mfAFMJCqMuQUvDAoYcxhC1wmHuAey3AYa79sEHGLp6s"),
            user_token_x: pubkey!("FsKePukyoEW14F7XVkeS38veoCR6VAxjXprm6L6C9gfK"),
            user_token_y: pubkey!("7Mv53UFYF3b9vaFsmhVTLMsnfBMY9L2699zcQRjvBbJm"),
            token_x_mint: pubkey!("Hjw6bEcHtbHGpQr8onG3izfJY5DJiWdt7uk2BfdSpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/claim_fee_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClaimFee::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_claim_reward_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClaimReward(ClaimReward { reward_index: 0 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("J5BwKjUTKLdYQf9sexskHgHLyVWHtmG7gPxf3sGUfJLJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5VAkBGkRRsoPx6PxQr4jeCAGS1Mb1PHuqDPcFZPquCLX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AnDCpj7VxG72LvXdaCn7zsHjaVruSBDWTbpVcdSGiFkK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5B6DoUgQHYfftC89eALn94QfaUzjKJUr3dKQzKYEXDih"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HWg7gUo8aSZD61q4wfwwr9AYZDDZtb5NKgLVpGuxvgw3"),
                true,
            ),
            AccountMeta::new(
                pubkey!("A4kyhECGSMiKoP5DDQa4xqN3QQwwpmkNW5X1vdYDVyGk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("HzwqbKZw8HxMN6bF2yFZNrht3c2iXXzpKcFu7uBEDKtr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5kZ9Fis9hvdnhVCpxHGXCc9Yjf1GagLnacJkxSikJ77G"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClaimRewardInstructionAccounts {
            lb_pair: pubkey!("J5BwKjUTKLdYQf9sexskHgHLyVWHtmG7gPxf3sGUfJLJ"),
            position: pubkey!("5VAkBGkRRsoPx6PxQr4jeCAGS1Mb1PHuqDPcFZPquCLX"),
            bin_array_lower: pubkey!("AnDCpj7VxG72LvXdaCn7zsHjaVruSBDWTbpVcdSGiFkK"),
            bin_array_upper: pubkey!("5B6DoUgQHYfftC89eALn94QfaUzjKJUr3dKQzKYEXDih"),
            sender: pubkey!("HWg7gUo8aSZD61q4wfwwr9AYZDDZtb5NKgLVpGuxvgw3"),
            reward_vault: pubkey!("A4kyhECGSMiKoP5DDQa4xqN3QQwwpmkNW5X1vdYDVyGk"),
            reward_mint: pubkey!("HzwqbKZw8HxMN6bF2yFZNrht3c2iXXzpKcFu7uBEDKtr"),
            user_token_account: pubkey!("5kZ9Fis9hvdnhVCpxHGXCc9Yjf1GagLnacJkxSikJ77G"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/claim_reward_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClaimReward::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_position_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClosePosition(ClosePosition {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Bqy3p4nsgPeddNLD98cDFHvXTVNd3ay1qU8aiE75Me8E"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8aSioDcoMjZ1gQYXSZTKetXaceUts2ZBHz2twKqYRGK1"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8hUdax3HbAeRbxAK59xUZBJytmpHu5CkFbKFfkyMph1F"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HsSsSEd6YDRVRbqbUop3uWPaFtqobyrJUraQ7AxhgRy5"),
                false,
            ),
            AccountMeta::new(pubkey!("u9KjZ2Lie1EuJF3BsMR3U7PKSPesCQFkTupK4hantbs"), true),
            AccountMeta::new(pubkey!("u9KjZ2Lie1EuJF3BsMR3U7PKSPesCQFkTupK4hantbs"), true),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClosePositionInstructionAccounts {
            position: pubkey!("Bqy3p4nsgPeddNLD98cDFHvXTVNd3ay1qU8aiE75Me8E"),
            lb_pair: pubkey!("8aSioDcoMjZ1gQYXSZTKetXaceUts2ZBHz2twKqYRGK1"),
            bin_array_lower: pubkey!("8hUdax3HbAeRbxAK59xUZBJytmpHu5CkFbKFfkyMph1F"),
            bin_array_upper: pubkey!("HsSsSEd6YDRVRbqbUop3uWPaFtqobyrJUraQ7AxhgRy5"),
            sender: pubkey!("u9KjZ2Lie1EuJF3BsMR3U7PKSPesCQFkTupK4hantbs"),
            rent_receiver: pubkey!("u9KjZ2Lie1EuJF3BsMR3U7PKSPesCQFkTupK4hantbs"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_preset_parameter_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClosePresetParameter(ClosePresetParameter {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Axsuoe5peu6xPnxB4eAzsgWwpgqymjQT5aJ4KE8NgQei"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
        ];
        let expected_arranged_accounts = ClosePresetParameterInstructionAccounts {
            preset_parameter: pubkey!("Axsuoe5peu6xPnxB4eAzsgWwpgqymjQT5aJ4KE8NgQei"),
            admin: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
            rent_receiver: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_preset_parameter_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePresetParameter::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_go_to_a_bin_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::GoToABin(GoToABin { bin_id: -11653 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CRnGNKPrdgucFPSMsMn7ENAA2eJWn6CsCtunpVU8b3v8"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = GoToABinInstructionAccounts {
            lb_pair: pubkey!("CRnGNKPrdgucFPSMsMn7ENAA2eJWn6CsCtunpVU8b3v8"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            from_bin_array: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            to_bin_array: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/go_to_a_bin_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            GoToABin::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_bin_array_bitmap_extension_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializeBinArrayBitmapExtension(
            InitializeBinArrayBitmapExtension {},
        );
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("7bGcnxwxHhWKtE7L7opZPqzJH6aU3HbRbuQnrGKf8779"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H2mrTZ7gSQ6EknZUeBakw6u6VEfd4t4g7KEPWgRV5Ffv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BZ9BoTtGkPXCJ73EdBLvy36kop8tMpzHXVboEnB71T2m"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializeBinArrayBitmapExtensionInstructionAccounts {
            lb_pair: pubkey!("7bGcnxwxHhWKtE7L7opZPqzJH6aU3HbRbuQnrGKf8779"),
            bin_array_bitmap_extension: pubkey!("H2mrTZ7gSQ6EknZUeBakw6u6VEfd4t4g7KEPWgRV5Ffv"),
            funder: pubkey!("BZ9BoTtGkPXCJ73EdBLvy36kop8tMpzHXVboEnB71T2m"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_bin_array_bitmap_extension_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeBinArrayBitmapExtension::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_bin_array_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::InitializeBinArray(InitializeBinArray { index: 6 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Bwf8q9r1wgSvQWcAdLXJqMM6icGSVW5BWrHWEEpyCAcP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("C4ytp5DSAHx2nveimbZYR3rBPgvPKPYQY38V8abDNiUh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("235sX2nwcsYqJrzSYeViPJJNgaQ73atZ3oJbYqt5eA1i"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = InitializeBinArrayInstructionAccounts {
            lb_pair: pubkey!("Bwf8q9r1wgSvQWcAdLXJqMM6icGSVW5BWrHWEEpyCAcP"),
            bin_array: pubkey!("C4ytp5DSAHx2nveimbZYR3rBPgvPKPYQY38V8abDNiUh"),
            funder: pubkey!("235sX2nwcsYqJrzSYeViPJJNgaQ73atZ3oJbYqt5eA1i"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_bin_array_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeBinArray::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_customizable_permissionless_lb_pair_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair(
            InitializeCustomizablePermissionlessLbPair {
                params: CustomizableParams {
                    activation_point: None,
                    activation_type: 0,
                    active_id: 1,
                    base_factor: 5000,
                    bin_step: 2,
                    has_alpha_vault: false,
                    creator_pool_on_off_control: false,
                    base_fee_power_factor: 0,
                    padding: [0; 62],
                },
            },
        );
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Em94tW6wo3K15z7mM6wW926QRamuLKhiRe52tvk92ARs"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("BSJUzBQfPe7snyjxJxAJG183yYhDtLUEi3c8LGW7DCVw"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5Lbm1X6oYcSUcscMmvWqmg7sz2Qh3XJ3ydBy2v8cTrTV"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Fov1U4FMX8u4Ln5gM9LHsZcWgwHAeZ526usUjaYza2BR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AG47v72cfwRUwdk9M2AfE691wzsETrjo8iieH3gAWsTX"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("8dn7co13une6bT7dt7caCFj5E8cCuH5wzCZLaTncL3D2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3A2f4jax9vQRsaxDpocDKA3MhHdibwwZBSsBFcgfgkLw"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts =
            InitializeCustomizablePermissionlessLbPairInstructionAccounts {
                lb_pair: pubkey!("Em94tW6wo3K15z7mM6wW926QRamuLKhiRe52tvk92ARs"),
                bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                token_mint_x: pubkey!("BSJUzBQfPe7snyjxJxAJG183yYhDtLUEi3c8LGW7DCVw"),
                token_mint_y: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                reserve_x: pubkey!("5Lbm1X6oYcSUcscMmvWqmg7sz2Qh3XJ3ydBy2v8cTrTV"),
                reserve_y: pubkey!("Fov1U4FMX8u4Ln5gM9LHsZcWgwHAeZ526usUjaYza2BR"),
                oracle: pubkey!("AG47v72cfwRUwdk9M2AfE691wzsETrjo8iieH3gAWsTX"),
                user_token_x: pubkey!("8dn7co13une6bT7dt7caCFj5E8cCuH5wzCZLaTncL3D2"),
                funder: pubkey!("3A2f4jax9vQRsaxDpocDKA3MhHdibwwZBSsBFcgfgkLw"),
                token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                system_program: pubkey!("11111111111111111111111111111111"),
                user_token_y: pubkey!("SysvarRent111111111111111111111111111111111"),
                event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_customizable_permissionless_lb_pair_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeCustomizablePermissionlessLbPair::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_customizable_permissionless_lb_pair2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair2(
            InitializeCustomizablePermissionlessLbPair2 {
                params: CustomizableParams {
                    active_id: -1157,
                    bin_step: 100,
                    base_factor: 10000,
                    activation_type: 1,
                    has_alpha_vault: true,
                    activation_point: Some(1748602200),
                    creator_pool_on_off_control: false,
                    base_fee_power_factor: 0,
                    padding: [0; 62],
                },
            },
        );
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("8ookeFtaPGhyi3dSJY9ULjzePWfAe1cykRr7xtoSbgfz"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("FCjQFN49K5ufVnXkLHMPSMvtbyo8jMB1tdrGQ8QHtfnD"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8k6Cwds2WbD9cx4hTfZubaSLpzniL8Vs8ZKo5ZQAsfQW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8994hrKpj7aoZocJvbV59Z9o4ah1se1HjW4Wsi6wSiMu"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HMCCePCcK1jizjz4MJx3VYb9f9uHXY4UJPcaaRtwATVZ"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CjGtuik6GHKbTh9xuDHCzq2reGtMcJtTMqHjkBcbCMNX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GMtwcuktJfrRcnyGktWW4Vab8cfjPcBy3xbuZgRegw6E"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("23s7qdnDSVxFqGUURKMdTz8Pte1pf3T92kGmXWrWi8vS"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts =
            InitializeCustomizablePermissionlessLbPair2InstructionAccounts {
                lb_pair: pubkey!("8ookeFtaPGhyi3dSJY9ULjzePWfAe1cykRr7xtoSbgfz"),
                bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                token_mint_x: pubkey!("FCjQFN49K5ufVnXkLHMPSMvtbyo8jMB1tdrGQ8QHtfnD"),
                token_mint_y: pubkey!("So11111111111111111111111111111111111111112"),
                reserve_x: pubkey!("8k6Cwds2WbD9cx4hTfZubaSLpzniL8Vs8ZKo5ZQAsfQW"),
                reserve_y: pubkey!("8994hrKpj7aoZocJvbV59Z9o4ah1se1HjW4Wsi6wSiMu"),
                oracle: pubkey!("HMCCePCcK1jizjz4MJx3VYb9f9uHXY4UJPcaaRtwATVZ"),
                user_token_x: pubkey!("CjGtuik6GHKbTh9xuDHCzq2reGtMcJtTMqHjkBcbCMNX"),
                funder: pubkey!("GMtwcuktJfrRcnyGktWW4Vab8cfjPcBy3xbuZgRegw6E"),
                token_badge_x: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                token_badge_y: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                token_program_x: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                token_program_y: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                system_program: pubkey!("11111111111111111111111111111111"),
                user_token_y: pubkey!("23s7qdnDSVxFqGUURKMdTz8Pte1pf3T92kGmXWrWi8vS"),
                event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_customizable_permissionless_lb_pair2_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeCustomizablePermissionlessLbPair2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_lb_pair_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializeLbPair(InitializeLbPair {
            active_id: 91,
            bin_step: 250,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("12PMuMKowpmSndiUvQVZ9Nbs4ck6X35iqoZXBerR4ojf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("631BvY2KLFbUcQvUhK2nn2buWw1K1yfS2eMHU8t9pump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5oHBxFoJqGjL6eZHi9buv3JvvxdutVEVsZNEyafjWroY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("J6TQ9iXZGg1K5P2MXtBqkZUjMbRPL26D3y21DtaPCxAm"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3qQXfWooz78RBCpC1My76mBcgbPAPDK3HrhJmjRsxicL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("83XWJx8TNDYtiKA15EbCBKSTvNZ3h2jGjHz87CXncRVH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DiPkWknz9MccGhGmj4ku7yWuyhtmNP385j1UXnHLZPqZ"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializeLbPairInstructionAccounts {
            lb_pair: pubkey!("12PMuMKowpmSndiUvQVZ9Nbs4ck6X35iqoZXBerR4ojf"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_mint_x: pubkey!("631BvY2KLFbUcQvUhK2nn2buWw1K1yfS2eMHU8t9pump"),
            token_mint_y: pubkey!("So11111111111111111111111111111111111111112"),
            reserve_x: pubkey!("5oHBxFoJqGjL6eZHi9buv3JvvxdutVEVsZNEyafjWroY"),
            reserve_y: pubkey!("J6TQ9iXZGg1K5P2MXtBqkZUjMbRPL26D3y21DtaPCxAm"),
            oracle: pubkey!("3qQXfWooz78RBCpC1My76mBcgbPAPDK3HrhJmjRsxicL"),
            preset_parameter: pubkey!("83XWJx8TNDYtiKA15EbCBKSTvNZ3h2jGjHz87CXncRVH"),
            funder: pubkey!("DiPkWknz9MccGhGmj4ku7yWuyhtmNP385j1UXnHLZPqZ"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_lb_pair_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeLbPair::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_permission_lb_pair_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::InitializePermissionLbPair(InitializePermissionLbPair {
                ix_data: InitPermissionPairIx {
                    active_id: -48930,
                    bin_step: 1,
                    base_factor: 10000,
                    base_fee_power_factor: 0,
                    activation_type: 1,
                    protocol_share: 500,
                },
            });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("5FtFfp6KyxmTzb4if6FkFDTZqMN6FMdMd6A2eAkuhwNN"),
                true,
            ),
            AccountMeta::new(
                pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BvNQqUt9NwKb9nBrmpAH6mqvxXM6Uno6eC4cewD1PhwV"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("HUMA1821qVDKta3u2ovmfDQeW2fSQouSKE8fkF44wvGw"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EiNhNKD4nR5zZm6YkAKpWHE2wWGSof5LZ7uSGfG3star"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EoW2xdPSyDeRyGiATyZ58sPuchPfvQnNudLq1dLKYX8o"),
                false,
            ),
            AccountMeta::new(
                pubkey!("sUiZ2rXZyD2B5aGVZeK9XXrsgCAgvhNPedTuePVpkA2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePermissionLbPairInstructionAccounts {
            base: pubkey!("5FtFfp6KyxmTzb4if6FkFDTZqMN6FMdMd6A2eAkuhwNN"),
            lb_pair: pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
            bin_array_bitmap_extension: pubkey!("BvNQqUt9NwKb9nBrmpAH6mqvxXM6Uno6eC4cewD1PhwV"),
            token_mint_x: pubkey!("HUMA1821qVDKta3u2ovmfDQeW2fSQouSKE8fkF44wvGw"),
            token_mint_y: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            reserve_x: pubkey!("EiNhNKD4nR5zZm6YkAKpWHE2wWGSof5LZ7uSGfG3star"),
            reserve_y: pubkey!("EoW2xdPSyDeRyGiATyZ58sPuchPfvQnNudLq1dLKYX8o"),
            oracle: pubkey!("sUiZ2rXZyD2B5aGVZeK9XXrsgCAgvhNPedTuePVpkA2"),
            admin: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
            token_badge_x: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_badge_y: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_program_x: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_y: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_permission_lb_pair_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePermissionLbPair::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_position_by_operator_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::InitializePositionByOperator(InitializePositionByOperator {
                fee_owner: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                lock_release_point: 0,
                lower_bin_id: -2022,
                width: 1,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                true,
            ),
            AccountMeta::new(
                pubkey!("HmgRf3Pp6ZVzJ819Q7mghHF1u1FwExPPUW4wCLzBnuaa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DBS3T76RM6Ca1LuR5Rq5HSDJaRhFR1njskNpwAj2KJ1j"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePositionByOperatorInstructionAccounts {
            payer: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
            base: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
            position: pubkey!("HmgRf3Pp6ZVzJ819Q7mghHF1u1FwExPPUW4wCLzBnuaa"),
            lb_pair: pubkey!("DBS3T76RM6Ca1LuR5Rq5HSDJaRhFR1njskNpwAj2KJ1j"),
            owner: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
            operator: pubkey!("2yRC3SXXh8dUpHfKYXyWwn1b1QJPrJwuJ7UKtLU4Usmu"),
            operator_token_x: pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
            owner_token_x: pubkey!("2JJpTp9mxYDfMv2LeVYwNWqaiiPUkcyCQeG1MDBTGA7G"),
            system_program: pubkey!("11111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_position_by_operator_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePositionByOperator::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_position_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializePosition(InitializePosition {
            lower_bin_id: -730,
            width: 1,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("2gV1bRqcF4kmmwe3ahUbzD6xaDE1E1hZ7cys2hWis6Cs"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2zQPS74ub2Rsyjcjn226HGJVGhQSCLZmsrezBGcee4JY"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2bYjEuupzFtBwgQuzSDkpnCFX9A2iBK6oL9JA3wXwbsa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2gV1bRqcF4kmmwe3ahUbzD6xaDE1E1hZ7cys2hWis6Cs"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePositionInstructionAccounts {
            payer: pubkey!("2gV1bRqcF4kmmwe3ahUbzD6xaDE1E1hZ7cys2hWis6Cs"),
            position: pubkey!("2zQPS74ub2Rsyjcjn226HGJVGhQSCLZmsrezBGcee4JY"),
            lb_pair: pubkey!("2bYjEuupzFtBwgQuzSDkpnCFX9A2iBK6oL9JA3wXwbsa"),
            owner: pubkey!("2gV1bRqcF4kmmwe3ahUbzD6xaDE1E1hZ7cys2hWis6Cs"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_position_pda_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializePositionPda(InitializePositionPda {
            lower_bin_id: -2070,
            width: 70,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5xScxBWRWFVuMGBLJFVSsj2jAi5WZzNXD7skTS4Qqyed"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("4gUTbkvyP6nMS74N6pTtWt9pSLagjewLf221DriGZAML"),
                true,
            ),
            AccountMeta::new(
                pubkey!("75uZqmbettMfwem3w7KvWdRHHhx5iYE7LuGj8fV5oHAn"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("8gJ7UWboMeQ6z6AQwFP3cAZwSYG8udVS2UesyCbH79r7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5xScxBWRWFVuMGBLJFVSsj2jAi5WZzNXD7skTS4Qqyed"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePositionPdaInstructionAccounts {
            payer: pubkey!("5xScxBWRWFVuMGBLJFVSsj2jAi5WZzNXD7skTS4Qqyed"),
            base: pubkey!("4gUTbkvyP6nMS74N6pTtWt9pSLagjewLf221DriGZAML"),
            position: pubkey!("75uZqmbettMfwem3w7KvWdRHHhx5iYE7LuGj8fV5oHAn"),
            lb_pair: pubkey!("8gJ7UWboMeQ6z6AQwFP3cAZwSYG8udVS2UesyCbH79r7"),
            owner: pubkey!("5xScxBWRWFVuMGBLJFVSsj2jAi5WZzNXD7skTS4Qqyed"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_position_pda_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePositionPda::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_migrate_position_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::MigratePosition(MigratePosition {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AEYyKDsv4fRebC2pAmv3hksXwtzzDKG5dCMxbXrVKzRf"),
                true,
            ),
            AccountMeta::new(
                pubkey!("8y9Rjqvb3UdT9SDfcM7v94Po2G36ndVM3pe1gLpGLRhx"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("FoSDw2L5DmTuQTFe55gWPDXf88euaxAEKFre74CnvQbX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6XnihiWoRfgFUncqAYeajEFEdu5m5bpY9kA39goDuDk9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3kBRpy3Lj434ZXiCJ8u7TSRDcL2PKCGCCHrW4WuZWzbZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DxocTN1otn4MUpJZED6XFbRxQmK5RrB3hqMYjoBZnXUb"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new(
                pubkey!("DxocTN1otn4MUpJZED6XFbRxQmK5RrB3hqMYjoBZnXUb"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = MigratePositionInstructionAccounts {
            position_v2: pubkey!("AEYyKDsv4fRebC2pAmv3hksXwtzzDKG5dCMxbXrVKzRf"),
            position_v1: pubkey!("8y9Rjqvb3UdT9SDfcM7v94Po2G36ndVM3pe1gLpGLRhx"),
            lb_pair: pubkey!("FoSDw2L5DmTuQTFe55gWPDXf88euaxAEKFre74CnvQbX"),
            bin_array_lower: pubkey!("6XnihiWoRfgFUncqAYeajEFEdu5m5bpY9kA39goDuDk9"),
            bin_array_upper: pubkey!("3kBRpy3Lj434ZXiCJ8u7TSRDcL2PKCGCCHrW4WuZWzbZ"),
            owner: pubkey!("DxocTN1otn4MUpJZED6XFbRxQmK5RrB3hqMYjoBZnXUb"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent_receiver: pubkey!("DxocTN1otn4MUpJZED6XFbRxQmK5RrB3hqMYjoBZnXUb"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/migrate_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            MigratePosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_remove_all_liquidity_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::RemoveAllLiquidity(RemoveAllLiquidity {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("9ryhChmSnFjN1Guow3wmCMfHcDqnPMNQWw1fd22re6B3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BaAcxMzRRLPsCeozVFxSBkNKRNtGxWsuwAJD97FwHrFS"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DSbDmhQ2g83dB7gxgEj2SU4YXmNTsuu1rJ7ZpYRD4FFN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8WDJNUUet9GKYp2LFZtMPn2szjADWqDBxh36xMzN34RG"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HvsP7Sc6FdcYQgjMsZFmgucfXWpHBV2Z4ZSxokGDmvw3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5F8BjK245zNk8gDZ4wfzqFk5zNKnfBZu1nQqxTf9bw3q"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6ugNo7XbU5RpaNVi6MMP8tMN85MmNeXFZmcPyPvWpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4ogZsCfAmNAnNyWRqDpny2Fx7q2hxXsY64eHtmqmT9eH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7a4ucyBoMGmse63FQ8irJ7JskAtHne9Tanidf3uWnXKD"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GiLZMHHhLG34bv5uhDimeycrdGKhBqUc4W7rzFcPHT5o"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = RemoveAllLiquidityInstructionAccounts {
            position: pubkey!("9ryhChmSnFjN1Guow3wmCMfHcDqnPMNQWw1fd22re6B3"),
            lb_pair: pubkey!("BaAcxMzRRLPsCeozVFxSBkNKRNtGxWsuwAJD97FwHrFS"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("DSbDmhQ2g83dB7gxgEj2SU4YXmNTsuu1rJ7ZpYRD4FFN"),
            user_token_y: pubkey!("8WDJNUUet9GKYp2LFZtMPn2szjADWqDBxh36xMzN34RG"),
            reserve_x: pubkey!("HvsP7Sc6FdcYQgjMsZFmgucfXWpHBV2Z4ZSxokGDmvw3"),
            reserve_y: pubkey!("5F8BjK245zNk8gDZ4wfzqFk5zNKnfBZu1nQqxTf9bw3q"),
            token_x_mint: pubkey!("6ugNo7XbU5RpaNVi6MMP8tMN85MmNeXFZmcPyPvWpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            bin_array_lower: pubkey!("4ogZsCfAmNAnNyWRqDpny2Fx7q2hxXsY64eHtmqmT9eH"),
            bin_array_upper: pubkey!("7a4ucyBoMGmse63FQ8irJ7JskAtHne9Tanidf3uWnXKD"),
            sender: pubkey!("GiLZMHHhLG34bv5uhDimeycrdGKhBqUc4W7rzFcPHT5o"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/remove_all_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            RemoveAllLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_remove_liquidity_by_range_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::RemoveLiquidityByRange(RemoveLiquidityByRange {
            bps_to_remove: 10000,
            from_bin_id: -239,
            to_bin_id: -171,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("8q5pnyLyZDywab5ELtfEmjeukN6QjAiLyVjA1jtz1Hjo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HZErmEhFdPtEv8miyRNJ6YYDCJVTUDQ8vb6b9gYV1pAY"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8soE8tjnUX9hW6pacq8Fhmb14zaxmxYZrvd4AHttGpN5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5gACEv1QcqviCgKDC3JrWQbvgfFv9rqYE4L3y6SnZyXg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DScYDgLUTZLoB8D1YQUHS8btV1DcddyArwMwpZvaX9F2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8F8GPCCPsqDiwshYthe4NZjBCNYmZFx2abmbFRni2dZu"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CniPCE4b3s8gSUPhUiyMjXnytrEqUrMfSsnbBjLCpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("bKnQuoFJkhJvaTfEtvv1bQxJwnU1Qoor3C8nZ2x9VZR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("567njsp2GVpy62Q6WsSeMLGTNFfChe8SNXbe1egew5FM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Hisq5w4hewLPPXvDHkVwDzcweq8XRM5a1fphKWXrLeYL"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = RemoveLiquidityByRangeInstructionAccounts {
            position: pubkey!("8q5pnyLyZDywab5ELtfEmjeukN6QjAiLyVjA1jtz1Hjo"),
            lb_pair: pubkey!("HZErmEhFdPtEv8miyRNJ6YYDCJVTUDQ8vb6b9gYV1pAY"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("8soE8tjnUX9hW6pacq8Fhmb14zaxmxYZrvd4AHttGpN5"),
            user_token_y: pubkey!("5gACEv1QcqviCgKDC3JrWQbvgfFv9rqYE4L3y6SnZyXg"),
            reserve_x: pubkey!("DScYDgLUTZLoB8D1YQUHS8btV1DcddyArwMwpZvaX9F2"),
            reserve_y: pubkey!("8F8GPCCPsqDiwshYthe4NZjBCNYmZFx2abmbFRni2dZu"),
            token_x_mint: pubkey!("CniPCE4b3s8gSUPhUiyMjXnytrEqUrMfSsnbBjLCpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            bin_array_lower: pubkey!("bKnQuoFJkhJvaTfEtvv1bQxJwnU1Qoor3C8nZ2x9VZR"),
            bin_array_upper: pubkey!("567njsp2GVpy62Q6WsSeMLGTNFfChe8SNXbe1egew5FM"),
            sender: pubkey!("Hisq5w4hewLPPXvDHkVwDzcweq8XRM5a1fphKWXrLeYL"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/remove_liquidity_by_range_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            RemoveLiquidityByRange::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_remove_liquidity_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::RemoveLiquidity(RemoveLiquidity {
            bin_liquidity_removal: vec![
                BinLiquidityReduction {
                    bin_id: 1687,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1688,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1689,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1690,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1691,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1692,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1693,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1694,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1695,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1696,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1697,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1698,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1699,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1700,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1701,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1702,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1703,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1704,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1705,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1706,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1707,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1708,
                    bps_to_remove: 1,
                },
                BinLiquidityReduction {
                    bin_id: 1709,
                    bps_to_remove: 1,
                },
            ],
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("GTg4upAYuAqXnSAdSGBbcgR7mbxcLwp8pxZp5iQe1tLJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BoeMUkCLHchTD31HdXsbDExuZZfcUppSLpYtV3LZTH6U"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6MZw9Y8FiGGjnL4DVXS812w9t9p5B9tK5xcrCWu6p2Ev"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H1HwdDkL5qHM8AZR9tBWYZQDTt9wwigKqCdF4KWuX7zP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("93d6ukn24o1xMcMDip2SACKG8GbvhGUZim1e3ZEcQVm2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CodroyzrRNvc5kHRoAQYjpVSr1jA9fLcUWVFouiuWGsD"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CzeWb8k7wDfubQz6McUcYkZuM6UQ7y2XJ6hfYBRCqBea"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5Kp3m4p7QxN629DA8wcHuLrHkecxEmdGFLRcB9PD4HAa"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("HdZCvCH4qwUqfy5YukMyyy5gYDhtmMWK7GvqEbLVsSWj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = RemoveLiquidityInstructionAccounts {
            position: pubkey!("GTg4upAYuAqXnSAdSGBbcgR7mbxcLwp8pxZp5iQe1tLJ"),
            lb_pair: pubkey!("BoeMUkCLHchTD31HdXsbDExuZZfcUppSLpYtV3LZTH6U"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("6MZw9Y8FiGGjnL4DVXS812w9t9p5B9tK5xcrCWu6p2Ev"),
            user_token_y: pubkey!("H1HwdDkL5qHM8AZR9tBWYZQDTt9wwigKqCdF4KWuX7zP"),
            reserve_x: pubkey!("93d6ukn24o1xMcMDip2SACKG8GbvhGUZim1e3ZEcQVm2"),
            reserve_y: pubkey!("CodroyzrRNvc5kHRoAQYjpVSr1jA9fLcUWVFouiuWGsD"),
            token_x_mint: pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            bin_array_lower: pubkey!("CzeWb8k7wDfubQz6McUcYkZuM6UQ7y2XJ6hfYBRCqBea"),
            bin_array_upper: pubkey!("5Kp3m4p7QxN629DA8wcHuLrHkecxEmdGFLRcB9PD4HAa"),
            sender: pubkey!("HdZCvCH4qwUqfy5YukMyyy5gYDhtmMWK7GvqEbLVsSWj"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/remove_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            RemoveLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_exact_out_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::SwapExactOut(SwapExactOut {
            max_in_amount: 18446744073709551615,
            out_amount: 5950000,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("8gJ7UWboMeQ6z6AQwFP3cAZwSYG8udVS2UesyCbH79r7"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("chM5ZB1uPZxvJJAK4D1Z4KHAYjWKvwuQTy6fFAeWQ1T"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FGFaiYjXTVuLsKvzn6ueckraNTeqUGHeYqrQPQCpd7kH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BF9S5Kvygv3Qf4aSnjyG98aoij11k3yiSLBHcxTa53h3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("He2uhHBU7uE9BiSe2QLRzQd9U3fWwjZY5anmC9k84nzd"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DoBNfRox1ZjEsZq6QPY4jpN8hN4Fu9JVkAxJQro164VR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BF9S5Kvygv3Qf4aSnjyG98aoij11k3yiSLBHcxTa53h3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HuTshmtwcQkWBLzgW3m4uwcmik7Lmz4YFpYcTqMJpXiP"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FXrrHPdrSodRZWXNMf8DRXCogkX1vSzLWNS7zonYHiSw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8A6sET38nPh6VX6bs9df6HjcvvQ14bNJvxhGRJ5eHGbh"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapExactOutInstructionAccounts {
            lb_pair: pubkey!("8gJ7UWboMeQ6z6AQwFP3cAZwSYG8udVS2UesyCbH79r7"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            reserve_x: pubkey!("chM5ZB1uPZxvJJAK4D1Z4KHAYjWKvwuQTy6fFAeWQ1T"),
            reserve_y: pubkey!("FGFaiYjXTVuLsKvzn6ueckraNTeqUGHeYqrQPQCpd7kH"),
            user_token_in: pubkey!("BF9S5Kvygv3Qf4aSnjyG98aoij11k3yiSLBHcxTa53h3"),
            user_token_out: pubkey!("He2uhHBU7uE9BiSe2QLRzQd9U3fWwjZY5anmC9k84nzd"),
            token_x_mint: pubkey!("So11111111111111111111111111111111111111112"),
            token_y_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            oracle: pubkey!("DoBNfRox1ZjEsZq6QPY4jpN8hN4Fu9JVkAxJQro164VR"),
            host_fee_in: pubkey!("BF9S5Kvygv3Qf4aSnjyG98aoij11k3yiSLBHcxTa53h3"),
            user: pubkey!("HuTshmtwcQkWBLzgW3m4uwcmik7Lmz4YFpYcTqMJpXiP"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/swap_exact_out_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapExactOut::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::Swap(Swap {
            amount_in: 60165522,
            min_amount_out: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FwwjuDSCZGRqB4ngwZd4zqjvLdbykU7PQaFTzocSAxmt"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CWBe4vsnqxK87NzFRnhoXH4Pi2mDhLAfpmZZVw7rmtDr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7u9JUmEDLbF4FAQmybpbYwUXHtSFP7HsESMsop6yHtBR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GjxJ1KCKU6tduWpgJKXA81spVK5mHZHbXNFTSvFELqgN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7sfqR1grUb5jCTrpYm31cc1HA4quapJ8ur8ZdkUwUqtS"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cy1GS2FqefgaMbi45UunrUzin1rfEmTUYnomddzBpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GJiUHhpg1E2asD4iHhVd8sGPryxg8uiyjmRygieRNkVr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CfKSQ1DbJa79XGp3fieJJwnskHBbWkgrpC3DmL6rTzQH"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9d9WZ5dUB8AfQ7LkBoe4NjKwZRDjFaBhzRvw5DoWtTk6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HiSfB3uFcPENZABoE94VJ8YzhdeuQucFT1N5dbfquWeQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GXx6rxy44GrxWYqKmiLUgoAw5WPX6zaK7RzhifJzH29M"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapInstructionAccounts {
            lb_pair: pubkey!("FwwjuDSCZGRqB4ngwZd4zqjvLdbykU7PQaFTzocSAxmt"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            reserve_x: pubkey!("CWBe4vsnqxK87NzFRnhoXH4Pi2mDhLAfpmZZVw7rmtDr"),
            reserve_y: pubkey!("7u9JUmEDLbF4FAQmybpbYwUXHtSFP7HsESMsop6yHtBR"),
            user_token_in: pubkey!("GjxJ1KCKU6tduWpgJKXA81spVK5mHZHbXNFTSvFELqgN"),
            user_token_out: pubkey!("7sfqR1grUb5jCTrpYm31cc1HA4quapJ8ur8ZdkUwUqtS"),
            token_x_mint: pubkey!("Cy1GS2FqefgaMbi45UunrUzin1rfEmTUYnomddzBpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            oracle: pubkey!("GJiUHhpg1E2asD4iHhVd8sGPryxg8uiyjmRygieRNkVr"),
            host_fee_in: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user: pubkey!("CfKSQ1DbJa79XGp3fieJJwnskHBbWkgrpC3DmL6rTzQH"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            Swap::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_fees_and_rewards_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::UpdateFeesAndRewards(UpdateFeesAndRewards {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("xVntejWTytykHTM33wHCfogpQRcy7mDAnH3U5b7X8eD"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6cDtJkcJKFEsGDhptmgvy3XtbwyRqnW3GoGcmnwVzJ7U"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3bi5tFzspV17UGEKra1HReFN6doQ2iSsyvWY8XzQGaoc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5vfo4AvbiE4T1VBtqYhv8YAA92fERDevKCGHj8sVyYuQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6jW5kGzAQ6dM3CUGF7k1uTRBb8DFm9whv52UTEPFpgF8"),
                false,
            ),
        ];
        let expected_arranged_accounts = UpdateFeesAndRewardsInstructionAccounts {
            position: pubkey!("xVntejWTytykHTM33wHCfogpQRcy7mDAnH3U5b7X8eD"),
            lb_pair: pubkey!("6cDtJkcJKFEsGDhptmgvy3XtbwyRqnW3GoGcmnwVzJ7U"),
            bin_array_lower: pubkey!("3bi5tFzspV17UGEKra1HReFN6doQ2iSsyvWY8XzQGaoc"),
            bin_array_upper: pubkey!("5vfo4AvbiE4T1VBtqYhv8YAA92fERDevKCGHj8sVyYuQ"),
            owner: pubkey!("6jW5kGzAQ6dM3CUGF7k1uTRBb8DFm9whv52UTEPFpgF8"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/update_fees_and_rewards_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdateFeesAndRewards::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_position_if_empty_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClosePositionIfEmpty(ClosePositionIfEmpty {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5zasWBT9q3SxnY5nnCYXX6f95f35Jd99f7b6e41Ld8iQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4R1YbARsv2PjznvXFfPG9tGzHXZnDjP9gHhGQuKGnXi1"),
                true,
            ),
            AccountMeta::new(
                pubkey!("4R1YbARsv2PjznvXFfPG9tGzHXZnDjP9gHhGQuKGnXi1"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClosePositionIfEmptyInstructionAccounts {
            position: pubkey!("5zasWBT9q3SxnY5nnCYXX6f95f35Jd99f7b6e41Ld8iQ"),
            sender: pubkey!("4R1YbARsv2PjznvXFfPG9tGzHXZnDjP9gHhGQuKGnXi1"),
            rent_receiver: pubkey!("4R1YbARsv2PjznvXFfPG9tGzHXZnDjP9gHhGQuKGnXi1"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_position_if_empty_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePositionIfEmpty::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_position2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClosePosition2(ClosePosition2 {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("DYrwi3FjL9i87kRPq9Sex7i1qVBPTqwvjjCLJmR2fRGz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3Fki2gTbdT3YQQzGkE3nJA2djU3ijsfCw7EYprqroBGP"),
                true,
            ),
            AccountMeta::new(
                pubkey!("3Fki2gTbdT3YQQzGkE3nJA2djU3ijsfCw7EYprqroBGP"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClosePosition2InstructionAccounts {
            position: pubkey!("DYrwi3FjL9i87kRPq9Sex7i1qVBPTqwvjjCLJmR2fRGz"),
            sender: pubkey!("3Fki2gTbdT3YQQzGkE3nJA2djU3ijsfCw7EYprqroBGP"),
            rent_receiver: pubkey!("3Fki2gTbdT3YQQzGkE3nJA2djU3ijsfCw7EYprqroBGP"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_position2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePosition2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_migrate_bin_array_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::MigrateBinArray(MigrateBinArray {});
        let expected_accounts = vec![AccountMeta::new_readonly(
            pubkey!("zNLqkEXHxVDAbyjSSA8E17e1ksw5XEGrAA2Jha6e3FH"),
            false,
        )];
        let expected_arranged_accounts = MigrateBinArrayInstructionAccounts {
            lb_pair: pubkey!("zNLqkEXHxVDAbyjSSA8E17e1ksw5XEGrAA2Jha6e3FH"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/migrate_bin_array_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            MigrateBinArray::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_remove_liquidity_by_range2_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::RemoveLiquidityByRange2(RemoveLiquidityByRange2 {
                bps_to_remove: 10000,
                from_bin_id: -3567,
                remaining_accounts_info: RemainingAccountsInfo {
                    slices: vec![
                        RemainingAccountsSlice {
                            accounts_type: AccountsType::TransferHookX,
                            length: 0,
                        },
                        RemainingAccountsSlice {
                            accounts_type: AccountsType::TransferHookY,
                            length: 0,
                        },
                    ],
                },
                to_bin_id: -3499,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Cz3nviU2HQofE4cnZGDPTgt9q3T6eMEQbmuaGZB9noXg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2MEBV4deAZ8P2E9ZwKZji9RmQFh4y266hoDh7zRcGjcf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3EY5CVGgCKc5EEfdv5b1aVe2GMevERW69644nLJ4Er6K"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7urKMjRZKVt5z2vKRt6tV1xHFQWH3227g6KngnhkFidL"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = RemoveLiquidityByRange2InstructionAccounts {
            position: pubkey!("Cz3nviU2HQofE4cnZGDPTgt9q3T6eMEQbmuaGZB9noXg"),
            lb_pair: pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("2MEBV4deAZ8P2E9ZwKZji9RmQFh4y266hoDh7zRcGjcf"),
            user_token_y: pubkey!("3EY5CVGgCKc5EEfdv5b1aVe2GMevERW69644nLJ4Er6K"),
            reserve_x: pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
            reserve_y: pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
            token_x_mint: pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            sender: pubkey!("7urKMjRZKVt5z2vKRt6tV1xHFQWH3227g6KngnhkFidL"),
            token_x_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/remove_liquidity_by_range2_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            RemoveLiquidityByRange2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_remove_liquidity2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::RemoveLiquidity2(RemoveLiquidity2 {
            bin_liquidity_removal: vec![BinLiquidityReduction {
                bin_id: -560,
                bps_to_remove: 10000,
            }],
            remaining_accounts_info: RemainingAccountsInfo {
                slices: vec![
                    RemainingAccountsSlice {
                        accounts_type: AccountsType::TransferHookX,
                        length: 0,
                    },
                    RemainingAccountsSlice {
                        accounts_type: AccountsType::TransferHookY,
                        length: 0,
                    },
                ],
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("3gWKqojAWeJRgdSe3bPyzNwjAdjeyaYJuraTEbFkEn9x"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FvgJNjvU1swX8roYZpJ3BpjwVDqcrM2tEt18nhTNLCfd"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Eurd4K4mrUypp3EhXxvBPhUm3e9WLoLyAwkzNDXv4shW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EgJsiHQDTfxXH6EkNTqLE6jd1H9Q1GZJPRMzVDfZKPGn"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2u8ZyhXETPLkTGFhgtFm3zjpD1PosF7VySFoc9PdQQ7K"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4UN3pAUqsM1X73zPTG1iLaxAWxy5vzg8kspBbqfPEq2n"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("dHNnE3a7aJpwmfGhh5QssrPUj4gAMNdkaCvUwyTwest"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9NQ5D4CjcTa4AsXZ37j7gAjEJiqjSgn5SSTftryMwest"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = RemoveLiquidity2InstructionAccounts {
            position: pubkey!("3gWKqojAWeJRgdSe3bPyzNwjAdjeyaYJuraTEbFkEn9x"),
            lb_pair: pubkey!("FvgJNjvU1swX8roYZpJ3BpjwVDqcrM2tEt18nhTNLCfd"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user_token_x: pubkey!("Eurd4K4mrUypp3EhXxvBPhUm3e9WLoLyAwkzNDXv4shW"),
            user_token_y: pubkey!("EgJsiHQDTfxXH6EkNTqLE6jd1H9Q1GZJPRMzVDfZKPGn"),
            reserve_x: pubkey!("2u8ZyhXETPLkTGFhgtFm3zjpD1PosF7VySFoc9PdQQ7K"),
            reserve_y: pubkey!("4UN3pAUqsM1X73zPTG1iLaxAWxy5vzg8kspBbqfPEq2n"),
            token_x_mint: pubkey!("dHNnE3a7aJpwmfGhh5QssrPUj4gAMNdkaCvUwyTwest"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            sender: pubkey!("9NQ5D4CjcTa4AsXZ37j7gAjEJiqjSgn5SSTftryMwest"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/remove_liquidity2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            RemoveLiquidity2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_activation_point_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::SetActivationPoint(SetActivationPoint {
            activation_point: 1748178300,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
        ];
        let expected_arranged_accounts = SetActivationPointInstructionAccounts {
            lb_pair: pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
            admin: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/set_activation_point_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetActivationPoint::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_pair_status_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::SetPairStatus(SetPairStatus { status: 1 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
        ];
        let expected_arranged_accounts = SetPairStatusInstructionAccounts {
            lb_pair: pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
            admin: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/set_pair_status_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetPairStatus::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_pair_status_permissionless_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::SetPairStatusPermissionless(SetPairStatusPermissionless {
                status: 0,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("C6881xvUWFRBFgDX8TrdiuQzHf6gBrmrh6DRzGa5B3Ft"),
                false,
            ),
            AccountMeta::new(
                pubkey!("E4PmHc6Hmsf5soqFsz9ZjsGwTwNjkWd9pw5MZfRctP6e"),
                true,
            ),
        ];
        let expected_arranged_accounts = SetPairStatusPermissionlessInstructionAccounts {
            lb_pair: pubkey!("C6881xvUWFRBFgDX8TrdiuQzHf6gBrmrh6DRzGa5B3Ft"),
            creator: pubkey!("E4PmHc6Hmsf5soqFsz9ZjsGwTwNjkWd9pw5MZfRctP6e"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/set_pair_status_permissionless_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetPairStatusPermissionless::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_pre_activation_duration_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::SetPreActivationDuration(SetPreActivationDuration {
                pre_activation_duration: 3600,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
        ];
        let expected_arranged_accounts = SetPreActivationDurationInstructionAccounts {
            lb_pair: pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
            creator: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/set_pre_activation_duration_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetPreActivationDuration::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_pre_activation_swap_address_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::SetPreActivationSwapAddress(SetPreActivationSwapAddress {
                pre_activation_swap_address: pubkey!(
                    "8gCqHrCcP5PxUf2fPR1nre5B9t2HjWQu6StujDCor4oW"
                ),
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
        ];
        let expected_arranged_accounts = SetPreActivationSwapAddressInstructionAccounts {
            lb_pair: pubkey!("Ex3x6Two22ypWzvfXM8hdeJq6CWGG74k7wi4ZSafeyGj"),
            creator: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/set_pre_activation_swap_address_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetPreActivationSwapAddress::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_exact_out2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::SwapExactOut2(SwapExactOut2 {
            max_in_amount: 3826290145,
            out_amount: 4891000000000,
            remaining_accounts_info: RemainingAccountsInfo { slices: vec![] },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GShxvtESt69624EjwvbLBTHmQYcBUn6KKce3Wm7fCqmL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8y1KDhsqTqi9poaExmnGJ9mUaNa8K1Y5fi6M1LsXZwYi"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AFH1UXkECQwYoWkkCSydxU8UGciH8jxqB9EebV1NJVHs"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("G3ztVULQTspy6wBwKVtGHMfX3GBi1FtsfQHzz3RU2bon"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapExactOut2InstructionAccounts {
            lb_pair: pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            reserve_x: pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
            reserve_y: pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
            user_token_in: pubkey!("GShxvtESt69624EjwvbLBTHmQYcBUn6KKce3Wm7fCqmL"),
            user_token_out: pubkey!("8y1KDhsqTqi9poaExmnGJ9mUaNa8K1Y5fi6M1LsXZwYi"),
            token_x_mint: pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            oracle: pubkey!("AFH1UXkECQwYoWkkCSydxU8UGciH8jxqB9EebV1NJVHs"),
            host_fee_in: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user: pubkey!("G3ztVULQTspy6wBwKVtGHMfX3GBi1FtsfQHzz3RU2bon"),
            token_x_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/swap_exact_out2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapExactOut2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_with_price_impact2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::SwapWithPriceImpact2(SwapWithPriceImpact2 {
            active_id: Some(-533),
            amount_in: 58823,
            max_price_impact_bps: 1000,
            remaining_accounts_info: RemainingAccountsInfo { slices: vec![] },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("D4ARLASjg2Suy7M6vQtHAvx5NZEECuR7SFzWXhp41hfY"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6RPeoQaz4aCCLY92qiF22eymiu8HtXfi6o96PCHro85R"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Eq7wrmK1m4DpvihrnGwzBPccbsdP8u4nxi3obZ4325MC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2YGA6ogjCtCJGg5HaPpreiH7qeiQEWsboiV1UhNVk8LB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("J1aRY8W75LHRoTrQjKri9ZrnZWVwH6b9pMYAJYLCxQk1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7VnT8zHzorYS92snKC4CZU2veigEVnVVBSxTw7G1pump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CS4LtXuwrM5Z7UcS1GsA9KdSzo6d2yzsaVcJ3YkUvwzj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6ZgXjkSmcAqV2ELiRv33D16CQV5GAkFpL8jmuBg5QfPY"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapWithPriceImpact2InstructionAccounts {
            lb_pair: pubkey!("D4ARLASjg2Suy7M6vQtHAvx5NZEECuR7SFzWXhp41hfY"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            reserve_x: pubkey!("6RPeoQaz4aCCLY92qiF22eymiu8HtXfi6o96PCHro85R"),
            reserve_y: pubkey!("Eq7wrmK1m4DpvihrnGwzBPccbsdP8u4nxi3obZ4325MC"),
            user_token_in: pubkey!("2YGA6ogjCtCJGg5HaPpreiH7qeiQEWsboiV1UhNVk8LB"),
            user_token_out: pubkey!("J1aRY8W75LHRoTrQjKri9ZrnZWVwH6b9pMYAJYLCxQk1"),
            token_x_mint: pubkey!("7VnT8zHzorYS92snKC4CZU2veigEVnVVBSxTw7G1pump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            oracle: pubkey!("CS4LtXuwrM5Z7UcS1GsA9KdSzo6d2yzsaVcJ3YkUvwzj"),
            host_fee_in: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user: pubkey!("6ZgXjkSmcAqV2ELiRv33D16CQV5GAkFpL8jmuBg5QfPY"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/swap_with_price_impact2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapWithPriceImpact2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::Swap2(Swap2 {
            amount_in: 217206901,
            min_amount_out: 0,
            remaining_accounts_info: RemainingAccountsInfo { slices: vec![] },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5cuy7pMhTPhVZN9xuhgSbykRb986siGJb6vnEtkuBrSU"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9wbTcHco8daQYxVPWn1eqDQe2YPY3ak3gPfQuYAcZ4PJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Cpwo6h4koL8pC87R17g1dX8zfEQ6Pnv3AHXGPpNJqBuf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CPVAAuzZGX4nBk1o42qLVSZf9PQEVGdh6wcpxw8bF6Ar"),
                false,
            ),
            AccountMeta::new(
                pubkey!("53mNdjYekY37E5JQ9vXB57seaTxt2j6degVmcXLVr77r"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2NBaawB9aeYocWvyiECcDxSSwcyJd1B8oaHzpyEFbapc"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AmZaQwdMRKNC5JzRUujZfrVFyjmoUuvrRmc6iKnQHEv6"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = Swap2InstructionAccounts {
            lb_pair: pubkey!("5cuy7pMhTPhVZN9xuhgSbykRb986siGJb6vnEtkuBrSU"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            reserve_x: pubkey!("9wbTcHco8daQYxVPWn1eqDQe2YPY3ak3gPfQuYAcZ4PJ"),
            reserve_y: pubkey!("Cpwo6h4koL8pC87R17g1dX8zfEQ6Pnv3AHXGPpNJqBuf"),
            user_token_in: pubkey!("CPVAAuzZGX4nBk1o42qLVSZf9PQEVGdh6wcpxw8bF6Ar"),
            user_token_out: pubkey!("53mNdjYekY37E5JQ9vXB57seaTxt2j6degVmcXLVr77r"),
            token_x_mint: pubkey!("27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4"),
            token_y_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            oracle: pubkey!("2NBaawB9aeYocWvyiECcDxSSwcyJd1B8oaHzpyEFbapc"),
            host_fee_in: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            user: pubkey!("AmZaQwdMRKNC5JzRUujZfrVFyjmoUuvrRmc6iKnQHEv6"),
            token_x_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_y_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap2_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            Swap2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_base_fee_parameters_ix() {
        // Arrange
        let expected_ix =
            MeteoraDlmmInstruction::UpdateBaseFeeParameters(UpdateBaseFeeParameters {
                fee_parameter: BaseFeeParameter {
                    base_factor: 3125,
                    base_fee_power_factor: 0,
                    protocol_share: 2000,
                },
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("4uPKbdiUTdLWjTMGwUQb2tiw1HYhy6FLSmZU5tx85vBZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = UpdateBaseFeeParametersInstructionAccounts {
            lb_pair: pubkey!("4uPKbdiUTdLWjTMGwUQb2tiw1HYhy6FLSmZU5tx85vBZ"),
            admin: pubkey!("5unTfT2kssBuNvHPY6LbJfJpLqEcdMxGYLWHwShaeTLi"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/update_base_fee_parameters_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdateBaseFeeParameters::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_claim_fee2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClaimFee2(ClaimFee2 {
            max_bin_id: -3601,
            min_bin_id: -3669,
            remaining_accounts_info: RemainingAccountsInfo {
                slices: vec![
                    RemainingAccountsSlice {
                        accounts_type: AccountsType::TransferHookX,
                        length: 0,
                    },
                    RemainingAccountsSlice {
                        accounts_type: AccountsType::TransferHookY,
                        length: 0,
                    },
                ],
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
                false,
            ),
            AccountMeta::new(
                pubkey!("JBgwd6QqY1QANj5mY6ccBRuVxsfvoJkLVeSWDxvNT2bw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ACwQDp4FqpvMaYbCDQomDPYTawGbfi49cdZqUABjCqTm"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CgB7w527GSoFjQc5p8E3jn93eVj7ByNb6TjTKArU45vs"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EZizF1NkMtyQavZy4E5NK6SN8wnJFdukgPhMRVbpPZ6i"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClaimFee2InstructionAccounts {
            lb_pair: pubkey!("AjM8Qn62EhR4ikJ1rvyeezB1NyvrSsb4zwJiFUFs9ycs"),
            position: pubkey!("JBgwd6QqY1QANj5mY6ccBRuVxsfvoJkLVeSWDxvNT2bw"),
            sender: pubkey!("ACwQDp4FqpvMaYbCDQomDPYTawGbfi49cdZqUABjCqTm"),
            reserve_x: pubkey!("DfWWLJvVHDM9byp6y7Rpw5Rx4mGizSwB5GEoUMegi3z8"),
            reserve_y: pubkey!("6qxaasNgXsfVp8tKkoJavp29hZYiDrcEirsS3oAsYCLc"),
            user_token_x: pubkey!("CgB7w527GSoFjQc5p8E3jn93eVj7ByNb6TjTKArU45vs"),
            user_token_y: pubkey!("EZizF1NkMtyQavZy4E5NK6SN8wnJFdukgPhMRVbpPZ6i"),
            token_x_mint: pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            token_program_x: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_program_y: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/claim_fee2_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClaimFee2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_claim_reward2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::ClaimReward2(ClaimReward2 {
            max_bin_id: 4,
            min_bin_id: 4,
            remaining_accounts_info: RemainingAccountsInfo {
                slices: vec![RemainingAccountsSlice {
                    accounts_type: AccountsType::TransferHookReward,
                    length: 0,
                }],
            },
            reward_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("4wM3eJMduZBFytW6VqV5DC2CaSovRrM2RJG8bJkroqLD"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5qeKgwuWULUacKHQxeTyJH1hzr4xXEDS6PwCfuSC8ntf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GKELxnW2LL2aHXe61pCoCFyko2UjZKKwRgHg8kCyGNzv"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DLuW6nRywCBG5BssbaFpLfLsMSoXoXRiuzUtH6VJb12c"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("AuQaustGiaqxRvj2gtCdrd22PBzTn8kM3kEPEkZCtuDw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Co1sWgvQ3KvTgsGcHZy9YNA9s7oNjDL6xH2RNJbWxDPC"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClaimReward2InstructionAccounts {
            lb_pair: pubkey!("4wM3eJMduZBFytW6VqV5DC2CaSovRrM2RJG8bJkroqLD"),
            position: pubkey!("5qeKgwuWULUacKHQxeTyJH1hzr4xXEDS6PwCfuSC8ntf"),
            sender: pubkey!("GKELxnW2LL2aHXe61pCoCFyko2UjZKKwRgHg8kCyGNzv"),
            reward_vault: pubkey!("DLuW6nRywCBG5BssbaFpLfLsMSoXoXRiuzUtH6VJb12c"),
            reward_mint: pubkey!("AuQaustGiaqxRvj2gtCdrd22PBzTn8kM3kEPEkZCtuDw"),
            user_token_account: pubkey!("Co1sWgvQ3KvTgsGcHZy9YNA9s7oNjDL6xH2RNJbWxDPC"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/claim_reward2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClaimReward2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_lb_pair2_ix() {
        // Arrange
        let expected_ix = MeteoraDlmmInstruction::InitializeLbPair2(InitializeLbPair2 {
            params: InitializeLbPair2Params {
                active_id: -341,
                padding: [0; 96],
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AythiR24hnGBjPq6FK6ADiVGqXHTbBpydkBPfNC1ndgg"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("B7zNKphr8fjczB71oi9uF9pCd5XSNJvBn78TVF7kpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7PMC89FyUXvjoK5V1ZZJQWWGXbXt6h9n3qS4dBVg7Yn4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2EPCtbGKWKeFnpdevbRAthgGqpA6WRqYMSjfEBMZH5oZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Dnfpn4ZCRSCxtoYFcbNowGP215dsed2N45SS693GTY5W"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CrZUmJzkSs4TWg8GpCq5UGRX4ryRYHYYVQQ4dNMYo1GW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EaeLcEeKz3XoN9xn99kjJhmWZzdVx2dJ4uYxqRYxRWhG"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializeLbPair2InstructionAccounts {
            lb_pair: pubkey!("AythiR24hnGBjPq6FK6ADiVGqXHTbBpydkBPfNC1ndgg"),
            bin_array_bitmap_extension: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_mint_x: pubkey!("B7zNKphr8fjczB71oi9uF9pCd5XSNJvBn78TVF7kpump"),
            token_mint_y: pubkey!("So11111111111111111111111111111111111111112"),
            reserve_x: pubkey!("7PMC89FyUXvjoK5V1ZZJQWWGXbXt6h9n3qS4dBVg7Yn4"),
            reserve_y: pubkey!("2EPCtbGKWKeFnpdevbRAthgGqpA6WRqYMSjfEBMZH5oZ"),
            oracle: pubkey!("Dnfpn4ZCRSCxtoYFcbNowGP215dsed2N45SS693GTY5W"),
            preset_parameter: pubkey!("CrZUmJzkSs4TWg8GpCq5UGRX4ryRYHYYVQQ4dNMYo1GW"),
            funder: pubkey!("EaeLcEeKz3XoN9xn99kjJhmWZzdVx2dJ4uYxqRYxRWhG"),
            token_badge_x: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_badge_y: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            token_program_x: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_y: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            event_authority: pubkey!("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
            program: pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_lb_pair2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeLbPair2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
