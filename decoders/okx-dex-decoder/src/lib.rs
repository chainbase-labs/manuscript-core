use solana_pubkey::Pubkey;
pub struct OkxDexDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma");
