// This pallet use The Open Runtime Module Library (ORML) which is a community maintained collection of Substrate runtime modules.
// Thanks to all contributors of orml.
// https://github.com/open-web3-stack/open-runtime-module-library

//! ### Module Functions
//!
//! - `create_class` - Create asset class
//! - `mint` - Mint asset


#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_error, decl_event
    dispatch::{DispatchResultWithPostInfo},
    ensure,
};

use frame_system::{self as system, ensure_signed};
use orml_nft::Pallet as AssetModule;
use gamepower_wallet::Module as WalletModule;
use gamepower_primitives::{WalletClassData, WalletAssetData};
use gamepower_traits::{
	OnTransferHandler, OnBurnHandler, OnClaimHandler,
};
use sp_std::vec::Vec;

pub trait Config:
system::Config +
orml_nft::Config<
    TokenData=WalletAssetData,
    ClassData=WalletClassData,
>{
	type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;
}

pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;

decl_event!(
	pub enum Event<T>
	where
	  <T as frame_system::Config>::AccountId,
	  ClassId = ClassIdOf<T>,
	  TokenId = TokenIdOf<T>,
	{
	  /// Asset successfully transferred [from, to, classId, tokenId]
	  AssetTransferred(AccountId, AccountId, ClassId, TokenId),
	  /// Asset successfully burned [owner, classId, tokenId]
	  AssetBurned(AccountId, ClassId, TokenId),
	  /// Claiming assset [owner]
	  AssetBeingClaimed(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// A generic error
		NoPermission,
	}
  }

decl_module! {
  pub struct Module<T: Config> for enum Call where origin: T::Origin {
    type Error = Error<T>;

	/// Class creation
	/// The wallet needs a way to creat classes which are buckets that hold minted assets.
	///
	/// - `metadata`: data for our class. usually an IPFS hash
	/// - `properties`: properties for our class. This uses WalletClassData which you can replace with any type of data
    #[weight = 10_000]
    pub fn create_class(origin, metadata: Vec<u8>, properties: Vec<u8>) -> DispatchResultWithPostInfo{

        let sender = ensure_signed(origin)?;

        let class_data = WalletClassData
        {
            properties,
        };

        AssetModule::<T>::create_class(&sender, metadata, class_data)?;

        Ok(().into())
    }

	/// Class creation
	/// The wallet needs a way to creat classes which are buckets that hold minted assets.
	///
	/// - `class_id`: the class we would like to place this asset in
	/// - `metadata`: data for our class. usually an IPFS hash
	/// - `properties`: properties for our class. This uses WalletClassData which you can replace with any type of data
	/// - `quantity`: instructs the pallet on how many tokens to mint
    #[weight = 10_000]
    pub fn mint(origin, class_id: ClassIdOf<T>, metadata: Vec<u8>, properties: Vec<u8>, quantity: u32) -> DispatchResultWithPostInfo {

        let sender = ensure_signed(origin)?;

        ensure!(quantity >= 1, Error::<T>::NoPermission);
        let class_info = AssetModule::<T>::classes(class_id).ok_or(Error::<T>::NoPermission)?;
        ensure!(sender == class_info.owner, Error::<T>::NoPermission);

        let new_asset_data = WalletAssetData {
            properties: properties.clone(),
        };

        let mut new_asset_ids: Vec<u64> = Vec::new();

        for _ in 0..quantity{
          AssetModule::<T>::mint(&sender, class_id, metadata.clone(), new_asset_data.clone())?;
        }

        Ok(().into())
    }

  }
}

// Implement OnTransferHandler
impl<T: Config> OnTransferHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
	fn transfer(from: &T::AccountId, to: &T::AccountId, asset: (T::ClassId, T::TokenId)) -> DispatchResult {
		AssetModule::<T>::transfer(&from, &to, asset)?;
		Module::<T>::deposit_event(RawEvent::AssetTransferred(from.clone(), to.clone(), asset.0, asset.1));
		Ok(())
	}
}

// Implement OnBurnHandler
impl<T: Config> OnBurnHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
	fn burn(owner: &T::AccountId, asset: (T::ClassId, T::TokenId)) -> DispatchResult {
		AssetModule::<T>::burn(&owner, asset)?;
		Module::<T>::deposit_event(RawEvent::AssetBurned(owner.clone(), asset.0, asset.1));
		Ok(())
	}
}

// Implement OnBurnHandler
impl<T: Config> OnClaimHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
	fn claim(owner: &T::AccountId, _asset: (T::ClassId, T::TokenId)) -> DispatchResult {

		// Here is an example of causing a handler to fail. Preventing the wallet from completing the transaction.
		let allowDispatch = false;
		ensure!(allowDispatch, Error::<T>::NoPermission);

		Module::<T>::deposit_event(RawEvent::AssetBeingClaimed(owner.clone()));
		Ok(())
	}
}
