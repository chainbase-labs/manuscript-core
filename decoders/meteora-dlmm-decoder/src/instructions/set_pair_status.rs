use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x43f8e7899a95d9ae")]
pub struct SetPairStatus {
    pub status: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPairStatus {
    type ArrangedAccounts = SetPairStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPairStatusInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
        })
    }
}
