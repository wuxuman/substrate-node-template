
use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks!{
      create_claim{
            let d in 0..T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller:T::AccountId=whitelisted_caller();
      }:_(RawOrigin::Signed(caller),claim)

      impl_benchmark_test_suite!(PoeModule,crate::mock::new_test_ext(),crate::mock::Test);

}