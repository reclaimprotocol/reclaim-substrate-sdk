#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::ConstU32,
	sp_io::{crypto::secp256k1_ecdsa_recover_compressed, hashing::keccak_256},
	sp_runtime::{
		traits::{IdentifyAccount, Verify},
		BoundedVec, SaturatedConversion,
	},
};
pub use pallet::*;
use pallet_timestamp::{self as timestamp};
use scale_info::prelude::{
	fmt::Debug,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};
use sha2::{Digest, Sha256};
use sp_core::ecdsa;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Clone,
	PartialOrd,
	Ord,
	scale_info::TypeInfo,
	MaxEncodedLen,
	Debug,
)]
pub struct ReclaimConfig<AccountId> {
	pub owner: AccountId,
	pub current_epoch: u128,
}

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Clone,
	PartialOrd,
	Ord,
	scale_info::TypeInfo,
	MaxEncodedLen,
	Debug,
)]
pub struct Witness {
	pub address: ecdsa::Public,
	pub host: [u8; 32],
}

impl Witness {
	pub fn get_addresses(witness: Vec<Witness>) -> Vec<ecdsa::Public> {
		let mut vec_addresses = vec![];
		for wit in witness {
			vec_addresses.push(wit.address);
		}
		return vec_addresses
	}
}

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Clone,
	PartialOrd,
	Ord,
	scale_info::TypeInfo,
	MaxEncodedLen,
	Default,
	Debug,
)]
pub struct Epoch {
	pub id: u128,
	pub timestamp_start: u64,
	pub timestamp_end: u64,
	pub minimum_witness_for_claim_creation: u128,
	pub witness: BoundedVec<Witness, ConstU32<100>>,
}

fn generate_random_seed(bytes: Vec<u8>, offset: usize) -> u32 {
	let hash_slice = &bytes[offset..offset + 4];
	let mut seed = 0u32;
	for (i, &byte) in hash_slice.iter().enumerate() {
		seed |= u32::from(byte) << (i * 8);
	}

	seed
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, scale_info::TypeInfo, Debug)]
pub struct ClaimInfo {
	pub provider: String,
	pub parameters: String,
	pub context: String,
}

impl ClaimInfo {
	pub fn hash(&self) -> Vec<u8> {
		let mut hasher = Sha256::new();
		let hash_str = format!("{}\n{}\n{}", &self.provider, &self.parameters, &self.context);
		hasher.update(hash_str.as_bytes().to_vec());
		let result = hasher.finalize().to_vec();
		result
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, scale_info::TypeInfo, Debug)]
pub struct CompleteClaimData {
	pub identifier: Vec<u8>,
	pub owner: ecdsa::Public,
	pub epoch: u128,
	pub timestamp_s: u128,
}

impl CompleteClaimData {
	pub fn serialise(&self) -> Vec<u8> {
		let hash_str = format!(
			"{}\n{}\n{}\n{}",
			hex::encode(&self.identifier),
			hex::encode(&self.owner),
			&self.timestamp_s.to_string(),
			&self.epoch.to_string()
		);
		hash_str.as_bytes().to_vec()
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, scale_info::TypeInfo, Debug)]
pub struct SignedClaim {
	pub claim: CompleteClaimData,
	pub bytes: Vec<[u8; 65]>,
}

impl SignedClaim {
	pub fn recover_signers_of_signed_claim(self) -> Result<Vec<ecdsa::Public>, ()> {
		let mut expected = vec![];
		let mut hasher = Sha256::new();
		let serialised_claim = self.claim.serialise();
		hasher.update(serialised_claim);
		let result = hasher.finalize().to_vec();
		let hash = keccak_256(&result);
		for signature in self.bytes {
			if let Ok(recovered_raw) = secp256k1_ecdsa_recover_compressed(&signature, &hash) {
				let recovered = ecdsa::Public::from_raw(recovered_raw);
				expected.push(recovered.into_account());
			}
		}
		Ok(expected)
	}
}

