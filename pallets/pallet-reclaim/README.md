
# Pallet Reclaim 
Welcome to the `pallet-reclaim`, a Substrate pallet that provides the core logic for the Reclaim Protocol. This pallet enables the verification of proofs and management of verifiable claims within a Substrate-based blockchain application.

## Overview 
The `pallet-reclaim` is designed to integrate the Reclaim Protocol into Substrate-based blockchains. It defines the `ReclaimVerifier` trait, which other pallets can implement to verify proofs according to the Reclaim logic.
## Features 
 
- **Proof Verification** : Verify proofs and manage verifiable claims.
 
- **Epoch Management** : Add and manage epochs, each containing a set of witnesses.
 
- **Event Emission** : Emits events for significant actions like initialization, proof verification, and epoch addition.
 
- **Error Handling** : Provides detailed errors for troubleshooting.

## Structure 

The pallet consists of the following key files:
 
- **`lib.rs`** : Contains the main pallet logic, including storage definitions, extrinsics, and the implementation of the `ReclaimVerifier` trait.
 
- **`benchmarking.rs`** : Provides benchmarking setup for the pallet to calculate the weights of extrinsics.
 
- **`tests.rs`** : Contains unit tests to ensure the pallet functions correctly.

## Usage 

### Configuration 
To use the `pallet-reclaim` in your Substrate runtime, implement its configuration trait and include it in your runtime.In your runtime's `lib.rs`:** 

```rust
impl pallet_reclaim::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Signature = sp_core::ecdsa::Signature;
    type PublicKey = sp_core::ecdsa::Public;
    type WeightInfo = pallet_reclaim::weights::SubstrateWeightInfo<Runtime>;
}
```
Include the pallet in the `construct_runtime!` macro:

```rust
construct_runtime!(
    pub struct Runtime {
        // -- other pallets --
        Reclaim: pallet_reclaim,
        // -- other pallets --
    }
);
```

### Extrinsics 

The pallet provides the following callable functions:
 
- **`init`** : Initializes the Reclaim Protocol. Must be called before other functionalities are used.

```rust
pub fn init(origin: OriginFor<T>) -> DispatchResult
```
 
- **`add_epoch`** : Adds a new epoch with specified witnesses and minimum witnesses required for claim creation. Only callable by the owner.

```rust
pub fn add_epoch(
    origin: OriginFor<T>,
    witness: BoundedVec<Witness, ConstU32<100>>,
    minimum_witness: u128,
) -> DispatchResult
```
 
- **`verify_proof`** : Verifies a proof according to the Reclaim Protocol.

```rust
pub fn verify_proof(
    origin: OriginFor<T>,
    claim_info: ClaimInfo,
    signed_claim: SignedClaim,
) -> DispatchResult
```

### Storage 
 
- **`PReclaimConfig`** : Stores the Reclaim configuration, including the owner and the current epoch.
 
- **`Epochs`** : Stores epoch information such as witnesses, timestamps, and minimum witnesses required.

### Events 
 
- **`ContractInitialized`** : Emitted when the protocol is initialized.
 
- **`EpochAdded`** : Emitted when a new epoch is added.
 
- **`ProofVerified`** : Emitted when a proof is successfully verified.

### Errors 
 
- **`OnlyOwner`** : Thrown when a non-owner attempts an owner-restricted action.
 
- **`AlreadyInitialized`** : Thrown when initialization is attempted more than once.
 
- **`HashMismatch`** : Thrown when there is a hash mismatch during proof verification.
 
- **`LengthMismatch`** : Thrown when there is a length mismatch in expected data.
 
- **`SignatureMismatch`** : Thrown when signatures do not match the expected witnesses.

## Integration 
Other pallets can integrate with `pallet-reclaim` by utilizing the `ReclaimVerifier` trait.Example in another pallet's `Config` trait:** 

```rust
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type ReclaimVerifier: ReclaimVerifier<Proof>;
    type WeightInfo: WeightInfo;
}
```
Implementing the `ReclaimVerifier` trait:** 

```rust
impl<T> ReclaimVerifier<Proof> for Pallet<T>
where
    T: Config,
{
    fn verify_proof(proof: &Proof) -> DispatchResult {
        // Call the verify_proof function from pallet-reclaim
        pallet_reclaim::Pallet::<T>::verify_proof(proof)
    }
}
```

## Benchmarking 

Benchmarking is essential for calculating the weights of extrinsics.
**Building with Benchmarking Features:** 

```bash
cargo build --release --features runtime-benchmarks
```
**Running Benchmarks:** 

```bash
./target/release/node-template benchmark pallet \
    --chain dev \
    --wasm-execution=compiled \
    --pallet pallet_reclaim \
    --extrinsic '*' \
    --steps 50 \
    --repeat 20 \
    --output ./runtime/src/weights.rs
```
This command benchmarks all extrinsics in the `pallet_reclaim` and updates the `weights.rs` file with the results.
## Testing 
Unit tests are provided in `tests.rs` to ensure the pallet functions correctly.**Running Tests:** 

```bash
cargo test -p pallet-reclaim --lib
```
Example Test in `tests.rs`:** 

```rust
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
```
