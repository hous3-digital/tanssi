#![cfg(test)]

//! # Tests for Counter Pallet
//!
//! This module contains unit tests for the `counter_pallet`, ensuring its functionality works as expected.
//! The tests cover the following scenarios:
//!
//! - **`set_value`**: Verifies that a value is correctly stored and that the `ValueStored` event is emitted.
//! - **`get_value`**: Tests the retrieval of stored values, both for the caller's account (`None`) and for another account (`Some(account)`).
//! - **Error Handling**: Ensures the `NoneValue` error is returned when attempting to retrieve a value that has not been set.
//!
//! ## Test Cases
//! - `set_value_works`: Tests storing a value and verifies the `ValueStored` event.
//! - `get_value_works_with_none`: Tests retrieving the caller's stored value using `None`.
//! - `get_value_works_with_some`: Tests retrieving another account's stored value using `Some(account)`.
//! - `get_value_fails_when_no_value_set`: Tests error handling when retrieving a value that has not been set.

use crate::mock::{new_test_ext, Runtime, System};
use crate::Error;
use crate::Event as CounterEvent;
use crate::Values;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_works() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 42.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            42
        ));

        // Verify the value is stored correctly.
        assert_eq!(Values::<Runtime>::get(&1), Some(42));

        // Check if the `ValueStored` event was emitted.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueStored(account, value)) =
                record.event
            {
                account == 1 && value == 42
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueStored event not found");
    });
}

#[test]
fn get_value_works_with_none() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 55.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            55
        ));

        // Account 1 retrieves its own value by passing `None`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            None
        ));

        // Check if the `ValueRetrieved` event was emitted with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 1 && value == 55
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}

#[test]
fn get_value_works_with_some() {
    new_test_ext().execute_with(|| {
        // Account 2 stores the value 99.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(2).into(),
            99
        ));

        // Account 1 retrieves the value stored for account 2 by passing `Some(2)`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            Some(2)
        ));

        // Check if the `ValueRetrieved` event was emitted for account 2 with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 2 && value == 99
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}

#[test]
fn get_value_fails_when_no_value_set() {
    new_test_ext().execute_with(|| {
        // Attempt to retrieve the value of an account (3) that has never stored a value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(1).into(), Some(3)),
            Error::<Runtime>::NoneValue
        );

        // Attempt to retrieve the value of the caller's account (4), which has no stored value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(4).into(), None),
            Error::<Runtime>::NoneValue
        );
    });
}
