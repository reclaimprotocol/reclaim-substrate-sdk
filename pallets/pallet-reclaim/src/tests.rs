use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;

#[test]
fn init() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		System::assert_has_event(Event::ContractInitialized { owner: source_account_id }.into());
		assert_eq!(
			Reclaim::reclaim_config(),
			Some(ReclaimConfig { owner: source_account_id, current_epoch: 0_u64 })
		)
	})
}

#[test]
fn should_add_epochs() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		System::set_block_number(1);
		assert_ok!(Reclaim::init(RawOrigin::Signed(source_account_id).into()));
		let addr = hex::decode("244897572368eadf65bfbc5aec98d8e5443a9072").unwrap();
		let addr_slice = addr.get(0..20).unwrap();

		let mut addr_normzlized: [u8; 20] = [0_u8; 20];

		for i in 0..20 {
			addr_normzlized[i] = addr_slice[i]
		}

		let w1 = Witness { address: addr_normzlized, host: [1_u8; 32] };

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
		let addr = hex::decode("244897572368eadf65bfbc5aec98d8e5443a9072").unwrap();
		let addr_slice = addr.get(0..20).unwrap();

		let mut addr_normzlized: [u8; 20] = [0_u8; 20];

		for i in 0..20 {
			addr_normzlized[i] = addr_slice[i]
		}

		let w1 = Witness { address: addr_normzlized, host: [1_u8; 32] };

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
            provider: "http".to_string(),
            parameters: "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"regex\",\"value\":\"_steamid\\\">Steam ID: (?<CLAIM_DATA>.*)</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (?<CLAIM_DATA>.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}".to_string(),
            context: "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\",\"extractedParameters\":{\"CLAIM_DATA\":\"76561199601812329\"},\"providerHash\":\"0xffd5f761e0fb207368d9ebf9689f077352ab5d20ae0a2c23584c2cd90fc1b1bf\"}".to_string(),
        };
		let complete_claim_data = CompleteClaimData {
            identifier: "0xd1dcfc5338cb588396e44e6449e8c750bd4d76332c7e9440c92383382fced0fd"
                .to_string(),
            owner: "0x13239fc6bf3847dfedaf067968141ec0363ca42f".to_string(),
            epoch: 1_u64,
            timestampS: 1712174155_u64,
        };

		let mut sigs = Vec::<String>::new();

        let str_signature = "2888485f650f8ed02d18e32dd9a1512ca05feb83fc2cbf2df72fd8aa4246c5ee541fa53875c70eb64d3de9143446229a250c7a762202b7cc289ed31b74b31c811c".to_string();

        sigs.push(str_signature);

        let signed_claim = SignedClaim {
            claim: complete_claim_data,
            signatures: sigs,
        };

		assert_ok!(Reclaim::verify_proof(
			RawOrigin::Signed(source_account_id).into(),
			claim_info,
			signed_claim
		));
	})
}
