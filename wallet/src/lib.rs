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
  decl_error, decl_event, decl_module, decl_storage, ensure, Parameter,
  traits::{Get},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
    traits::{AtLeast32Bit, One, StaticLookup, Zero},
    DispatchError, DispatchResult,
};
use sp_std::vec::Vec;
use gamepower_traits::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The module configuration trait.
pub trait Config: system::Config {
    type TransferPermission: Get<bool>;

    type BurnPermission: Get<bool>;

    type MarketPermission: Get<bool>;
}


decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

      const TransferPermission: bool = T::TransferPermission::get();
      const BurnPermission: bool = T::BurnPermission::get();
      const MarketPermission: bool = T:MarketPermission::get();

      #[weight = 10_000]
        pub fn transfer(origin, name: Vec<u8>, properties: Vec<u8>) -> DispatchResult{

            let sender = ensure_signed(origin)?;

            Ok(())
        }

    }
}


impl<T: Config> Module<T> {
    
}