use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::MeteoraDlmmDecoder;
pub mod bin_array;
pub mod bin_array_bitmap_extension;
pub mod claim_fee_operator;
pub mod lb_pair;
pub mod oracle;
pub mod position;
pub mod position_v2;
pub mod preset_parameter;
pub mod preset_parameter2;
pub mod token_badge;

#[allow(clippy::large_enum_variant)]
pub enum MeteoraDlmmAccount {
    BinArrayBitmapExtension(bin_array_bitmap_extension::BinArrayBitmapExtension),
    BinArray(bin_array::BinArray),
    ClaimFeeOperator(claim_fee_operator::ClaimFeeOperator),
    LbPair(lb_pair::LbPair),
    Oracle(oracle::Oracle),
    Position(position::Position),
    PositionV2(position_v2::PositionV2),
    PresetParameter2(preset_parameter2::PresetParameter2),
    PresetParameter(preset_parameter::PresetParameter),
    TokenBadge(token_badge::TokenBadge),
}

impl AccountDecoder<'_> for MeteoraDlmmDecoder {
    type AccountType = MeteoraDlmmAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bin_array_bitmap_extension::BinArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArrayBitmapExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = bin_array::BinArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            claim_fee_operator::ClaimFeeOperator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::ClaimFeeOperator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lb_pair::LbPair::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::LbPair(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = oracle::Oracle::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Oracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position_v2::PositionV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PositionV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            preset_parameter2::PresetParameter2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PresetParameter2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            preset_parameter::PresetParameter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PresetParameter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use crate::types::{
        FeeInfo, ProtocolFee, RewardInfo, StaticParameters, UserRewardInfo, VariableParameters,
    };
    use solana_pubkey::pubkey;

    use super::{
        lb_pair::LbPair, oracle::Oracle, position::Position, preset_parameter::PresetParameter, *,
    };

    #[test]
    fn test_decode_lb_pair_account() {
        // Arrange
        let expected_account = LbPair {
            activation_point: 0,
            activation_type: 0,
            active_id: -1054,
            base_factor_seed: [36, 244],
            base_key: pubkey!("VamLzWq6NAZMK8Vw2tFEfr4bkkeeaS7yDDf2C5sNqLQ"),
            bin_array_bitmap: [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                3377699720527872,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            bin_step: 80,
            bin_step_seed: [80, 0],
            bump_seed: [255],
            creator: pubkey!("5oWhudgFVd1mEeKTzduHreYWSmYgfsMmBFLxqkD5AeJT"),
            last_updated_at: 0,
            oracle: pubkey!("YJ2QX2V48nZiB4kH2XtRZnLNKkA7v7N1sXnWfQ8n9Jq"),
            creator_pool_on_off_control: 0,
            token_mint_x_program_flag: 0,
            token_mint_y_program_flag: 0,
            padding1: [0; 32],
            padding2: [0; 32],
            padding3: [0; 8],
            padding4: 0,
            pair_type: 3,
            parameters: StaticParameters {
                base_factor: 62500,
                filter_period: 300,
                decay_period: 1200,
                reduction_factor: 5000,
                variable_fee_control: 7500,
                max_volatility_accumulator: 150000,
                min_bin_id: -5458,
                max_bin_id: 5458,
                protocol_share: 500,
                base_fee_power_factor: 0,
                padding: [0; 5],
            },
            pre_activation_duration: 0,
            pre_activation_swap_address: pubkey!("11111111111111111111111111111111"),
            protocol_fee: ProtocolFee {
                amount_x: 317260998460,
                amount_y: 240423958,
            },
            require_base_factor_seed: 0,
            reserve_x: pubkey!("7SWjpLPaLKxGogCY321XRFbiSiAkXSKh8q8qioGdu8N8"),
            reserve_y: pubkey!("iiWfKZygJii3g8RrKyWVQJdc6CciK2xGpuD8aCbgqbS"),
            reserved: [0; 22],
            reward_infos: [
                RewardInfo {
                    mint: pubkey!("11111111111111111111111111111111"),
                    vault: pubkey!("11111111111111111111111111111111"),
                    funder: pubkey!("11111111111111111111111111111111"),
                    reward_duration: 0,
                    reward_duration_end: 0,
                    reward_rate: 0,
                    last_update_time: 0,
                    cumulative_seconds_with_empty_liquidity_reward: 0,
                },
                RewardInfo {
                    mint: pubkey!("11111111111111111111111111111111"),
                    vault: pubkey!("11111111111111111111111111111111"),
                    funder: pubkey!("11111111111111111111111111111111"),
                    reward_duration: 0,
                    reward_duration_end: 0,
                    reward_rate: 0,
                    last_update_time: 0,
                    cumulative_seconds_with_empty_liquidity_reward: 0,
                },
            ],
            status: 0,
            token_x_mint: pubkey!("FzytSxQv6nBgH3dByVJtgMVg6rANTZaZPzU6dcRbpump"),
            token_y_mint: pubkey!("So11111111111111111111111111111111111111112"),
            v_parameters: VariableParameters {
                volatility_accumulator: 150000,
                volatility_reference: 75000,
                index_reference: -886,
                padding: [0; 4],
                last_update_timestamp: 1748616125,
                padding1: [0; 8],
            },
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/lb_pair_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MeteoraDlmmAccount::LbPair(account) => {
                assert_eq!(expected_account.activation_point, account.activation_point);
                assert_eq!(expected_account.activation_type, account.activation_type);
                assert_eq!(expected_account.active_id, account.active_id);
                assert_eq!(expected_account.base_factor_seed, account.base_factor_seed);
                assert_eq!(expected_account.base_key, account.base_key);
                assert_eq!(expected_account.bin_array_bitmap, account.bin_array_bitmap);
                assert_eq!(expected_account.bin_step, account.bin_step);
                assert_eq!(expected_account.bin_step_seed, account.bin_step_seed);
                assert_eq!(expected_account.bump_seed, account.bump_seed);
                assert_eq!(expected_account.creator, account.creator);
                assert_eq!(expected_account.last_updated_at, account.last_updated_at);
                assert_eq!(expected_account.oracle, account.oracle);
                assert_eq!(expected_account.padding1, account.padding1);
                assert_eq!(expected_account.padding2, account.padding2);
                assert_eq!(expected_account.padding3, account.padding3);
                assert_eq!(expected_account.padding4, account.padding4);
                assert_eq!(expected_account.pair_type, account.pair_type);
                assert_eq!(expected_account.parameters, account.parameters);
                assert_eq!(
                    expected_account.pre_activation_duration,
                    account.pre_activation_duration
                );
                assert_eq!(
                    expected_account.pre_activation_swap_address,
                    account.pre_activation_swap_address
                );
                assert_eq!(expected_account.protocol_fee, account.protocol_fee);
                assert_eq!(
                    expected_account.require_base_factor_seed,
                    account.require_base_factor_seed
                );
                assert_eq!(expected_account.reserve_x, account.reserve_x);
                assert_eq!(expected_account.reserve_y, account.reserve_y);
                assert_eq!(expected_account.reserved, account.reserved);
                assert_eq!(expected_account.reward_infos, account.reward_infos);
                assert_eq!(expected_account.status, account.status);
                assert_eq!(expected_account.token_x_mint, account.token_x_mint);
                assert_eq!(expected_account.token_y_mint, account.token_y_mint);
                assert_eq!(expected_account.v_parameters, account.v_parameters);
            }
            _ => panic!("Expected LbPairAccount"),
        }
    }

    #[test]
    fn test_decode_position_account() {
        // Arrange
        let expected_account = Position {
            fee_infos: [
                FeeInfo {
                    fee_x_per_token_complete: 3237430216995946070,
                    fee_y_per_token_complete: 338315940628455803,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3344215977641041254,
                    fee_y_per_token_complete: 359095789650660862,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3449067785093342886,
                    fee_y_per_token_complete: 390207297842947808,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3649746854123535238,
                    fee_y_per_token_complete: 372813125206544406,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4061348525785686840,
                    fee_y_per_token_complete: 423600912387165973,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4174051943599568166,
                    fee_y_per_token_complete: 445721745593672613,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4253164394862537286,
                    fee_y_per_token_complete: 424774068691991118,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3631511938755515833,
                    fee_y_per_token_complete: 382122079192575165,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3470715508927983598,
                    fee_y_per_token_complete: 385675387657977693,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3987418285622101436,
                    fee_y_per_token_complete: 439206402054294020,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4676575511895198856,
                    fee_y_per_token_complete: 471890585439944091,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4868613589236393543,
                    fee_y_per_token_complete: 478838451906984553,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4056474453903998277,
                    fee_y_per_token_complete: 396353952659702067,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2900985452688169261,
                    fee_y_per_token_complete: 257431649512112207,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1954244734204519486,
                    fee_y_per_token_complete: 184546370495890298,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1710709942673573716,
                    fee_y_per_token_complete: 179297770399655848,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1698160436942218995,
                    fee_y_per_token_complete: 194980253157133459,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2607164261053237624,
                    fee_y_per_token_complete: 270448688290498108,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2342060806885896269,
                    fee_y_per_token_complete: 238003435002880878,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3019341186719815397,
                    fee_y_per_token_complete: 308538775065394983,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4607511958966065930,
                    fee_y_per_token_complete: 433966778859712268,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 6451080832649175722,
                    fee_y_per_token_complete: 606167420423557883,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 8687875173560330499,
                    fee_y_per_token_complete: 800007388453413869,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 7305177228619618660,
                    fee_y_per_token_complete: 700150980617911254,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 6410004177076413178,
                    fee_y_per_token_complete: 646241116445589198,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 6363717907631000413,
                    fee_y_per_token_complete: 636540478379733740,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 6250195746351436029,
                    fee_y_per_token_complete: 631070172930907384,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 5630500732261984124,
                    fee_y_per_token_complete: 563347055381846655,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 5349544728334565168,
                    fee_y_per_token_complete: 525210519965865253,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3764329285427732080,
                    fee_y_per_token_complete: 365695527636916700,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3995512331991557943,
                    fee_y_per_token_complete: 395813535562255275,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4451677614351678142,
                    fee_y_per_token_complete: 437197265552201064,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3880912260623670734,
                    fee_y_per_token_complete: 395497833478777476,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4201995051360829556,
                    fee_y_per_token_complete: 415446678516441506,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 4486779568212043087,
                    fee_y_per_token_complete: 416500269206756618,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3217846640755188322,
                    fee_y_per_token_complete: 298767432884691310,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3573275137441754082,
                    fee_y_per_token_complete: 341667262397036939,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3755338225242181548,
                    fee_y_per_token_complete: 355262626467208041,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3461655249653659231,
                    fee_y_per_token_complete: 328177955060004600,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 3269651569214089689,
                    fee_y_per_token_complete: 297283342114947194,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2715514794685244397,
                    fee_y_per_token_complete: 245294905066967057,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2367625033796641650,
                    fee_y_per_token_complete: 213902351053043453,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2634250947370164479,
                    fee_y_per_token_complete: 246196177474824205,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 2519318248962674164,
                    fee_y_per_token_complete: 221440051508924725,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1680660981993680015,
                    fee_y_per_token_complete: 145548371838832985,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1664920973918393194,
                    fee_y_per_token_complete: 147292010640495879,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1613226648653358450,
                    fee_y_per_token_complete: 145461501603737762,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1034860904526350205,
                    fee_y_per_token_complete: 97394981100307429,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1376107217537806549,
                    fee_y_per_token_complete: 127145580115014617,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1613748999277948507,
                    fee_y_per_token_complete: 146204383800444496,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 1249589971801912607,
                    fee_y_per_token_complete: 111412892536818670,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 797464684034192397,
                    fee_y_per_token_complete: 72348586515025862,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 656612686779265758,
                    fee_y_per_token_complete: 62704467030982166,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 575803780381410494,
                    fee_y_per_token_complete: 58731876639458945,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 553186693487790121,
                    fee_y_per_token_complete: 56364657218830879,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 202336058579925293,
                    fee_y_per_token_complete: 19214628418178675,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 184939686121133222,
                    fee_y_per_token_complete: 17576654238193037,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 175946761721174878,
                    fee_y_per_token_complete: 16735344704666033,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 198991684334454249,
                    fee_y_per_token_complete: 18942430431927217,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 578631091549282,
                    fee_y_per_token_complete: 55128072976260,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
                FeeInfo {
                    fee_x_per_token_complete: 0,
                    fee_y_per_token_complete: 0,
                    fee_x_pending: 0,
                    fee_y_pending: 0,
                },
            ],
            last_updated_at: 1703204155,
            lb_pair: pubkey!("FoSDw2L5DmTuQTFe55gWPDXf88euaxAEKFre74CnvQbX"),
            liquidity_shares: [
                134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835,
                134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835, 134835,
                134835, 134835, 134835, 134835, 134835, 134835, 134823, 134835, 134835, 134835,
                134835, 134835, 134835, 149725, 127182, 127321, 127460, 127600, 127600, 127739,
                127878, 128017, 128017, 128156, 128295, 128434, 128435, 128574, 128713, 128852,
                128852, 128991, 129130, 129130, 129269, 129409, 129548, 129548, 129687, 129826,
                129965, 129965, 130104, 130243, 130383, 130383, 130522, 130661, 130743, 0,
            ],
            lower_bin_id: -2999,
            owner: pubkey!("2W7yBhBJDScbun4JbocAcRp5sx8SShy8uaHFDHfBeks6"),
            reserved: [0; 160],
            reward_infos: [UserRewardInfo {
                reward_per_token_completes: [0; 2],
                reward_pendings: [0; 2],
            }; 70],
            total_claimed_fee_x_amount: 0,
            total_claimed_fee_y_amount: 0,
            total_claimed_rewards: [0, 0],
            upper_bin_id: -2931,
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/position_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MeteoraDlmmAccount::Position(account) => {
                assert_eq!(expected_account.fee_infos, account.fee_infos);
                assert_eq!(expected_account.last_updated_at, account.last_updated_at);
                assert_eq!(expected_account.lb_pair, account.lb_pair);
                assert_eq!(expected_account.liquidity_shares, account.liquidity_shares);
                assert_eq!(expected_account.lower_bin_id, account.lower_bin_id);
                assert_eq!(expected_account.owner, account.owner);
                assert_eq!(expected_account.reserved, account.reserved);
                assert_eq!(expected_account.reward_infos, account.reward_infos);
                assert_eq!(
                    expected_account.total_claimed_fee_x_amount,
                    account.total_claimed_fee_x_amount
                );
                assert_eq!(
                    expected_account.total_claimed_fee_y_amount,
                    account.total_claimed_fee_y_amount
                );
                assert_eq!(
                    expected_account.total_claimed_rewards,
                    account.total_claimed_rewards
                );
                assert_eq!(expected_account.upper_bin_id, account.upper_bin_id);
            }
            _ => panic!("Expected PositionAccount"),
        }
    }

    #[test]
    fn test_decode_preset_parameter_account() {
        // Arrange
        let expected_account = PresetParameter {
            base_factor: 10000,
            bin_step: 30,
            decay_period: 600,
            filter_period: 30,
            max_bin_id: 14577,
            max_volatility_accumulator: 350000,
            min_bin_id: -14577,
            protocol_share: 500,
            reduction_factor: 5000,
            variable_fee_control: 15000,
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/preset_parameter_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MeteoraDlmmAccount::PresetParameter(account) => {
                assert_eq!(expected_account.base_factor, account.base_factor);
                assert_eq!(expected_account.bin_step, account.bin_step);
                assert_eq!(expected_account.decay_period, account.decay_period);
                assert_eq!(expected_account.filter_period, account.filter_period);
                assert_eq!(expected_account.max_bin_id, account.max_bin_id);
                assert_eq!(
                    expected_account.max_volatility_accumulator,
                    account.max_volatility_accumulator
                );
                assert_eq!(expected_account.min_bin_id, account.min_bin_id);
                assert_eq!(expected_account.protocol_share, account.protocol_share);
                assert_eq!(expected_account.reduction_factor, account.reduction_factor);
                assert_eq!(
                    expected_account.variable_fee_control,
                    account.variable_fee_control
                );
            }
            _ => panic!("Expected PresetParameterAccount"),
        }
    }

    #[test]
    fn test_decode_oracle_account() {
        // Arrange
        let expected_account = Oracle {
            active_size: 1,
            idx: 0,
            length: 100,
        };

        // Act
        let decoder = MeteoraDlmmDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/oracle_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MeteoraDlmmAccount::Oracle(account) => {
                assert_eq!(expected_account.active_size, account.active_size);
                assert_eq!(expected_account.idx, account.idx);
                assert_eq!(expected_account.length, account.length);
            }
            _ => panic!("Expected OracleAccount"),
        }
    }
}
