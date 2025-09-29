use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x42bc47d3626d0eba")]
pub struct InitializePresetParameter {
    pub ix: InitPresetParametersIx,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePresetParameterInstructionAccounts {
    pub preset_parameter: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePresetParameter {
    type ArrangedAccounts = InitializePresetParameterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [preset_parameter, admin, system_program, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializePresetParameterInstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
