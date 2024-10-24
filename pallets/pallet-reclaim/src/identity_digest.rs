use digest::{
	consts::U32, generic_array::GenericArray, FixedOutput, HashMarker, Output, OutputSizeUser,
	Reset, Update,
};

/// A container for a 256-bit identity, represented as a fixed-size array.
#[derive(Clone, Default)]
pub struct Identity256 {
	array: GenericArray<u8, U32>,
}

impl Update for Identity256 {
    	/// Updates the identity with a new 32-byte hash.
	fn update(&mut self, hash: &[u8]) {
		assert_eq!(hash.as_ref().len(), 32); // Ensure the input length is 32 bytes.
		self.array = *GenericArray::from_slice(hash);	// Update the array with the new hash.
	}
}

impl OutputSizeUser for Identity256 {
	type OutputSize = U32;
}

impl FixedOutput for Identity256 {
    /// Finalizes the hashing process and outputs the result.
	fn finalize_into(self, out: &mut Output<Self>) {
		*out = self.array;
	}
}

impl HashMarker for Identity256 {}	// Marker trait for identifying as a hash function.

impl Reset for Identity256 {
    /// Resets the identity back to its default state.
	fn reset(&mut self) {
		*self = Self::default();
	}
}
