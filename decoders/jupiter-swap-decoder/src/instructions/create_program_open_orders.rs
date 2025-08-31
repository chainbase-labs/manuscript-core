use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1ce22094bc8871ab")]
pub struct CreateProgramOpenOrders {
    pub id: u8,
}

pub struct CreateProgramOpenOrdersInstructionAccounts {
    pub open_orders: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProgramOpenOrders {
    type ArrangedAccounts = CreateProgramOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [open_orders, payer, program_authority, dex_program, system_program, rent, market, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateProgramOpenOrdersInstructionAccounts {
            open_orders: open_orders.pubkey,
            payer: payer.pubkey,
            program_authority: program_authority.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            market: market.pubkey,
        })
    }
}
