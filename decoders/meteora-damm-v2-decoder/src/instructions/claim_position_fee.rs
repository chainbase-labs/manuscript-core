use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb4269a118521a2d3")]
pub struct ClaimPositionFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimPositionFeeInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimPositionFee {
    type ArrangedAccounts = ClaimPositionFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_authority, pool, position, token_a_account, token_b_account, token_a_vault, token_b_vault, token_a_mint, token_b_mint, position_nft_account, owner, token_a_program, token_b_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimPositionFeeInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            token_a_account: token_a_account.pubkey,
            token_b_account: token_b_account.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            owner: owner.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
