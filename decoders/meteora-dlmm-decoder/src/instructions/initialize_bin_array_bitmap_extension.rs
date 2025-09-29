use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f9de2b40cf02147")]
pub struct InitializeBinArrayBitmapExtension {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinArrayBitmapExtensionInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeBinArrayBitmapExtension {
    type ArrangedAccounts = InitializeBinArrayBitmapExtensionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, funder, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeBinArrayBitmapExtensionInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
