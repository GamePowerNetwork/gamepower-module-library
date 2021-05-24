#![cfg(test)]

use super::*;

use crate as gamepower_wallet;
use balances;
use frame_support::{
	parameter_types,
	traits::{Filter, InstanceFilter},
};
use frame_system as system;
use gamepower_primitives::{WalletAssetData, WalletClassData};
use sp_core::{H256};
use sp_runtime::ModuleId;
use sp_runtime::{
  testing::Header,
  traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
      Block = Block,
      NodeBlock = Block,
      UncheckedExtrinsic = UncheckedExtrinsic,
    {
      System: frame_system::{Module, Call, Config, Storage, Event<T>},
      Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
      OrmlNFT: orml_nft::{Module ,Storage},
      GamePowerWallet: gamepower_wallet::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
  pub const BlockHashCount: u64 = 250;
}

/*
impl_outer_origin! {
  pub enum Origin for Test {}
}
*/

pub type AccountId = u64;

impl system::Config for Test {
  type BaseCallFilter = ();
  type BlockWeights = ();
  type BlockLength = ();
  type DbWeight = ();
  type Origin = Origin;
  type Call = Call;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type AccountData = balances::AccountData<u64>;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = Event;
  type BlockHashCount = BlockHashCount;
  type Version = ();
  type PalletInfo = PalletInfo;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type SystemWeightInfo = ();
  type SS58Prefix = ();
}

parameter_types! {
  pub const ExistentialDeposit: u64 = 500;
  pub const MaxLocks: u32 = 50;
}
impl balances::Config for Test {
  type MaxLocks = ();
  type Balance = u64;
  type Event = Event;
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = ();
}

parameter_types! {
  	pub AllowTransfer: bool = true;
	pub AllowBurn: bool = true;
	pub AllowEscrow: bool = true;
	pub AllowClaim: bool = true;
	pub const WalletModuleId: ModuleId = ModuleId(*b"gpwallet");
}

impl gamepower_wallet::Config for Test {
  	type Event = Event;
	type Transfer = GamePowerWallet;
	type Burn = GamePowerWallet;
	type Claim = GamePowerWallet;
	type AllowTransfer = AllowTransfer;
	type AllowBurn = AllowBurn;
	type AllowEscrow = AllowEscrow;
	type AllowClaim = AllowClaim;
	type Currency = Balances;
	type ModuleId = WalletModuleId;
}


impl orml_nft::Config for Test {
	type ClassId = u32;
	type TokenId = u64;
	type ClassData = ();
	type TokenData = ();
}

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CLASS_ID: <Test as orml_nft::Config>::ClassId = 0;
pub const CLASS_ID_NOT_EXIST: <Test as orml_nft::Config>::ClassId = 1;
pub const TOKEN_ID: <Test as orml_nft::Config>::TokenId = 0;
pub const TOKEN_ID_NOT_EXIST: <Test as orml_nft::Config>::TokenId = 1;
pub const LISTING_ID: u64 = 0;
pub const LISTING_ID_NOT_EXIST: u64 = 1;

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    balances::GenesisConfig::<Test> {
        // Provide some initial balances
        balances: vec![
            (1, 1000000),
            (2, 1000000),
            (3, 1000000),
            (4, 1000000),
            (5, 1000000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    let mut ext: sp_io::TestExternalities = t.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
