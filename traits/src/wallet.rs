use sp_runtime::{
	DispatchResult,
};

/// A transfer handler
pub trait OnTransferHandler<AccountId, ClassId, TokenId>  {
	/// Transfer the given token ID from one account to another.
	fn transfer(from: &AccountId, to: &AccountId, asset: (ClassId, TokenId)) -> DispatchResult;
}

/// A asset burn handler
pub trait OnBurnHandler<AccountId, ClassId, TokenId> {
	/// Transfer the given token ID from one account to another.
	fn burn(from: &AccountId, asset: (ClassId, TokenId)) -> DispatchResult;
}