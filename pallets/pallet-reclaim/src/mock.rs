use crate as pallet_reclaim;
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

// Mock block type for testing
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Reclaim: pallet_reclaim,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type MinimumPeriod = ();
	type OnTimestampSet = ();
	type WeightInfo = ();
}

impl pallet_reclaim::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Signature = sp_core::ecdsa::Signature;
	type PublicKey = sp_core::ecdsa::Public;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

// Constants for testing
const _ZERO_ADDRESS: &str = "00000000000000000000000000000000000000000000000000000000000000";
const _ZERO_HOST: &str = "00000000000000000000000000000000000000000000000000000000000000";
const _MOCK_IDENTIFIER: &str = "0x27da6b2ce5887ac6a0084b57a5239f4ead79acfeab3aab84d588e907f66daf3f";
const _MOCK_OWNER: &str = "0x033f29e3b7f3b9b1486b5492c9e4dd2d197fc132ce5be6d4fdc0d7df72a82ec50a";
const _MOCK_SIGNATURE: &str = "0x5fc263049dd7e487dde835ae14c81166d10f630d001726a6440491264a96d73e0ba68168fec50c3803c7d9dcecf20fc47c053d0142055e010f4c41f87f0a3d9600";