pub fn fetch_witness_for_claim(
	epoch: Epoch,
	identifier: Vec<u8>,
	claim_timestamp: u128,
) -> Vec<Witness> {
	let mut selected_witness = vec![];
	let hash_str = format!(
		"{}\n{}\n{}\n{}",
		hex::encode(identifier),
		epoch.minimum_witness_for_claim_creation.to_string(),
		claim_timestamp.to_string(),
		epoch.id.to_string()
	);
	let result = hash_str.as_bytes().to_vec();
	let mut hasher = Sha256::new();
	hasher.update(result);
	let hash_result = hasher.finalize().to_vec();
	let witenesses_left_list = epoch.witness;
	let mut byte_offset = 0;
	let witness_left = witenesses_left_list.len();
	for _i in 0..epoch.minimum_witness_for_claim_creation.into() {
		let random_seed = generate_random_seed(hash_result.clone(), byte_offset) as usize;
		let witness_index = random_seed % witness_left;
		let witness = witenesses_left_list.get(witness_index);
		match witness {
			Some(data) => selected_witness.push(data.clone()),
			None => {},
		}
		byte_offset = (byte_offset + 4) % hash_result.len();
	}

	selected_witness
}

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;

	use super::*;
	use frame_support::pallet_prelude::{DispatchResult, StorageMap, *};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Signature: Verify<Signer = Self::PublicKey> + Encode + Decode + Parameter;
		type PublicKey: IdentifyAccount<AccountId = Self::PublicKey> + Encode + Decode + Parameter;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn reclaim_config)]
	pub type PReclaimConfig<T: Config> = StorageValue<_, ReclaimConfig<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn epochs)]
	pub(super) type Epochs<T: Config> = StorageMap<_, Blake2_128Concat, u128, Epoch, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ContractInitialized { owner: T::AccountId },
		ProofVerified { epoch_id: u128 },
		EpochAdded { epoch_id: u128 },
	}

	#[pallet::error]
	pub enum Error<T> {
		OnlyOwner,
		AlreadyInitialized,
		HashMismatch,
		LengthMismatch,
		SignatureMismatch,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::init())]
		pub fn init(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<PReclaimConfig<T>>::exists(), Error::<T>::AlreadyInitialized);
			let reclaim_config = ReclaimConfig { owner: who.clone(), current_epoch: 0_u128 };
			<PReclaimConfig<T>>::put(reclaim_config);
			Self::deposit_event(Event::ContractInitialized { owner: who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::verify_proof())]
		pub fn verify_proof(
			origin: OriginFor<T>,
			claim_info: ClaimInfo,
			signed_claim: SignedClaim,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let config = <PReclaimConfig<T>>::get().unwrap();
			let epoch_count = config.current_epoch;
			let current_epoch = <Epochs<T>>::get(&epoch_count);
			let hashed = claim_info.hash();

			ensure!(signed_claim.claim.identifier == hashed, Error::<T>::HashMismatch);
			let expected_witness = fetch_witness_for_claim(
				current_epoch.clone(),
				signed_claim.claim.identifier.clone(),
				signed_claim.claim.timestamp_s,
			);

			let expected_witness_addresses = Witness::get_addresses(expected_witness);

			let signed_witness = signed_claim.recover_signers_of_signed_claim().unwrap();
			ensure!(
				expected_witness_addresses.len() == signed_witness.len(),
				Error::<T>::LengthMismatch
			);

			for signed in signed_witness {
				ensure!(
					expected_witness_addresses.contains(&signed),
					Error::<T>::SignatureMismatch
				);
			}
			Self::deposit_event(Event::ProofVerified { epoch_id: current_epoch.id });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_epoch())]
		pub fn add_epoch(
			origin: OriginFor<T>,
			witness: BoundedVec<Witness, ConstU32<100>>,
			minimum_witness: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let config = <PReclaimConfig<T>>::get().unwrap();
			let owner = config.owner;
			ensure!(who == owner, Error::<T>::OnlyOwner);
			let new_epoch_id = config.current_epoch + 1_u128;
			let now = timestamp::Pallet::<T>::get().saturated_into::<u64>();
			let epoch = Epoch {
				id: new_epoch_id,
				witness,
				timestamp_start: now,
				timestamp_end: now + 10000_u64,
				minimum_witness_for_claim_creation: minimum_witness,
			};

			<Epochs<T>>::insert(new_epoch_id, epoch);
			<PReclaimConfig<T>>::set(Some(ReclaimConfig { owner, current_epoch: new_epoch_id }));
			Self::deposit_event(Event::EpochAdded { epoch_id: new_epoch_id });

			Ok(())
		}
	}
}
