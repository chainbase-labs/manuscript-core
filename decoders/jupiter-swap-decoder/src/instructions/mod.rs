use crate::PROGRAM_ID;

use super::JupiterSwapDecoder;
pub mod claim;
pub mod claim_token;
pub mod create_open_orders;
pub mod create_program_open_orders;
pub mod create_token_ledger;
pub mod exact_out_route;
pub mod fee_event;
pub mod route;
pub mod route_with_token_ledger;
pub mod set_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod shared_accounts_route;
pub mod shared_accounts_route_with_token_ledger;
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
pub enum JupiterSwapInstruction {
    Claim(claim::Claim),
    ClaimToken(claim_token::ClaimToken),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
    CreateProgramOpenOrders(create_program_open_orders::CreateProgramOpenOrders),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    ExactOutRoute(exact_out_route::ExactOutRoute),
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    SharedAccountsExactOutRoute(shared_accounts_exact_out_route::SharedAccountsExactOutRoute),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(
        shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
    ),
    FeeEvent(fee_event::FeeEvent),
    SwapEvent(swap_event::SwapEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for JupiterSwapDecoder {
    type InstructionType = JupiterSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            JupiterSwapInstruction::Claim => claim::Claim,
            JupiterSwapInstruction::ClaimToken => claim_token::ClaimToken,
            JupiterSwapInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
            JupiterSwapInstruction::CreateProgramOpenOrders => create_program_open_orders::CreateProgramOpenOrders,
            JupiterSwapInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            JupiterSwapInstruction::ExactOutRoute => exact_out_route::ExactOutRoute,
            JupiterSwapInstruction::Route => route::Route,
            JupiterSwapInstruction::RouteWithTokenLedger => route_with_token_ledger::RouteWithTokenLedger,
            JupiterSwapInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterSwapInstruction::SharedAccountsExactOutRoute => shared_accounts_exact_out_route::SharedAccountsExactOutRoute,
            JupiterSwapInstruction::SharedAccountsRoute => shared_accounts_route::SharedAccountsRoute,
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger => shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
            JupiterSwapInstruction::FeeEvent => fee_event::FeeEvent,
            JupiterSwapInstruction::SwapEvent => swap_event::SwapEvent,
        )
    }
}
