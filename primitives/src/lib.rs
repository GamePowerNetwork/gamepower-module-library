#![cfg_attr(not(feature = "std"), no_std)]
pub use wallet::*;

pub mod wallet;

/// An index to a block.
pub type BlockNumber = u64;
/// Balance of an account.
pub type Balance = u128;