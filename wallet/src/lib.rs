// This file is part of GamePower Network.

// Copyright (C) 2021 GamePower Network.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_support::{
  decl_module, decl_storage, decl_error, decl_event, ensure,
  traits::{Currency, Get, ReservableCurrency},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
  DispatchResult, ModuleId, RuntimeDebug,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;
use orml_nft::Pallet as NftModule;
use gamepower_traits::*;
use gamepower_primitives::{ Balance };

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Listing<ClassIdOf, TokenIdOf, AccountId> {
    pub asset: (ClassIdOf, TokenIdOf),
    pub seller: AccountId,
    pub price: Balance,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Order<AccountId, ListingOf> {
    pub listing: ListingOf,
    pub buyer: AccountId,
}

/// The module configuration trait.
pub trait Config: system::Config + orml_nft::Config {
  type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
  /// Wallet Transfer Handler
  type Transfer: OnTransferHandler<Self::AccountId, Self::ClassId, Self::TokenId>;
  /// Wallet Burn Handler
  type Burn: OnBurnHandler<Self::AccountId, Self::ClassId, Self::TokenId>;
  /// Allow assets to be transferred through the wallet
  type AllowTransfer: Get<bool>;
  /// Allow assets to be burned from the wallet
  type AllowBurn: Get<bool>;
  /// Allow assets to be listed on the market
  type AllowMarketListing: Get<bool>;
  /// Allow asset claiming
  type AllowClaim: Get<bool>;
  /// Currency type for reserve/unreserve balance
  type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
  /// Wallet Module Id
  type ModuleId: Get<ModuleId>;
}

pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
pub type ListingOf<T> = Listing<ClassIdOf<T>, TokenIdOf<T>, <T as frame_system::Config>::AccountId>;
pub type OrderOf<T> = Order<<T as frame_system::Config>::AccountId, ListingOf<T>>;

decl_storage! {
  trait Store for Module<T: Config> as GamePowerWallet {

      pub ListingByOwner get(fn get_listing_by_owner): map hasher(blake2_128_concat) T::AccountId => Listing<ClassIdOf<T>, TokenIdOf<T>, T::AccountId>;
      pub OrderByOwner get(fn get_order_by_owner): map hasher(blake2_128_concat) T::AccountId => OrderOf<T>;
      pub AllListings get(fn all_listings_count): u64;
      pub AllOrders get(fn all_orders_count): u64;
      pub NextListingId get(fn next_listing_id): u64;
      pub NextOrderId get(fn next_order_id): u64;
  }
}

decl_event!(
  pub enum Event<T>
  where
      <T as frame_system::Config>::AccountId,
      ClassId = ClassIdOf<T>,
      TokenId = TokenIdOf<T>,
  {
      /// Asset successfully transferred through the wallet [from, to, classId, tokenId]
      WalletAssetTransferred(AccountId, AccountId, ClassId, TokenId),
      /// Asset successfully burned through the wallet [owner, classId, tokenId]
      WalletAssetBurned(AccountId, ClassId, TokenId),
  }
);

decl_error! {
  pub enum Error for Module<T: Config> {
      /// Assets cannot be tranferred
      TransfersNotAllowed,
      /// Assets cannot be burned
      BurningNotAllowed,
      /// Assets cannot be listed on the market
      MarketListingNotAllowed,
      /// Assets cannot be claimed
      ClaimingNotAllowed,
  }
}


decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
      type Error = Error<T>;

      fn deposit_event() = default;

      const AllowTransfer: bool = T::AllowTransfer::get();
      const AllowBurn: bool = T::AllowBurn::get();
      const AllowMarketListing: bool = T::AllowMarketListing::get();
      const AllowClaim: bool = T::AllowClaim::get();

      #[weight = 10_000]
      pub fn transfer(origin, asset:(ClassIdOf<T>, TokenIdOf<T>), to: T::AccountId) -> DispatchResult{

        let sender = ensure_signed(origin)?;

        ensure!(T::AllowTransfer::get(), Error::<T>::TransfersNotAllowed);

        T::Transfer::transfer(&sender, &to, asset)?;

        Self::deposit_event(RawEvent::WalletAssetTransferred(sender, to, asset.0, asset.1));

        Ok(().into())
      }

      #[weight = 10_000]
      pub fn burn(origin, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

        let sender = ensure_signed(origin)?;

        ensure!(T::AllowBurn::get(), Error::<T>::BurningNotAllowed);

        T::Burn::burn(&sender, asset)?;

        Self::deposit_event(RawEvent::WalletAssetBurned(sender, asset.0, asset.1));

        Ok(().into())
      }

      #[weight = 10_000]
      pub fn list(origin, asset:(ClassIdOf<T>, TokenIdOf<T>), price: Balance) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn buy(origin, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn emote(origin, asset:(ClassIdOf<T>, TokenIdOf<T>), emote: Vec<u8>) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn claim(origin, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

    }
}

// Implement OnTransferHandler
impl<T: Config> OnTransferHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
  fn transfer(from: &T::AccountId, to: &T::AccountId, asset: (T::ClassId, T::TokenId)) -> DispatchResult {
    NftModule::<T>::transfer(&from, &to, asset)?;
    Ok(())
  }
}

// Implement OnBurnHandler
impl<T: Config> OnBurnHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
  fn burn(owner: &T::AccountId, asset: (T::ClassId, T::TokenId)) -> DispatchResult {
    NftModule::<T>::burn(&owner, asset)?;
    Ok(())
  }
}