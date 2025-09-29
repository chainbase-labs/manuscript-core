use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x331396fc699d305b")]
pub struct CreateClaimProtocolFeeOperator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateClaimProtocolFeeOperatorInstructionAccounts {
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateClaimProtocolFeeOperator {
    type ArrangedAccounts = CreateClaimProtocolFeeOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [claim_fee_operator, operator, admin, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(CreateClaimProtocolFeeOperatorInstructionAccounts {
            claim_fee_operator: claim_fee_operator.pubkey,
            operator: operator.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
