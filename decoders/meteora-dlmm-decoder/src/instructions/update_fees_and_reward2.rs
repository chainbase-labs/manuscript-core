use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x208eb89a6741b858")]
pub struct UpdateFeesAndReward2 {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeesAndReward2InstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeesAndReward2 {
    type ArrangedAccounts = UpdateFeesAndReward2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeesAndReward2InstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            owner: owner.pubkey,
        })
    }
}
