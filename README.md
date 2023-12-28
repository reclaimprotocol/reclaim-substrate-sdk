# Implementation of the Reclaim Protocol in Substrate.

## Node

Build
```bash
cargo build --release
```

Test
```bash
cargo test
```

Test only pallet functionality
```bash
cargo test -p pallet-reclaim --lib 
```

Run node dev
```bash
./target/release/node-template --dev
```

## Benchmarking
Build
```bash
cargo build --release --features runtime-benchmarks
```

Run
```bash
./target/release/node-template benchmark pallet \
    --chain dev \
    --wasm-execution=compiled \
    --pallet pallet-reclaim \
    --extrinsic '*' \
    --steps 50 \
    --repeat 20 \
    --output ./runtime/src/weights.rs
```