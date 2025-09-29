use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x082957235030791a")]
pub struct CloseClaimProtocolFeeOperator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseClaimProtocolFeeOperatorInstructionAccounts {
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseClaimProtocolFeeOperator {
    type ArrangedAccounts = CloseClaimProtocolFeeOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [claim_fee_operator, rent_receiver, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseClaimProtocolFeeOperatorInstructionAccounts {
            claim_fee_operator: claim_fee_operator.pubkey,
            rent_receiver: rent_receiver.pubkey,
            admin: admin.pubkey,
        })
    }
}
