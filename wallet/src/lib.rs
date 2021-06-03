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
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
    traits::{AccountIdConversion, One},
    DispatchError, DispatchResult, ModuleId, RuntimeDebug,
};

use gamepower_primitives::{ClaimId, ListingId};
use gamepower_traits::*;
use orml_nft::Pallet as AssetModule;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::str;
use sp_std::vec;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Listing data
pub struct Listing<ClassIdOf, TokenIdOf, AccountId, Balance> {
    /// Listing Id
    pub id: ListingId,
    /// Seller of the listing
    pub seller: AccountId,
    /// Asset - (class_id, token_id)
    pub asset: (ClassIdOf, TokenIdOf),
    /// Price of the asset listed
    pub price: Balance,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Claim data
pub struct Claim<ClassIdOf, TokenIdOf, AccountId> {
    /// account this claim is meant for
    pub receiver: AccountId,
    /// Asset - (class_id, token_id)
    pub asset: (ClassIdOf, TokenIdOf),
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Order data
pub struct Order<ListingOf, AccountId, BlockNumber> {
    /// order listing
    pub listing: ListingOf,
    /// order buyer
    pub buyer: AccountId,
    /// genesis block
    pub block: BlockNumber,
}

/// The module configuration trait.
pub trait Config: system::Config + orml_nft::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// Wallet Transfer Handler
    type Transfer: OnTransferHandler<Self::AccountId, Self::ClassId, Self::TokenId>;
    /// Wallet Burn Handler
    type Burn: OnBurnHandler<Self::AccountId, Self::ClassId, Self::TokenId>;
    /// Wallet Claim Handler
    type Claim: OnClaimHandler<Self::AccountId, Self::ClassId, Self::TokenId>;
    /// Allow assets to be transferred through the wallet
    type AllowTransfer: Get<bool>;
    /// Allow assets to be burned from the wallet
    type AllowBurn: Get<bool>;
    /// Allow assets to be listed on the market
    type AllowEscrow: Get<bool>;
    /// Allow asset claiming
    type AllowClaim: Get<bool>;
    /// Currency type for reserve/unreserve balance
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// Wallet Module Id
    type ModuleId: Get<ModuleId>;
}

/// Class Id
pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
/// Token Id
pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
/// Listing Data
pub type ListingOf<T> =
    Listing<ClassIdOf<T>, TokenIdOf<T>, <T as system::Config>::AccountId, BalanceOf<T>>;
/// Claim Data
pub type ClaimOf<T> = Claim<ClassIdOf<T>, TokenIdOf<T>, <T as system::Config>::AccountId>;
/// Order Data
pub type OrderOf<T> =
    Order<ListingOf<T>, <T as system::Config>::AccountId, <T as system::Config>::BlockNumber>;
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

decl_storage! {
  trait Store for Module<T: Config> as GamePowerWallet {

    /// Get a listing by the listing_id
    pub Listings get(fn listings):
        map hasher(twox_64_concat) ListingId => Option<ListingOf<T>>;
    /// Get all listings ids by an account
    pub ListingsByOwner get(fn listings_by_owner):
        map hasher(blake2_128_concat) T::AccountId => Option<Vec<ListingId>>;
    /// Get a vector of all listings. Used as a quick lookup.
    pub AllListings get(fn all_listings): Vec<(ClassIdOf<T>, TokenIdOf<T>)>;
    /// Get the next listing id
    pub NextListingId get(fn next_listing_id): ListingId;
    /// A fast and simple count of all current listings
    pub ListingCount get(fn listing_count): u64;
    /// A count of all orders made through the wallet
    pub OrderCount: u64;
    /// A history of orders for an asset
    pub OrderHistory get(fn order_history):
        map hasher(twox_64_concat) (ClassIdOf<T>, TokenIdOf<T>) => Option<OrderOf<T>>;
    /// Get one or more claims by AccountId or a single claim including the claim_id
    pub OpenClaims get(fn open_claims):
        double_map hasher(blake2_128_concat) T::AccountId, hasher(twox_64_concat) ClaimId => Option<ClaimOf<T>>;
    /// Get a vector of all claims. Used as a quick lookup.
    pub AllClaims get(fn all_claims): Vec<(ClassIdOf<T>, TokenIdOf<T>)>;
    /// Get the next claim id
    pub NextClaimId get(fn next_claim_id): ClaimId;
    /// Emotes used by the wallet
    pub Emotes get(fn emotes):
        double_map hasher(twox_64_concat) (ClassIdOf<T>, TokenIdOf<T>), hasher(twox_64_concat) T::AccountId => Vec<Vec<u8>>;
  }
}

decl_event!(
  pub enum Event<T>
  where
    <T as frame_system::Config>::AccountId,
    ClassId = ClassIdOf<T>,
    TokenId = TokenIdOf<T>,
    Balance = BalanceOf<T>,
  {
    /// Asset successfully transferred through the wallet [from, to, classId, tokenId]
    WalletAssetTransferred(AccountId, AccountId, ClassId, TokenId),
    /// Asset successfully burned through the wallet [owner, classId, tokenId]
    WalletAssetBurned(AccountId, ClassId, TokenId),
    /// Asset successfully listed through the wallet [owner, price, listingId,, classId, tokenId]
    WalletAssetListed(AccountId, Balance, ListingId, ClassId, TokenId),
    /// Asset successfully unlisted through the wallet [owner, listingId, classId, tokenId]
    WalletAssetUnlisted(AccountId, ListingId, ClassId, TokenId),
    /// Asset successfully purchased through the wallet [seller, buyer, classId, tokenId]
    WalletAssetPurchased(AccountId, AccountId, ClassId, TokenId),
    /// Asset successfully purchased through the wallet [receiver, classId, tokenId]
    WalletAssetClaimed(AccountId, ClassId, TokenId),
    /// Asset claim created [creator, receiver, classId, tokenId]
    WalletClaimCreated(AccountId, AccountId, ClassId, TokenId),
    /// Asset buy successful [seller, buyer, listingId, price]
    WalletAssetBuySuccess(AccountId, AccountId, ListingId, Balance),
    /// New Emote posted [poster, classId, tokenId, emote]
    WalletAssetEmotePosted(AccountId, ClassId, TokenId, Vec<u8>),
  }
);

decl_error! {
  pub enum Error for Module<T: Config> {
    /// Assets cannot be tranferred
    TransfersNotAllowed,
    /// An error occurred during transfer
    TransferCancelled,
    /// An error occurred during burn
    BurnCancelled,
    /// Assets cannot be burned
    BurningNotAllowed,
    /// Assets cannot be listed on the market
    EscrowNotAllowed,
    /// Asset locked in Escrow or Claims
    AssetLocked,
    /// Assets cannot be claimed
    ClaimingNotAllowed,
    /// An error occurred during claim
    ClaimCancelled,
    /// Asset not found
    AssetNotFound,
    /// Listing not found
    ListingNotFound,
    /// Listing not found
    UnlistingFailed,
    /// Claim not found
    ClaimNotFound,
    /// Claim creation failed
    ClaimCreateFailed,
    /// Maximum listings in Escrow
    NoAvailableListingId,
    /// Maximum claims made
    NoAvailableClaimId,
    /// Maximum orders in Escrow
    NoAvailableOrderId,
    /// Invalid Emote
    InvalidEmote,
    /// No Permission for this action
    NoPermission,
  }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        const AllowTransfer: bool = T::AllowTransfer::get();
        const AllowBurn: bool = T::AllowBurn::get();
        const AllowEscrow: bool = T::AllowEscrow::get();
        const AllowClaim: bool = T::AllowClaim::get();

          /// Transfer asset
        ///
        /// - `to`: the token recipient
        /// - `asset`: (class_id, token_id)
        #[weight = 10_000]
        pub fn transfer(origin, to: T::AccountId, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to transfer assets
            ensure!(T::AllowTransfer::get(), Error::<T>::TransfersNotAllowed);

            // Check that the sender owns this asset
            let check_ownership = Self::check_ownership(&sender, &asset)?;
            ensure!(check_ownership, Error::<T>::NoPermission);

            // Ensure that the asset is not locked in Escrow or Claims
            ensure!(!Self::is_locked(&asset), Error::<T>::AssetLocked);

            // Transfer the asset
            ensure!(T::Transfer::transfer(&sender, &to, asset).is_ok(), Error::<T>::TransferCancelled);

            Ok(())
        }

        /// Burn asset
        ///
        /// - `asset`: (class_id, token_id)
        #[weight = 10_000]
        pub fn burn(origin, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to burn assets
            ensure!(T::AllowBurn::get(), Error::<T>::BurningNotAllowed);

            // Check that the sender owns this asset
            let check_ownership = Self::check_ownership(&sender, &asset)?;
            ensure!(check_ownership, Error::<T>::NoPermission);

            // Ensure that the asset is not locked in Escrow or Claims
            ensure!(!Self::is_locked(&asset), Error::<T>::AssetLocked);

            // Burn the asset
            ensure!(T::Burn::burn(&sender, asset).is_ok(), Error::<T>::BurnCancelled);

            Ok(())
        }

        /// Send the asset to escrow to be listed on the market
        ///
        /// - `asset`: (class_id, token_id)
        /// - `price`: price to sell the asset on the market
        #[weight = 10_000]
        pub fn list(origin, asset:(ClassIdOf<T>, TokenIdOf<T>), price: BalanceOf<T>) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to list assets
            ensure!(T::AllowEscrow::get(), Error::<T>::EscrowNotAllowed);

            // Check that the sender owns this asset
            let check_ownership = Self::check_ownership(&sender, &asset)?;
            ensure!(check_ownership, Error::<T>::NoPermission);

            // Ensure this asset isn't already listed
            ensure!(!Self::is_locked(&asset), Error::<T>::AssetLocked);

            // Escrow Account
            let escrow_account: T::AccountId = Self::get_escrow_account();

            // Transfer into escrow
            Self::do_transfer(&sender, &escrow_account, asset).ok();

            // Add the new listing id to storage
            let listing_id = NextListingId::try_mutate(|id| -> Result<ListingId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(One::one()).ok_or(Error::<T>::NoAvailableListingId)?;

                Ok(current_id)
            })?;

