use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x235613b94ed44bd3")]
pub struct InitializeBinArray {
    pub index: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinArrayInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeBinArray {
    type ArrangedAccounts = InitializeBinArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array, funder, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeBinArrayInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array: bin_array.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
