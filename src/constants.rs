use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;

pub const NATIVE_MINT_STR: &'static str = "So11111111111111111111111111111111111111112";

pub const ADMIN_PUBKEY: &'static str = "66rpMu8a9ngQqsfE1a8ubkuW2bpEM9aLvCbxV4Qro12h";

pub const FEE_PER_DIV: u128 = 1000;

pub const TOTAL_SUPPLY: u64 = 1_000_000_000_000_000; // 1 billion
pub const VIRT_SOL_RESERVE: u64 = 11_000_000_000; // 11 SOL
pub const REAL_SOL_THRESHOLD: u64 = 37_000_000_000; //37 SOL
