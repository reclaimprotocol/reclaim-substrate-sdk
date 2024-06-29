use frame_support::dispatch::DispatchResult;

pub trait ReclaimVerifier<Proof> {
	fn verify_proof(proof: &Proof) -> DispatchResult;
}

impl<Proof> ReclaimVerifier<Proof> for () {
	fn verify_proof(_proof: &Proof) -> DispatchResult {
		unimplemented!()
	}
}
