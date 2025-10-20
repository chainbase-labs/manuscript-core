use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5b07d06e7abbad5")]
pub struct PermanentLockPosition {
    pub permanent_lock_liquidity: u128,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermanentLockPositionInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermanentLockPosition {
    type ArrangedAccounts = PermanentLockPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, position, position_nft_account, owner, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PermanentLockPositionInstructionAccounts {
            pool: pool.pubkey,
            position: position.pubkey,
            position_nft_account: position_nft_account.pubkey,
            owner: owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
