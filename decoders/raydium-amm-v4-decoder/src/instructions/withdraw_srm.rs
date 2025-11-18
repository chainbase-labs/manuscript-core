use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct WithdrawSrm {
    pub amount: u64,
}

pub struct WithdrawSrmInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_owner_account: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub srm_token: solana_pubkey::Pubkey,
    pub dest_srm_token: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSrm {
    type ArrangedAccounts = WithdrawSrmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_owner_account, amm_authority, srm_token, dest_srm_token, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawSrmInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_owner_account: amm_owner_account.pubkey,
            amm_authority: amm_authority.pubkey,
            srm_token: srm_token.pubkey,
            dest_srm_token: dest_srm_token.pubkey,
        })
    }
}
