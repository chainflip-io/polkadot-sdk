
//! Autogenerated weights for `cumulus_pallet_parachain_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot-parachain
// benchmark
// pallet
// --chain
// bridge-hub-rococo-dev
// --pallet
// cumulus_pallet_parachain_system
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cumulus_pallet_parachain_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_parachain_system::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainSystem LastDmqMqcHead (r:1 w:1)
	/// Proof Skipped: ParachainSystem LastDmqMqcHead (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem ReservedDmpWeightOverride (r:1 w:0)
	/// Proof Skipped: ParachainSystem ReservedDmpWeightOverride (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: ParachainSystem ProcessedDownwardMessages (r:0 w:1)
	/// Proof Skipped: ParachainSystem ProcessedDownwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MessageQueue Pages (r:0 w:16)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn enqueue_inbound_downward_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12`
		//  Estimated: `8013`
		// Minimum execution time: 1_645_000 picoseconds.
		Weight::from_parts(1_717_000, 0)
			.saturating_add(Weight::from_parts(0, 8013))
			// Standard Error: 12_258
			.saturating_add(Weight::from_parts(24_890_934, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
