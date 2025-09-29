use solana_pubkey::Pubkey;

pub struct MeteoraDlmmDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
