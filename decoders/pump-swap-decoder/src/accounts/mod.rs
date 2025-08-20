use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpSwapDecoder;
pub mod bonding_curve;
pub mod global_config;
pub mod global_volume_accumulator;
pub mod pool;
pub mod user_volume_accumulator;

pub enum PumpSwapAccount {
    BondingCurve(bonding_curve::BondingCurve),
    GlobalConfig(global_config::GlobalConfig),
    GlobalVolumeAccumulator(global_volume_accumulator::GlobalVolumeAccumulator),
    Pool(pool::Pool),
    UserVolumeAccumulator(user_volume_accumulator::UserVolumeAccumulator),
}

impl AccountDecoder<'_> for PumpSwapDecoder {
    type AccountType = PumpSwapAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::BondingCurve(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_config::GlobalConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::GlobalConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_volume_accumulator::GlobalVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::GlobalVolumeAccumulator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_volume_accumulator::UserVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::UserVolumeAccumulator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
