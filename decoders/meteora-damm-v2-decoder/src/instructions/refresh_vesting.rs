use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x095ed80e74ccf700")]
pub struct RefreshVesting {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RefreshVestingInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshVesting {
    type ArrangedAccounts = RefreshVestingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, position, position_nft_account, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshVestingInstructionAccounts {
            pool: pool.pubkey,
            position: position.pubkey,
            position_nft_account: position_nft_account.pubkey,
            owner: owner.pubkey,
        })
    }
}
