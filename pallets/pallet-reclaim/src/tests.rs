use super::*;
use crate::mock::*;
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;
use sha2::{Digest, Sha256};
use sp_core::{ecdsa, keccak_256, Pair};
use sp_runtime::SaturatedConversion;

#[test]
fn init() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		System::assert_has_event(Event::ContractInitialized { owner: source_account_id }.into());
		assert_eq!(
			Reclaim::reclaim_config(),
			Some(ReclaimConfig { owner: source_account_id, current_epoch: 0_u128 })
		)
	})
}

#[test]
fn should_add_epochs() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		let pair = ecdsa::Pair::generate().0;
		let w1 = Witness { address: pair.public(), host: [1_u8; 32] };
		let mut witnesses_vec = Vec::<Witness>::new();
		witnesses_vec.push(w1);
		let witnesses: BoundedVec<Witness, ConstU32<100>> =
			BoundedVec::<Witness, ConstU32<100>>::try_from(witnesses_vec).unwrap();
		let minimum_witness = 1;
		assert_ok!(Reclaim::add_epoch(
			RawOrigin::Signed(source_account_id).into(),
			witnesses,
			minimum_witness
		));
		System::assert_has_event(Event::EpochAdded { epoch_id: 1 }.into());
	})
}

#[test]
fn should_approve_valid_proofs() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		let pair = ecdsa::Pair::generate().0;
		let w1 = Witness { address: pair.public(), host: [2_u8; 32] };
		let mut witnesses_vec = Vec::<Witness>::new();
		witnesses_vec.push(w1);
		let witnesses: BoundedVec<Witness, ConstU32<100>> =
			BoundedVec::<Witness, ConstU32<100>>::try_from(witnesses_vec).unwrap();
		let minimum_witness = 1;
		assert_ok!(Reclaim::add_epoch(
			RawOrigin::Signed(source_account_id).into(),
			witnesses,
			minimum_witness
		));
		let claim_info = ClaimInfo {
			provider: "provider".to_string(),
			parameters: "{}".to_string(),
			context: "context".to_string(),
		};
		let hashed = claim_info.hash();
		dbg!(&hex::encode(&hashed));

		let now = Timestamp::get().saturated_into::<u128>();
		let complete_claim_data = CompleteClaimData {
			identifier: hashed,
			owner: pair.public(),
			epoch: 1_u128,
			timestamp_s: now,
		};
		let mut hasher = Sha256::new();
		let serialised_claim = complete_claim_data.serialise();
		hasher.update(serialised_claim);
		let result = hasher.finalize().to_vec();
		let mut sigs = Vec::new();
		let hash = keccak_256(&result);
		let signature = pair.sign_prehashed(&hash);
		sigs.push(signature.0);
		dbg!(hex::encode(&sigs[0]));
		let signed_claim = SignedClaim { claim: complete_claim_data, bytes: sigs };

		assert_ok!(Reclaim::verify_proof(
			RawOrigin::Signed(source_account_id).into(),
			claim_info,
			signed_claim
		));
	})
}

#[test]
fn should_not_approve_invalid_proofs() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		let pair = ecdsa::Pair::generate().0;
		let faulty_pair = ecdsa::Pair::generate().0;
		let w1 = Witness { address: pair.public(), host: [2_u8; 32] };
		let mut witnesses_vec = Vec::<Witness>::new();
		witnesses_vec.push(w1);
		let witnesses: BoundedVec<Witness, ConstU32<100>> =
			BoundedVec::<Witness, ConstU32<100>>::try_from(witnesses_vec).unwrap();
		let minimum_witness = 1;
		assert_ok!(Reclaim::add_epoch(
			RawOrigin::Signed(source_account_id).into(),
			witnesses,
			minimum_witness
		));
		let claim_info = ClaimInfo {
			provider: "uid-dob".to_string(),
			parameters: "{\"dob\":\"0000-00-00\"}".to_string(),
			context: "some-application-specific-context".to_string(),
		};
		let hashed = claim_info.hash();
		let now = Timestamp::get().saturated_into::<u128>();
		let complete_claim_data = CompleteClaimData {
			identifier: hashed,
			owner: pair.public(),
			epoch: 1_u128,
			timestamp_s: now,
		};
		let mut hasher = Sha256::new();
		let serialised_claim = complete_claim_data.serialise();
		hasher.update(serialised_claim);
		let result = hasher.finalize().to_vec();
		let mut sigs = Vec::new();
		let hash = keccak_256(&result);
		let signature = faulty_pair.sign_prehashed(&hash);
		sigs.push(signature.0);
		let signed_claim = SignedClaim { claim: complete_claim_data, bytes: sigs };

		assert_err!(
			Reclaim::verify_proof(
				RawOrigin::Signed(source_account_id).into(),
				claim_info,
				signed_claim
			),
			Error::<Test>::SignatureMismatch
		);
	})
}