            // Create listing data
            let listing = Listing {
                id: listing_id,
                seller: sender.clone(),
                asset,
                price,
            };

            // Increment Listing count
            ListingCount::mutate(|id| -> Result<u64, DispatchError> {
                let current_count = *id;
                *id = id.checked_add(One::one()).ok_or(Error::<T>::NoAvailableListingId)?;

                Ok(current_count)
            }).ok();

            // Add listing to storage
            Listings::<T>::insert(listing_id, listing);

            // Add listing to owner
            // Get owner listing data
            if ListingsByOwner::<T>::contains_key(&sender) {
                ListingsByOwner::<T>::try_mutate(&sender, |owner_data| -> DispatchResult {
                    let data = owner_data.as_mut().ok_or(Error::<T>::ListingNotFound)?;
                    // Append the new listing id
                    data.push(listing_id);

                    // Update owner listings
                    ListingsByOwner::<T>::insert(&sender, data);
                    Ok(())
                })?;
            } else {
                let listings = vec![listing_id];
                ListingsByOwner::<T>::insert(&sender, listings)
            }

            // Add asset to all listings
            AllListings::<T>::append(&asset);

            Self::deposit_event(RawEvent::WalletAssetListed(sender, price, listing_id, asset.0, asset.1));

            Ok(())
        }

        /// Remove the asset from escrow
        ///
        /// - `listing_id`: id of the Listing
        #[weight = 10_000]
        pub fn unlist(origin, listing_id: ListingId) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to list assets
            ensure!(T::AllowEscrow::get(), Error::<T>::EscrowNotAllowed);

            // Get listing data
            Listings::<T>::try_mutate_exists(listing_id, |listing_data| -> DispatchResult {
                let data = listing_data.as_mut().ok_or(Error::<T>::ListingNotFound)?;

                // Ensure the listing is in storage for this user
                ensure!(sender == data.seller, Error::<T>::NoPermission);

                // Ensure listing data was removed
                let is_unlisted = Self::do_unlist(&sender, data.clone(), false)?;
                ensure!(is_unlisted, Error::<T>::UnlistingFailed);

                Self::deposit_event(RawEvent::WalletAssetUnlisted(sender, listing_id, data.asset.0, data.asset.1));

                Ok(())
            })?;

            // Remove the actual listing from state
            Listings::<T>::remove(listing_id);

            Ok(())
        }

        /// Buy the asset from the market
        ///
        /// - `listing_id`: id of the Listing
        #[weight = 10_000]
        pub fn buy(origin, listing_id: ListingId) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to list assets
            ensure!(T::AllowEscrow::get(), Error::<T>::EscrowNotAllowed);

            // Ensure the listing is in storage
            ensure!(Listings::<T>::contains_key(listing_id), Error::<T>::ListingNotFound);

            // Get listing data
            Listings::<T>::try_mutate(listing_id, |listing_data| -> DispatchResult {
                let data = listing_data.as_mut().ok_or(Error::<T>::ListingNotFound)?;

                // Now that the order has been placed, let's remove the listing
                // Ensure listing data was removed
                let is_unlisted = Self::do_unlist(&data.seller, data.clone(), true)?;
                ensure!(is_unlisted, Error::<T>::UnlistingFailed);

                // Transfer funds to seller
                <T as Config>::Currency::transfer(&sender, &data.seller, data.price, ExistenceRequirement::KeepAlive)?;

                // Transfer the asset out of escrow to the buyer
                //Escrow Account
                let escrow_account: T::AccountId = Self::get_escrow_account();
                Self::do_transfer(&escrow_account, &sender, data.asset).ok();

                // Increment Order count
                OrderCount::mutate(|id| -> Result<u64, DispatchError> {
                    let current_count = *id;
                    *id = id.checked_add(One::one()).ok_or(Error::<T>::NoAvailableOrderId)?;

                    Ok(current_count)
                }).ok();

                // Get the current block for this order
                let block_number = <system::Module<T>>::block_number();

                // Create order data
                let order = Order {
                    listing: data.clone(),
                    buyer: sender.clone(),
                    block: block_number,
                };

                // Save order history
                OrderHistory::<T>::insert(order.listing.asset, order);

                Self::deposit_event(
                    RawEvent::WalletAssetBuySuccess(
                        data.seller.clone(),
                        sender,
                        data.id,
                        data.price
                    )
                );

                Ok(())
            })?;

            // Remove the actual listing from state
            Listings::<T>::remove(listing_id);

            Ok(())
        }

        /// Post an emote for the asset
        ///
        /// - `asset`: (class_id, token_id)
        /// - `emote`: name of the emote to use
        #[weight = 10_000]
        pub fn emote(origin, asset:(ClassIdOf<T>, TokenIdOf<T>), emote: Vec<u8>) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Ensure this token exists
            ensure!(!AssetModule::<T>::tokens(asset.0, asset.1).is_none(), Error::<T>::AssetNotFound);

            // Convert the emote to a string
            let str_emote = str::from_utf8(&emote).unwrap();

            // Ensure this is a valid emote
            ensure!(!emojis::lookup(str_emote).is_none(), Error::<T>::InvalidEmote);

            // Get emoji
            let emoji = emojis::lookup(str_emote).unwrap().as_str().as_bytes().to_vec();

            // Get emotes data
            let mut emotes_data = Emotes::<T>::get(asset, &sender);

            // Append the new emoji
            emotes_data.push(emoji.clone());

            // Add emote to storage
            Emotes::<T>::insert(asset, &sender, emotes_data);

            Self::deposit_event(RawEvent::WalletAssetEmotePosted(sender, asset.0, asset.1, emoji));

            Ok(())
        }

        /// Claim an asset
        ///
        /// - `claim_id`: id of the claim
        #[weight = 10_000]
        pub fn claim(origin, claim_id: ClaimId) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to claim assets
            ensure!(T::AllowClaim::get(), Error::<T>::ClaimingNotAllowed);

            // Ensure the claim is for this sender
            ensure!(OpenClaims::<T>::contains_key(&sender, claim_id), Error::<T>::ClaimNotFound);

            // Get claim data
            OpenClaims::<T>::try_mutate(sender.clone(), claim_id, |claim_data| -> DispatchResult {
                let data = claim_data.as_mut().ok_or(Error::<T>::ClaimNotFound)?;

                // Perform any domain related tasks to claiming
                ensure!(T::Claim::claim(&sender, data.asset).is_ok(), Error::<T>::ClaimCancelled);

                // Claim Account
                let claim_account: T::AccountId = Self::get_claim_account();

                // Transfer asset into the reciever's account
                Self::do_transfer(&claim_account, &sender, data.asset).ok();

                AllClaims::<T>::try_mutate(|asset_ids| -> DispatchResult {
                    let asset_index = asset_ids.iter().position(|x| *x == data.asset).unwrap();
                    asset_ids.remove(asset_index);

                    Ok(())
                })?;

                // Remove the open claim
                OpenClaims::<T>::remove(&sender, claim_id);

                Self::deposit_event(RawEvent::WalletAssetClaimed(sender, data.asset.0, data.asset.1));

                Ok(())
            })?;

            Ok(())
        }

        /// Create an asset claim for this account
        ///
        /// - `receiver`: account to receive this asset
        /// - `asset`: (class_id, token_id)
        #[weight = 10_000]
        pub fn create_claim(origin, receiver: T::AccountId, asset:(ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            // Check that the wallet has permission to claim assets
            ensure!(T::AllowClaim::get(), Error::<T>::ClaimingNotAllowed);

            // Check that the sender owns this asset
            let check_ownership = Self::check_ownership(&sender, &asset)?;
            ensure!(check_ownership, Error::<T>::NoPermission);

            // Ensure that the sender is the owner of this class
            let class_info = AssetModule::<T>::classes(asset.0).ok_or(Error::<T>::AssetNotFound)?;
            ensure!(sender == class_info.owner, Error::<T>::NoPermission);

            // Ensure the claim is created
            let claim_created = Self::do_create_claim(&sender, &receiver, asset)?;
            ensure!(claim_created, Error::<T>::ClaimCreateFailed);

            Self::deposit_event(RawEvent::WalletClaimCreated(sender, receiver, asset.0, asset.1));

            Ok(())
        }

    }
}

