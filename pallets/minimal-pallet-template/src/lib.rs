//! # The Minimal Pallet
//!
//! This pallet serves to store and retrieve a number on the blockchain.
//! Main functionalities:
//! - `set_number`: Allows a user to write a new number and emits an event.
//! - `get_number`: Allows a user to read a number.
//! - `cause_error`: Attempts to increment the stored number and returns an error if the number is not set or if an overflow occurs.
//!
//! The sections below document the configuration, storage, events, errors, and functions of this pallet.

#![cfg_attr(not(feature = "std"), no_std)]

// Modules for testing and runtime environment simulation.
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    // Imports essential macros and types for the pallet.
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // Main structure of the pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);
    
    // Pallet configuration: defines the configurations required for operation
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Type of event that this pallet will emit.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type that represents the weight (computational cost) of the pallet's calls.
        type WeightInfo;
    }

    // Events that will be emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a new number is successfully stored.
        /// - number: The new number stored.
        /// - account: The account that performed the operation.
        NewCounterStored {
            number: u32,
            account: T::AccountId,
        },
        /// Event emitted when a number is successfully retrieved.
        /// - number: The number retrieved.
        NumberRetrieved {
            number: u32,
        },
    }

    // Errors that may be returned during the execution of the pallet's functions.
    #[pallet::error]
    pub enum Error<T> {
        /// Error returned when trying to access a value that has not yet been set.
        NoneValue,
        /// Error returned when attempting to increment a value that has already reached its maximum (overflow).
        StorageOverflow,
    }

    // Main storage of the pallet: stores a number of type u32.
    #[pallet::storage]
    pub type Number<T> = StorageValue<_, u32>;

    // Implementation of the pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Function to store a new number.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn set_number(origin: OriginFor<T>, new_number: u32) -> DispatchResult {
            // Checks that the transaction is signed and retrieves the responsible account.
            let account = ensure_signed(origin)?;

            // Updates the storage with the new number.
            Number::<T>::put(new_number);

            // Emits an event indicating that the new number has been stored.
            Self::deposit_event(Event::NewCounterStored {
                number: new_number,
                account,
            });

            // Indicates that the operation completed successfully.
            Ok(())
        }

        // Function that attempts to increment the stored number.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            // Checks that the transaction is signed.
            let _who = ensure_signed(origin)?;

            // Attempts to read the stored value.
            match Number::<T>::get() {
                // If no value is set, returns the NoneValue error.
                None => Err(Error::<T>::NoneValue.into()),
                Some(n) => {
                    // Attempts to increment the value; if an overflow occurs, returns the StorageOverflow error.
                    let new = n.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Updates the storage with the incremented value.
                    Number::<T>::put(new);
                    Ok(())
                }
            }
        }

        // Function that returns the stored number.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::default())]
        pub fn get_number(origin: OriginFor<T>) -> DispatchResult {
            // Checks that the transaction is signed.
            let _who = ensure_signed(origin)?;

            // Attempts to read the stored value.
            match Number::<T>::get() {
                Some(n) => {
                    // Emits an event with the retrieved number.
                    Self::deposit_event(Event::NumberRetrieved { number: n });
                    Ok(())
                }
                None => Err(Error::<T>::NoneValue.into()),
            }
        }
    }
}