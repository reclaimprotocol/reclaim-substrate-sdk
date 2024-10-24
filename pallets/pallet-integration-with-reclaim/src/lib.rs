#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case, unused_imports)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use pallet_reclaim::{traits::ReclaimVerifier, Proof};

#[frame_support::pallet]
pub mod pallet {

	use crate::weights::WeightInfo;

	use super::*;
	use frame_support::pallet_prelude::{DispatchResult, StorageMap, *};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	
	/// Configuration trait for the pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ReclaimVerifier: ReclaimVerifier<Proof>;
		type WeightInfo: WeightInfo;
	}

	/// Storage map to track verified accounts
	#[pallet::storage]
	#[pallet::getter(fn account_verified)]
	pub(super) type AccountVerified<T: Config> =
		StorageMap<_, Identity, T::AccountId, bool, OptionQuery>;

	/// Events emitted by the pallet
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emits when a user is verified.
		UserVerified { account_id: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Verifies a user based on the provided proof
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::verify_user())]
		pub fn verify_user(origin: OriginFor<T>, proof: Proof) -> DispatchResult {
			let who = ensure_signed(origin)?;
			T::ReclaimVerifier::verify_proof(&proof)?;
			<AccountVerified<T>>::set(&who, Some(true));
			Self::deposit_event(Event::UserVerified { account_id: who });
			Ok(())
		}
	}
}