// Module Implementation
impl<T: Config> Module<T> {
    fn check_ownership(
        owner: &T::AccountId,
        asset: &(ClassIdOf<T>, TokenIdOf<T>),
    ) -> Result<bool, DispatchError> {
        Ok(AssetModule::<T>::is_owner(&owner, *asset))
    }

    fn do_transfer(
        from: &T::AccountId,
        to: &T::AccountId,
        asset: (ClassIdOf<T>, TokenIdOf<T>),
    ) -> Result<bool, DispatchError> {
        AssetModule::<T>::transfer(&from, &to, asset).ok();
        Ok(true)
    }

    fn is_listed(asset: &(ClassIdOf<T>, TokenIdOf<T>)) -> bool {
        Self::all_listings().contains(asset)
    }

    fn is_claiming(asset: &(ClassIdOf<T>, TokenIdOf<T>)) -> bool {
        Self::all_claims().contains(asset)
    }

    fn get_claim_account() -> T::AccountId {
        T::ModuleId::get().into_sub_account(100u32)
    }

    fn get_escrow_account() -> T::AccountId {
        T::ModuleId::get().into_account()
    }

    pub fn is_locked(asset: &(ClassIdOf<T>, TokenIdOf<T>)) -> bool {
        Self::is_listed(&asset) || Self::is_claiming(&asset)
    }

