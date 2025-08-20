use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x561fc057a3574fee")]
pub struct SyncUserVolumeAccumulator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SyncUserVolumeAccumulatorInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub global_volume_accumulator: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncUserVolumeAccumulator {
    type ArrangedAccounts = SyncUserVolumeAccumulatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SyncUserVolumeAccumulatorInstructionAccounts {
            user,
            global_volume_accumulator,
            user_volume_accumulator,
            event_authority,
            program,
        })
    }
}
