use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe33e02fcf70aabb9")]
pub struct LockPosition {
    pub params: VestingParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LockPositionInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub vesting: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockPosition {
    type ArrangedAccounts = LockPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, position, vesting, position_nft_account, owner, payer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockPositionInstructionAccounts {
            pool: pool.pubkey,
            position: position.pubkey,
            vesting: vesting.pubkey,
            position_nft_account: position_nft_account.pubkey,
            owner: owner.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
