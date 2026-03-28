use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x90486b9423da1fbb")]
pub struct IsNonceUsed {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IsNonceUsedInstructionAccounts {
    pub used_nonce: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IsNonceUsed {
    type ArrangedAccounts = IsNonceUsedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let used_nonce = next_account(&mut iter)?;

        Some(IsNonceUsedInstructionAccounts { used_nonce })
    }
}
