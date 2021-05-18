use sp_runtime::{
	DispatchResult,
};
use orml_traits::NFT;

/// Abstraction over the GamePower wallet pallet.
#[allow(clippy::upper_case_acronyms)]
pub trait Wallet<AccountId>: NFT<AccountId> {

	/// Burn the given token ID from the account
	fn burn(token: (Self::ClassId, Self::TokenId)) -> DispatchResult;

	/// Transfer the given token ID from one account to another.
	fn send(from: &AccountId, to: &AccountId, token: (Self::ClassId, Self::TokenId)) -> DispatchResult;
}