    fn do_unlist(
        sender: &T::AccountId,
        listing_data: ListingOf<T>,
        is_buy: bool,
    ) -> Result<bool, DispatchError> {
        //Escrow Account
        let escrow_account: T::AccountId = Self::get_escrow_account();

        // Transfer out of escrow
        if !is_buy {
            Self::do_transfer(&escrow_account, &sender, listing_data.asset).ok();
        }

        // Decrease Listing count
        ListingCount::mutate(|id| -> Result<u64, DispatchError> {
            let current_count = *id;
            *id = id
                .checked_sub(One::one())
                .ok_or(Error::<T>::NoAvailableListingId)?;

            Ok(current_count)
        })
        .ok();

        // Remove the asset from all listings
        AllListings::<T>::try_mutate(|asset_ids| -> DispatchResult {
            let asset_index = asset_ids
                .iter()
                .position(|x| *x == listing_data.asset)
                .unwrap();
            asset_ids.remove(asset_index);

            Ok(())
        })?;

        // Remove listing from owner
        // Get owner listing data
        ListingsByOwner::<T>::try_mutate(
            listing_data.clone().seller,
            |owner_data| -> DispatchResult {
                let data = owner_data.as_mut().ok_or(Error::<T>::ListingNotFound)?;

                // Remove the old listing id
                data.retain(|&x| x != listing_data.id);

                // Update owner listings
                ListingsByOwner::<T>::insert(listing_data.seller, data);

                Ok(())
            },
        )?;

        Ok(true)
    }

