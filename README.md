<div>
    <div>
        <img src="https://raw.githubusercontent.com/reclaimprotocol/.github/main/assets/banners/Substrate-SDK.png"  />
    </div>
</div>

# Substrate SDK for Reclaim Protocol 



## Introduction 

This repository is a Substrate SDK designed to integrate the Reclaim Protocol into Substrate-based blockchains. It provides the necessary pallets and configurations to verify proofs and manage verifiable claims within your blockchain applications.

The SDK includes:
 
- **Pallet Reclaim** : Contains the core logic for verifying proofs according to the Reclaim Protocol.
 
- **Pallet Integration with Reclaim** : An example pallet demonstrating how to integrate `ReclaimVerifier` trait into your own pallets.

## Prerequisites 
 
- Familiarity with the [Substrate Framework](https://docs.substrate.io/quick-start/) .
 
- Rust programming language installed for setup instructions).
 
- Yarn installed for the frontend.


## Getting Started 

### Clone the Repository 

Clone the repository to your local machine:


```bash
git clone https://github.com/reclaimprotocol/substrate-sdk.git
cd substrate-sdk
```

### Building the Node 

Build the Substrate node with the Reclaim Protocol integrated:


```bash
cargo build --release
```

### Testing 

Run the tests for the entire project:


```bash
cargo test
```
To test only the `pallet-reclaim` functionality:

```bash
cargo test -p pallet-reclaim --lib
```

### Running the Node 

Start the node in development mode:


```bash
./target/release/node-template --dev
```

### Frontend Setup 

In a separate terminal, navigate to the frontend directory and start the frontend interface:


```bash
cd substrate-front-end-template
yarn install
yarn start
```

This will launch the frontend interface connected to your local node.

## Code Overview 

This section provides an overview of the key components in the repository.

### Pallet Reclaim 
**Location** : `pallets/pallet-reclaim`The `pallet_reclaim` contains the core logic of the Reclaim Protocol, including: 
- **Traits** : Defines the `ReclaimVerifier` trait used for verifying proofs.

```rust
use frame_support::dispatch::DispatchResult;

pub trait ReclaimVerifier<Proof> {
    fn verify_proof(proof: &Proof) -> DispatchResult;
}

impl<Proof> ReclaimVerifier<Proof> for () {
    fn verify_proof(_proof: &Proof) -> DispatchResult {
        unimplemented!()
    }
}
```
 
- **Implementation** : Implements the `ReclaimVerifier` trait.

```rust
impl<T> ReclaimVerifier<Proof> for Pallet<T>
where
    T: Config,
{
    fn verify_proof(proof: &Proof) -> DispatchResult {
        let config = <PReclaimConfig<T>>::get().unwrap();
        let epoch_count = config.current_epoch;
        let current_epoch = <Epochs<T>>::get(epoch_count);
        // Verification logic
        Ok(())
    }
}
```
 
- **Extrinsics** : Provides callable functions for managing the Reclaim Protocol, such as initializing configurations, adding epochs, and verifying proofs.

```rust
// For Management. Setup Variables for Reclaim Protocol
pub fn init(origin: OriginFor<T>) -> DispatchResult {
    // Initialization logic
}

// For Management. Only callable by reclaim manager account (initializer account)
pub fn add_epoch(
    origin: OriginFor<T>,
    witness: BoundedVec<Witness, ConstU32<100>>,
    minimum_witness: u128,
) -> DispatchResult {
    // Epoch addition logic
}

// Anyone can call to check if their proofs are valid.
pub fn verify_proof(
    origin: OriginFor<T>,
    claim_info: ClaimInfo,
    signed_claim: SignedClaim,
) -> DispatchResult {
    // Proof verification logic
}
```

### Pallet Integration with Reclaim 
**Location** : `pallets/pallet-integration-with-reclaim`This is an example pallet that demonstrates how to integrate the `ReclaimVerifier` trait into your own pallet. 
- **Configuration** : Adds `ReclaimVerifier` as an associated type.

