use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd4dcd5f1be059df")]
pub struct InitializeTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeTokenBadgeInstructionAccounts {
    pub token_mint: solana_pubkey::Pubkey,
    pub token_badge: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenBadge {
    type ArrangedAccounts = InitializeTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_mint, token_badge, admin, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTokenBadgeInstructionAccounts {
            token_mint: token_mint.pubkey,
            token_badge: token_badge.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