    fn do_create_claim(
        owner: &T::AccountId,
        receiver: &T::AccountId,
        asset: (ClassIdOf<T>, TokenIdOf<T>),
    ) -> Result<bool, DispatchError> {
        // Get claim account
        let claim_account: T::AccountId = Self::get_claim_account();

        // Transfer asset into the claim account
        Self::do_transfer(&owner, &claim_account, asset).ok();

        // Create claim data
        let claim = Claim {
            receiver: receiver.clone(),
            asset,
        };

        // Add the new claim id to storage
        let claim_id = NextClaimId::try_mutate(|id| -> Result<ClaimId, DispatchError> {
            let current_id = *id;
            *id = id
                .checked_add(One::one())
                .ok_or(Error::<T>::NoAvailableClaimId)?;

            Ok(current_id)
        })?;

        // Add claim to storage
        OpenClaims::<T>::insert(receiver, claim_id, claim);
        AllClaims::<T>::append(&asset);

        Ok(true)
    }
}

// Implement OnTransferHandler
impl<T: Config> OnTransferHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
    fn transfer(
        from: &T::AccountId,
        to: &T::AccountId,
        asset: (T::ClassId, T::TokenId),
    ) -> DispatchResult {
        Self::do_transfer(&from, &to, asset)?;
        Module::<T>::deposit_event(RawEvent::WalletAssetTransferred(
            from.clone(),
            to.clone(),
            asset.0,
            asset.1,
        ));
        Ok(())
    }
}

// Implement OnBurnHandler
impl<T: Config> OnBurnHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
    fn burn(owner: &T::AccountId, asset: (T::ClassId, T::TokenId)) -> DispatchResult {
        AssetModule::<T>::burn(&owner, asset)?;
        Module::<T>::deposit_event(RawEvent::WalletAssetBurned(owner.clone(), asset.0, asset.1));
        Ok(())
    }
}

// Implement OnClaimHandler
impl<T: Config> OnClaimHandler<T::AccountId, T::ClassId, T::TokenId> for Module<T> {
    fn claim(_owner: &T::AccountId, _asset: (T::ClassId, T::TokenId)) -> DispatchResult {
        Ok(())
    }
}
