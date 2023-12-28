//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Reclaim;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
  init {
	  let src_account: T::AccountId = account("acc1", 0,0);
	  let current_epoch: u128 = 0;
  }:_(RawOrigin::Signed(src_account.clone()))
  verify {
	  assert_eq!(Reclaim::<T>::reclaim_config(), Some(ReclaimConfig{
		owner: src_account,
		current_epoch: 0_u128,
	}));

  }

}
