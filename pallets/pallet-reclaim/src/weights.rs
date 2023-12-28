use frame_support::weights::Weight;

pub trait WeightInfo {
	fn init() -> Weight;

	fn verify_proof() -> Weight;

	fn add_epoch() -> Weight;
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
