use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e3b98d346b72ed0")]
pub struct SetPairStatusPermissionless {
    pub status: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusPermissionlessInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPairStatusPermissionless {
    type ArrangedAccounts = SetPairStatusPermissionlessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, creator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPairStatusPermissionlessInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            creator: creator.pubkey,
        })
    }
}
