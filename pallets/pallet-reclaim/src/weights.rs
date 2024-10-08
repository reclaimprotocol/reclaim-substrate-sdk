//! Autogenerated weights for `pallet_reclaim`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// target/release/node-template
// benchmark
// pallet
// --pallet
// pallet-reclaim
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benches


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::traits::Get;
use core::marker::PhantomData;
use frame_support::weights::Weight;

pub trait WeightInfo {
	fn init() -> Weight;

	fn verify_proof() -> Weight;

	fn add_epoch() -> Weight;
}

/// Weight functions for `pallet_reclaim`.
pub struct SubstrateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeightInfo<T> {
	/// Storage: `Reclaim::PReclaimConfig` (r:1 w:1)
	/// Proof: `Reclaim::PReclaimConfig` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	fn init() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1525`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1525))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Reclaim::PReclaimConfig` (r:1 w:1)
	/// Proof: `Reclaim::PReclaimConfig` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Reclaim::Epochs` (r:0 w:1)
	/// Proof: `Reclaim::Epochs` (`max_values`: None, `max_size`: Some(5266), added: 7741, mode: `MaxEncodedLen`)
	fn add_epoch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `1525`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1525))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Reclaim::PReclaimConfig` (r:1 w:0)
	/// Proof: `Reclaim::PReclaimConfig` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Reclaim::Epochs` (r:1 w:0)
	/// Proof: `Reclaim::Epochs` (`max_values`: None, `max_size`: Some(5266), added: 7741, mode: `MaxEncodedLen`)
	fn verify_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `8731`
		// Minimum execution time: 482_000_000 picoseconds.
		Weight::from_parts(484_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8731))
			.saturating_add(T::DbWeight::get().reads(2))
	}
}


impl WeightInfo for () {
	fn init() -> Weight {
		Weight::from_parts(9_000_000, 0)
	}

	fn verify_proof() -> Weight {
		Weight::from_parts(9_000_000, 0)
	}

	fn add_epoch() -> Weight {
		Weight::from_parts(9_000_000, 0)
	}
}
