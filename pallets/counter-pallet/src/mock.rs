//! # Mock Runtime for Testing
//!
//! This module provides a mock runtime environment for testing the `counter_pallet` in isolation.
//! It simulates the Substrate runtime by defining a `Runtime` enum, configuring necessary pallets,
//! and providing utilities to initialize a test environment.
//!
//! ## Key Components
//! - **`Runtime`**: The mock runtime, combining the `frame_system` and `counter_pallet` pallets.
//! - **`frame_system::Config`**: Configuration for the `frame_system` pallet, using `MockBlock` and `u64` as `AccountId`.
//! - **`counter_pallet::Config`**: Configuration for the `counter_pallet`, specifying `RuntimeEvent` and `WeightInfo`.
//! - **`new_test_ext`**: A helper function to initialize a test environment with a default genesis configuration.
//!
//! ## Usage
//! Use `new_test_ext()` to create a test environment with a clean state and a starting block number of 1.
//! This allows you to test your pallet logic in a controlled and reproducible environment.

use crate::pallet as counter_pallet;
use frame_support::{construct_runtime, derive_impl};
use frame_system::mocking::MockBlock;
use sp_runtime::BuildStorage;

// Define the mock runtime by combining the `frame_system` and `counter_pallet` pallets.
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Counter: counter_pallet,
    }
);

// Implement the `frame_system::Config` trait for the mock runtime.
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = MockBlock<Runtime>;
    type AccountId = u64;
}

// Implement the `counter_pallet::Config` trait for the mock runtime.
impl counter_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

/// Initializes a test environment with a default genesis configuration and sets the block number to 1.
///
/// # Returns
/// - `sp_io::TestExternalities`: A test externalities instance for running tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Runtime>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        frame_system::Pallet::<Runtime>::set_block_number(1);
    });
    ext
}
