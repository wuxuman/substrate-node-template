
use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller,account};
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks!{
      create_claim{
            let d in 0..T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller:T::AccountId=whitelisted_caller();
      }:_(RawOrigin::Signed(caller),claim)

      revoke_claim{
            let d in 0..T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller:T::AccountId=whitelisted_caller();
            assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone()).is_ok());

      }:_(RawOrigin::Signed(caller),claim)

      transfer_claim{
            let d in 0..T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller:T::AccountId=whitelisted_caller();
		let receiver: T::AccountId = account("target", 0, 0);
            assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone()).is_ok());

      }:_(RawOrigin::Signed(caller),claim,receiver)

      impl_benchmark_test_suite!(PoeModule,crate::mock::new_test_ext(),crate::mock::Test);

}