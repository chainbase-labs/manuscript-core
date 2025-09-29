use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9204f8988e84689")]
pub struct ClaimFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimFeeInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFee {
    type ArrangedAccounts = ClaimFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, position, bin_array_lower, bin_array_upper, sender, reserve_x, reserve_y, user_token_x, user_token_y, token_x_mint, token_y_mint, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimFeeInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            position: position.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            user_token_x: user_token_x.pubkey,
            user_token_y: user_token_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
