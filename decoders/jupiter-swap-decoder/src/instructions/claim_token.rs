use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x74ce1bbfa6130049")]
pub struct ClaimToken {
    pub id: u8,
}

pub struct ClaimTokenInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub wallet: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub program_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub associated_token_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimToken {
    type ArrangedAccounts = ClaimTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, wallet, program_authority, program_token_account, destination_token_account, mint, associated_token_token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimTokenInstructionAccounts {
            payer: payer.pubkey,
            wallet: wallet.pubkey,
            program_authority: program_authority.pubkey,
            program_token_account: program_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            mint: mint.pubkey,
            associated_token_token_program: associated_token_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
