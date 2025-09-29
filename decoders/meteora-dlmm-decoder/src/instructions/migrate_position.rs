use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f843b32c706fb2e")]
pub struct MigratePosition {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigratePositionInstructionAccounts {
    pub position_v2: solana_pubkey::Pubkey,
    pub position_v1: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigratePosition {
    type ArrangedAccounts = MigratePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_v2, position_v1, lb_pair, bin_array_lower, bin_array_upper, owner, system_program, rent_receiver, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigratePositionInstructionAccounts {
            position_v2: position_v2.pubkey,
            position_v1: position_v1.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            rent_receiver: rent_receiver.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
