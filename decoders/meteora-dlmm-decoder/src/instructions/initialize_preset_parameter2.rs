use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb807f0ab672fb779")]
pub struct InitializePresetParameter2 {
    pub ix: InitPresetParameters2Ix,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePresetParameter2InstructionAccounts {
    pub preset_parameter: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePresetParameter2 {
    type ArrangedAccounts = InitializePresetParameter2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [preset_parameter, admin, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializePresetParameter2InstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
