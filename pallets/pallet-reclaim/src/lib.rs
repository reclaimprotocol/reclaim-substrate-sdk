#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case, unused_imports)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::ConstU32,
	sp_runtime::{
		traits::{IdentifyAccount, Verify},
		BoundedVec, SaturatedConversion,
	},
};
pub use pallet::*;
use pallet_timestamp::{self as timestamp};
use scale_info::prelude::{fmt::Debug, format, string::String, vec, vec::Vec};
pub use weights::WeightInfo;

use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

use crate::identity_digest::Identity256;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub mod traits;

mod identity_digest;

use traits::ReclaimVerifier;

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
	pub current_epoch: u64,
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
	pub address: [u8; 20],
	pub host: [u8; 32],
}

impl Witness {
	pub fn get_addresses(witness: Vec<Witness>) -> Vec<String> {
		let mut vec_addresses = vec![];
		for wit in witness {
			vec_addresses.push(hex::encode(wit.address));
		}
		vec_addresses
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
	pub id: u64,
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
	pub fn hash(&self) -> String {
		let mut hasher = Keccak256::new();
		let hash_str = format!("{}\n{}\n{}", &self.provider, &self.parameters, &self.context);
		hasher.update(&hash_str);

		let hash = hasher.finalize().to_vec();
		append_0x(hex::encode(hash).as_str())
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, scale_info::TypeInfo, Debug)]
pub struct CompleteClaimData {
	pub identifier: String,
	pub owner: String,
	pub epoch: u64,
	pub timestampS: u64,
}

impl CompleteClaimData {
	pub fn serialise(&self) -> String {
		format!("{}\n{}\n{}\n{}", &self.identifier, &self.owner, &self.timestampS, &self.epoch)
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, scale_info::TypeInfo, Debug)]
pub struct SignedClaim {
	pub claim: CompleteClaimData,
	pub signatures: Vec<String>,
}

impl SignedClaim {
	pub fn recover_signers_of_signed_claim(self) -> Vec<Vec<u8>> {
		// use crate::claims::identity_digest::Identity256;
		use digest::Update;
		// Create empty array
		let mut expected = vec![];
		// Hash the signature
		let serialised_claim = self.claim.serialise();

		let bm = keccak256_eth(serialised_claim.as_str());
		let message_hash = bm.to_vec();

		// For each signature in the claim
		for complete_signature in self.signatures {
			// complete_signature.remove(0);
			// complete_signature.remove(0);
			let rec_param = complete_signature
				.get((complete_signature.len() as usize - 2)..(complete_signature.len() as usize))
				.unwrap();
			let mut mut_sig_str = complete_signature.clone();
			mut_sig_str.pop();
			mut_sig_str.pop();

			let rec_dec = hex::decode(rec_param).unwrap();
			let rec_norm = rec_dec.first().unwrap() - 27;
			let r_s = hex::decode(mut_sig_str).unwrap();

			let id = match rec_norm {
				0 => RecoveryId::new(false, false),
				1 => RecoveryId::new(true, false),
				2_u8..=u8::MAX => todo!(),
			};

			let signature = Signature::from_bytes(r_s.as_slice().into()).unwrap();
			let message_digest = Identity256::new().chain(&message_hash);

			// Recover the public key
			let verkey = VerifyingKey::recover_from_digest(message_digest, &signature, id).unwrap();
			let key: Vec<u8> = verkey.to_encoded_point(false).as_bytes().into();
			let hasher = Keccak256::new_with_prefix(&key[1..]);

			let hash = hasher.finalize().to_vec();

			let address_bytes = hash.get(12..).unwrap();
			let public_key = &hex::encode(address_bytes);
			let dec_public_key = hex::decode(public_key).unwrap();
			expected.push(dec_public_key);
		}
		expected
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, scale_info::TypeInfo, Debug)]
pub struct Proof {
	pub claimInfo: ClaimInfo,
	pub signedClaim: SignedClaim,
}

pub fn fetch_witness_for_claim(epoch: Epoch, identifier: String, timestamp: u64) -> Vec<Witness> {
	let mut selected_witness = vec![];

	// Create a hash from identifier+epoch+minimum+timestamp
	let hash_str = format!(
		"{}\n{}\n{}\n{}",
		hex::encode(identifier),
		epoch.minimum_witness_for_claim_creation,
		timestamp,
		epoch.id
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

pub fn append_0x(content: &str) -> String {
	let mut initializer = String::from("0x");
	initializer.push_str(content);
	initializer
}

pub fn keccak256_eth(message: &str) -> Vec<u8> {
	let message: &[u8] = message.as_ref();

	let mut eth_message = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
	eth_message.extend_from_slice(message);
	let mut hasher = Keccak256::new();
	hasher.update(&eth_message);

	let hash = hasher.finalize().to_vec();
	hash
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
	pub(super) type Epochs<T: Config> = StorageMap<_, Blake2_128Concat, u64, Epoch, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ContractInitialized { owner: T::AccountId },
		ProofVerified { epoch_id: u64 },
		EpochAdded { epoch_id: u64 },
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
			let reclaim_config = ReclaimConfig { owner: who.clone(), current_epoch: 0_u64 };
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
			let current_epoch = <Epochs<T>>::get(epoch_count);

			let proof = Proof { claimInfo: claim_info, signedClaim: signed_claim };
			<Self as ReclaimVerifier<Proof, Epoch>>::verify_proof(&proof, &current_epoch)?;
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
			let new_epoch_id = config.current_epoch + 1_u64;
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

impl<T> ReclaimVerifier<Proof, Epoch> for Pallet<T>
where
	T: Config,
{
	fn verify_proof(proof: &Proof, current_epoch: &Epoch) -> DispatchResult {
		let signed_claim = proof.signedClaim.clone();
		let hashed = proof.claimInfo.hash();

		ensure!(signed_claim.claim.identifier == hashed, Error::<T>::HashMismatch);
		let expected_witness = fetch_witness_for_claim(
			current_epoch.clone(),
			signed_claim.claim.identifier.clone(),
			signed_claim.claim.timestampS,
		);

		let expected_witness_addresses = Witness::get_addresses(expected_witness);

		let signed_witness = signed_claim.recover_signers_of_signed_claim();
		ensure!(
			expected_witness_addresses.len() == signed_witness.len(),
			Error::<T>::LengthMismatch
		);

		for signed in signed_witness {
			ensure!(
				expected_witness_addresses.contains(&hex::encode(signed)),
				Error::<T>::SignatureMismatch
			);
		}
		Ok(())
	}
}
