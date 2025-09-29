use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9248aee028fd54ae")]
pub struct GoToABin {
    pub bin_id: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct GoToABinInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub from_bin_array: solana_pubkey::Pubkey,
    pub to_bin_array: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GoToABin {
    type ArrangedAccounts = GoToABinInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, from_bin_array, to_bin_array, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GoToABinInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            from_bin_array: from_bin_array.pubkey,
            to_bin_array: to_bin_array.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
