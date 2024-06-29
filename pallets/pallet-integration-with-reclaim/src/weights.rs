#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::traits::Get;
use core::marker::PhantomData;
use frame_support::weights::Weight;

pub trait WeightInfo {
	fn verify_user() -> Weight;
}

/// Weight functions for `pallet_reclaim`.
pub struct SubstrateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeightInfo<T> {
	/// Storage: `Reclaim::PReclaimConfig` (r:1 w:0)
	/// Proof: `Reclaim::PReclaimConfig` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Reclaim::Epochs` (r:1 w:0)
	/// Proof: `Reclaim::Epochs` (`max_values`: None, `max_size`: Some(5266), added: 7741, mode: `MaxEncodedLen`)
	/// Storage: `IntegrationWithReclaim::AccountVerified` (r:0 w:1)
	/// Proof: `IntegrationWithReclaim::AccountVerified` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn verify_user() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `8731`
		// Minimum execution time: 488_000_000 picoseconds.
		Weight::from_parts(496_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8731))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}


impl WeightInfo for () {
	fn verify_user() -> Weight {
		Weight::from_parts(9_000_000, 0)
	}

}
