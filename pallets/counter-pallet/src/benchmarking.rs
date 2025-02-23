#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_value() {
        let caller: T::AccountId = whitelisted_caller();
        
        #[extrinsic_call]
        set_value(RawOrigin::Signed(caller.clone()), 100);

        assert_eq!(Values::<T>::get(caller), Some(100));
    }

    #[benchmark]
    fn get_value_own() {
        let caller: T::AccountId = whitelisted_caller();
        Values::<T>::insert(&caller, 100);

        #[extrinsic_call]
        get_value(RawOrigin::Signed(caller.clone()), None);

        assert_eq!(Values::<T>::get(caller), Some(100));
    }

    #[benchmark]
    fn get_value_other() {
        let caller: T::AccountId = whitelisted_caller();
        let other: T::AccountId = account("other", 0, 0);
        Values::<T>::insert(&other, 100);

        #[extrinsic_call]
        get_value(RawOrigin::Signed(caller), Some(other.clone()));

        assert_eq!(Values::<T>::get(other), Some(100));
    }
}