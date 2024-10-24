//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::Pallet as IntegrationWithReclaim;
use frame_benchmarking::{account, benchmarks};
use frame_support::{assert_ok, BoundedVec};
use frame_system::RawOrigin;
use pallet_reclaim::{ClaimInfo, CompleteClaimData, Pallet as Reclaim, SignedClaim, Witness};
use scale_info::prelude::{fmt::Debug, format, string::String, vec, vec::Vec};
use sp_core::ConstU32;

benchmarks! {
	where_clause {
		where
			T: Config + pallet_reclaim::Config,
	}
// Benchmark for the verify_user extrinsic
  verify_user{
	// Set up test account
	let source_account_id: T::AccountId = account("acc1", 0,0);
	assert_ok!(Reclaim::<T>::init(RawOrigin::Signed(source_account_id.clone()).into()));

		// Prepare a witness address
		let addr = hex::decode("244897572368eadf65bfbc5aec98d8e5443a9072").unwrap();
		let addr_slice = addr.get(0..20).unwrap();

		// Normalize the address
		let mut addr_normzlized: [u8; 20] = [0_u8; 20];

		for i in 0..20 {
			addr_normzlized[i] = addr_slice[i]
		}
		// Create a witness
		let w1 = Witness { address: addr_normzlized, host: [1_u8; 32] };

		let mut witnesses_vec = Vec::<Witness>::new();
		witnesses_vec.push(w1);
		// Set up witnesses and add epoch
		let witnesses: BoundedVec<Witness, ConstU32<100>> =
			BoundedVec::<Witness, ConstU32<100>>::try_from(witnesses_vec).unwrap();
		let minimum_witness = 1;
		assert_ok!(Reclaim::<T>::add_epoch(
			RawOrigin::Signed(source_account_id.clone()).into(),
			witnesses,
			minimum_witness
		));

		// Prepare claim information
		let claim_info = ClaimInfo {
			provider: String::from("http"),
			parameters: String::from("{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"regex\",\"value\":\"_steamid\\\">Steam ID: (?<CLAIM_DATA>.*)</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (?<CLAIM_DATA>.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}"),
			context: String::from("{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\",\"extractedParameters\":{\"CLAIM_DATA\":\"76561199601812329\"},\"providerHash\":\"0xffd5f761e0fb207368d9ebf9689f077352ab5d20ae0a2c23584c2cd90fc1b1bf\"}"),
		};

		// Create signed claim data
		let complete_claim_data = CompleteClaimData {
			identifier: String::from("0xd1dcfc5338cb588396e44e6449e8c750bd4d76332c7e9440c92383382fced0fd"),
			owner: String::from("0x13239fc6bf3847dfedaf067968141ec0363ca42f"),
			epoch: 1_u64,
			timestampS: 1712174155_u64,
		};

		let mut sigs = Vec::<String>::new();

		let str_signature = String::from("2888485f650f8ed02d18e32dd9a1512ca05feb83fc2cbf2df72fd8aa4246c5ee541fa53875c70eb64d3de9143446229a250c7a762202b7cc289ed31b74b31c811c");

		sigs.push(str_signature);

		let signed_claim = SignedClaim {
			claim: complete_claim_data,
			signatures: sigs,
		};
		// Create proof
		let proof = Proof {
			claimInfo: claim_info,
			signedClaim: signed_claim,
		};

  } : {
		// Call verify_user extrinsic
		assert_ok!(IntegrationWithReclaim::<T>::verify_user(
			RawOrigin::Signed(source_account_id.clone()).into(),
			proof
		));
  }
  verify{
	// Verify the result of the verification
	assert_eq!(IntegrationWithReclaim::<T>::account_verified(source_account_id), Some(true));
  }



}
