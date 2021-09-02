use sp_runtime::DispatchResult;

/// An asset transfer handler
pub trait OnTransferHandler<AccountId, ClassId, TokenId> {
    /// Transfer the given token ID from one account to another.
    fn transfer(from: &AccountId, to: &AccountId, asset: (ClassId, TokenId)) -> DispatchResult;
}

/// An asset burn handler
pub trait OnBurnHandler<AccountId, ClassId, TokenId> {
    /// burn the given asset.
    fn burn(owner: &AccountId, asset: (ClassId, TokenId)) -> DispatchResult;
}

/// An asset claim handler
pub trait OnClaimHandler<AccountId, ClassId, TokenId> {
    /// claim the given asset
    fn claim(owner: &AccountId, asset: (ClassId, TokenId)) -> DispatchResult;
}
