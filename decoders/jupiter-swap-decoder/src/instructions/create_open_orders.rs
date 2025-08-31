use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe5c2d4ac080a8693")]
pub struct CreateOpenOrders {}

pub struct CreateOpenOrdersInstructionAccounts {
    pub open_orders: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrders {
    type ArrangedAccounts = CreateOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [open_orders, payer, dex_program, system_program, rent, market, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateOpenOrdersInstructionAccounts {
            open_orders: open_orders.pubkey,
            payer: payer.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            market: market.pubkey,
        })
    }
}
