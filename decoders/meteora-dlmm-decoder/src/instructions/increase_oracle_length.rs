use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe3d7d57674f9ead")]
pub struct IncreaseOracleLength {
    pub length_to_add: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IncreaseOracleLengthInstructionAccounts {
    pub oracle: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseOracleLength {
    type ArrangedAccounts = IncreaseOracleLengthInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [oracle, funder, system_program, event_authority, program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(IncreaseOracleLengthInstructionAccounts {
            oracle: oracle.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
