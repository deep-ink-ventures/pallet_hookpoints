
//! Autogenerated weights for pallet_hookpoints
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-13, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `chp`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/genesis-dao-solochain
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_hookpoints
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/hookpoints/src/weights.rs
// --template
// ./benchmarking/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_hookpoints.
pub trait WeightInfo {
	fn register_global_callback() -> Weight;
	fn register_specific_callback() -> Weight;
}

/// Weights for pallet_hookpoints using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Hookpoints::GlobalCallbacks` (r:0 w:1)
	/// Proof: `Hookpoints::GlobalCallbacks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_global_callback() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_155_000 picoseconds.
		Weight::from_parts(7_489_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Hookpoints::SpecificCallbacks` (r:0 w:1)
	/// Proof: `Hookpoints::SpecificCallbacks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_specific_callback() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_675_000 picoseconds.
		Weight::from_parts(8_073_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Hookpoints::GlobalCallbacks` (r:0 w:1)
	/// Proof: `Hookpoints::GlobalCallbacks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_global_callback() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_155_000 picoseconds.
		Weight::from_parts(7_489_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Hookpoints::SpecificCallbacks` (r:0 w:1)
	/// Proof: `Hookpoints::SpecificCallbacks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_specific_callback() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_675_000 picoseconds.
		Weight::from_parts(8_073_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
