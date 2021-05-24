# GamePower Wallet Pallet 🕹

[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2FGamePowerNet)](https://twitter.com/GamePowerNet)
[![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/em75apGJZV)

## Getting Started

# 1. Install ORML
The GamePower Wallet pallet requires the ORML NFT crate to be installed and configured into your runtime.

https://github.com/open-web3-stack/open-runtime-module-library/tree/master/nft

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
# Orml pacakges
orml-nft = { default-features = false, version = '0.4.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
use gamepower_primitives::{WalletClassData, WalletAssetData};

// Add this code
impl orml_nft::Config for Runtime {
	type ClassId = u32;
	type TokenId = u64;
	type ClassData = WalletClassData;
	type TokenData = WalletAssetData;
}



construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        -- SNIP --

        // Add this code
        OrmlNFT: orml_nft::{Module ,Storage},
	}
);
```
# 2. Install GamePower Wallet Pallet
### Importing the GamePower Wallet pallet

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code

# GamePower packages
gamepower-wallet = { package = "gamepower-wallet", git = "https://github.com/GamePowerNetwork/gamepower-module-library", branch = "main", default-features = false }
gamepower-primitives = { package = "gamepower-primitives", git = "https://github.com/GamePowerNetwork/gamepower-module-library", branch = "main", default-features = false }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use gamepower_wallet;


// Add this code
parameter_types! {
	pub AllowTransfer: bool = true;
	pub AllowBurn: bool = true;
	pub AllowEscrow: bool = true;
	pub AllowClaim: bool = true;
	pub const WalletModuleId: ModuleId = ModuleId(*b"gpwallet");
}

impl gamepower_wallet::Config for Runtime {
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


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        -- SNIP --

        // Add this code
        GamePowerWallet: gamepower_wallet::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo +nightly test
```

## Documentation

```
cargo doc --open --package gamepower-wallet
```

## Build

```
cargo +nightly build --release
```
