use frame_support::dispatch::DispatchResult;

pub trait ReclaimVerifier<Proof, Epoch> {
	fn verify_proof(proof: &Proof, current_epoch: &Epoch) -> DispatchResult;
}

impl<Proof, Epoch> ReclaimVerifier<Proof, Epoch> for () {
	fn verify_proof(_proof: &Proof, _current_epoch: &Epoch) -> DispatchResult {
		unimplemented!()
	}
}
