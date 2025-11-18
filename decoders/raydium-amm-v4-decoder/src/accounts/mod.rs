use {
    super::RaydiumAmmV4Decoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod amm_info;
pub mod fees;
pub mod target_orders;

#[allow(clippy::large_enum_variant)]
pub enum RaydiumAmmV4Account {
    TargetOrders(target_orders::TargetOrders),
    Fees(fees::Fees),
    AmmInfo(amm_info::AmmInfo),
}

impl AccountDecoder<'_> for RaydiumAmmV4Decoder {
    type AccountType = RaydiumAmmV4Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        let data_size = account.data.len();

        match data_size {
            amm_info::AMM_INFO_SIZE => {
                let decoded_account = amm_info::AmmInfo::deserialize(account.data.as_slice())?;
                Some(carbon_core::account::DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumAmmV4Account::AmmInfo(decoded_account),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                })
            }
            fees::FEES_SIZE => {
                let decoded_account = fees::Fees::deserialize(account.data.as_slice())?;
                Some(carbon_core::account::DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumAmmV4Account::Fees(decoded_account),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                })
            }
            target_orders::TARGET_ORDERS_SIZE => {
                let decoded_account =
                    target_orders::TargetOrders::deserialize(account.data.as_slice())?;
                Some(carbon_core::account::DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumAmmV4Account::TargetOrders(decoded_account),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                })
            }
            _ => None,
        }
    }
}
