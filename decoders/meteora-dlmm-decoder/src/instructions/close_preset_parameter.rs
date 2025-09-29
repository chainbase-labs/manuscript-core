use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04949164861ab53d")]
pub struct ClosePresetParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePresetParameterInstructionAccounts {
    pub preset_parameter: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePresetParameter {
    type ArrangedAccounts = ClosePresetParameterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [preset_parameter, admin, rent_receiver, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClosePresetParameterInstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
