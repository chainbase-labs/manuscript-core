use crate::PROGRAM_ID;

use super::OkxDexDecoder;
pub mod commission_sol_from_swap;
pub mod commission_sol_proxy_swap;
pub mod commission_sol_swap;
pub mod commission_sol_swap2;
pub mod commission_spl_from_swap;
pub mod commission_spl_proxy_swap;
pub mod commission_spl_swap;
pub mod commission_spl_swap2;
pub mod from_swap_log;
pub mod proxy_swap;
pub mod swap;
pub mod swap2;
pub mod swap_event;

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
pub enum OkxDexInstruction {
    CommissionSolFromSwap(commission_sol_from_swap::CommissionSolFromSwap),
    CommissionSolProxySwap(commission_sol_proxy_swap::CommissionSolProxySwap),
    CommissionSolSwap(commission_sol_swap::CommissionSolSwap),
    CommissionSolSwap2(commission_sol_swap2::CommissionSolSwap2),
    CommissionSplFromSwap(commission_spl_from_swap::CommissionSplFromSwap),
    CommissionSplProxySwap(commission_spl_proxy_swap::CommissionSplProxySwap),
    CommissionSplSwap(commission_spl_swap::CommissionSplSwap),
    CommissionSplSwap2(commission_spl_swap2::CommissionSplSwap2),
    FromSwapLog(from_swap_log::FromSwapLog),
    ProxySwap(proxy_swap::ProxySwap),
    Swap(swap::Swap),
    Swap2(swap2::Swap2),
    SwapEvent(swap_event::SwapEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for OkxDexDecoder {
    type InstructionType = OkxDexInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            OkxDexInstruction::CommissionSolFromSwap => commission_sol_from_swap::CommissionSolFromSwap,
            OkxDexInstruction::CommissionSolProxySwap => commission_sol_proxy_swap::CommissionSolProxySwap,
            OkxDexInstruction::CommissionSolSwap => commission_sol_swap::CommissionSolSwap,
            OkxDexInstruction::CommissionSolSwap2 => commission_sol_swap2::CommissionSolSwap2,
            OkxDexInstruction::CommissionSplFromSwap => commission_spl_from_swap::CommissionSplFromSwap,
            OkxDexInstruction::CommissionSplProxySwap => commission_spl_proxy_swap::CommissionSplProxySwap,
            OkxDexInstruction::CommissionSplSwap => commission_spl_swap::CommissionSplSwap,
            OkxDexInstruction::CommissionSplSwap2 => commission_spl_swap2::CommissionSplSwap2,
            OkxDexInstruction::FromSwapLog => from_swap_log::FromSwapLog,
            OkxDexInstruction::ProxySwap => proxy_swap::ProxySwap,
            OkxDexInstruction::Swap => swap::Swap,
            OkxDexInstruction::Swap2 => swap2::Swap2,
            OkxDexInstruction::SwapEvent => swap_event::SwapEvent,
        )
    }
}
