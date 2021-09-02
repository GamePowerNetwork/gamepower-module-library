# Wallet Integration Example

## Getting Started

---

This pallet provides an example of how to integrate a pallet with the GamePower Wallet Pallet.

## Basic Configuration

---

`runtime/src/lib.rs`

```
impl gamepower_wallet_integration::Config for Runtime {
	type Event = Event;
}



construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        -- SNIP --

        // Add this code
        GamePowerMarketIntegration: gamepower_wallet_integration::{Module, Call, Storage, Event<T>},
	}
);
```
