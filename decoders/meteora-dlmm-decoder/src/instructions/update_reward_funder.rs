use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd31c3020d7a02317")]
pub struct UpdateRewardFunder {
    pub reward_index: u64,
    pub new_funder: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardFunderInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardFunder {
    type ArrangedAccounts = UpdateRewardFunderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateRewardFunderInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
