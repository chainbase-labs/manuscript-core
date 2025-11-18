use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f")]
pub struct UpdateConfigAccount {
    pub param: u8,
    pub owner: solana_pubkey::Pubkey,
}

pub struct UpdateConfigAccountInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateConfigAccount {
    type ArrangedAccounts = UpdateConfigAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, amm_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateConfigAccountInstructionAccounts {
            admin: admin.pubkey,
            amm_config: amm_config.pubkey,
        })
    }
}