```rust
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type ReclaimVerifier: ReclaimVerifier<Proof>;
    type WeightInfo: WeightInfo;
}
```
 
- **Storage** : Stores verification status for accounts.

```rust
pub(super) type AccountVerified<T: Config> = StorageMap<_, Identity, T::AccountId, bool, OptionQuery>;
```
 
- **Extrinsics** : Provides a `verify_user` function that verifies a user's proof and updates their verification status.

```rust
#[pallet::call_index(0)]
#[pallet::weight(<T as pallet::Config>::WeightInfo::verify_user())]
pub fn verify_user(origin: OriginFor<T>, proof: Proof) -> DispatchResult {
    let who = ensure_signed(origin)?;
    // Call `verify_proof`. If verification fails, it will raise a `Reclaim` error and revert.
    T::ReclaimVerifier::verify_proof(&proof)?;
    <AccountVerified<T>>::insert(&who, true);
    Self::deposit_event(Event::UserVerified { account_id: who });
    Ok(())
}
```

### Runtime Configuration 
**Location** : `runtime/src/lib.rs`
The runtime configuration integrates the pallets into the Substrate runtime.
 
- **Implement Configs for Runtime** : Implements the configurations for `pallet_reclaim` and `pallet_integration_with_reclaim`.

```rust
impl pallet_reclaim::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Signature = sp_core::ecdsa::Signature;
    type PublicKey = sp_core::ecdsa::Public;
    type WeightInfo = pallet_reclaim::weights::SubstrateWeightInfo<Runtime>;
}

impl pallet_integration_with_reclaim::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ReclaimVerifier = pallet_reclaim::Pallet<Runtime>;
    type WeightInfo = pallet_integration_with_reclaim::weights::SubstrateWeightInfo<Runtime>;
}
```
 
- **Include Pallets in Runtime** : Adds the pallets to the `construct_runtime!` macro.

```rust
construct_runtime!(
    pub struct Runtime {
        // Other pallets
        Reclaim: pallet_reclaim,
        IntegrationWithReclaim: pallet_integration_with_reclaim,
    }
);
```
 
- **Benchmarking Configuration** : Adds pallets to the benchmarking module.

```rust
mod benches {
    define_benchmarks!(
        // Other benchmarks
        [pallet_reclaim, Reclaim]
        [pallet_integration_with_reclaim, IntegrationWithReclaim]
    );
}
```

## Benchmarking 

Benchmarking helps to calculate the weights for extrinsics.

### Building for Benchmarking 

Build the node with the benchmarking features:


```bash
cargo build --release --features runtime-benchmarks
```

### Running Benchmarks 

Run the benchmarking for a specific pallet:


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
This command benchmarks all extrinsics (`'*'`) in the `pallet_reclaim` and updates the `weights.rs` file.

## Contributing to Our Project

We're excited that you're interested in contributing to our project! Before you get started, please take a moment to review the following guidelines.

## Code of Conduct

Please read and follow our [Code of Conduct](https://github.com/reclaimprotocol/.github/blob/main/Code-of-Conduct.md) to ensure a positive and inclusive environment for all contributors.

## Security

If you discover any security-related issues, please refer to our [Security Policy](https://github.com/reclaimprotocol/.github/blob/main/SECURITY.md) for information on how to responsibly disclose vulnerabilities.

## Contributor License Agreement

Before contributing to this project, please read and sign our [Contributor License Agreement (CLA)](https://github.com/reclaimprotocol/.github/blob/main/CLA.md).

## Indie Hackers

For Indie Hackers: [Check out our guidelines and potential grant opportunities](https://github.com/reclaimprotocol/.github/blob/main/Indie-Hackers.md)

## License

This project is licensed under a [custom license](https://github.com/reclaimprotocol/.github/blob/main/LICENSE). By contributing to this project, you agree that your contributions will be licensed under its terms.

Thank you for your contributions!
