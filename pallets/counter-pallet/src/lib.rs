#![cfg_attr(not(feature = "std"), no_std)]

//! # Counter Pallet
//!
//! This pallet allows each account to store and retrieve a value of type `u32`.
//!
//! ## Features
//!
//! - **set_value**: Allows an account to store a new value. Only the signing account can change its own value.
//! - **get_value**: Allows retrieving the stored value. If an account parameter is provided, the value of the specified account will be returned;
//!                  otherwise, the value of the signing account will be returned.
//!
//! ## Events
//!
//! - **ValueStored**: Emitted after storing a value.
//! - **ValueRetrieved**: Emitted after retrieving a value.
//!
//! ## Errors
//!
//! - **NoneValue**: Returned when there is no stored value for the queried account.

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Main structure of the pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration of the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Type of event emitted by the pallet.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for the pallet's calls.
        type WeightInfo;
    }

    /// Events emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event triggered when a value is stored.
        ///
        /// **Parameters:**
        /// - `T::AccountId`: The account that stored the value.
        /// - `u32`: The stored value.
        ValueStored(T::AccountId, u32),
        /// Event triggered when a value is retrieved.
        ///
        /// **Parameters:**
        /// - `T::AccountId`: The account whose value was retrieved.
        /// - `u32`: The retrieved value.
        ValueRetrieved(T::AccountId, u32),
    }

    /// Possible errors of the pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Error returned when there is no stored value for the queried account.
        NoneValue,
    }

    /// Storage that associates each account (`T::AccountId`) with a value (`u32`).
    #[pallet::storage]
    pub type Values<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    /// Calls (extrinsics) available in the pallet.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Stores a new value for the signing account.
        ///
        /// # Parameters
        ///
        /// - `origin`: The origin of the call, which must be signed.
        /// - `value`: The new value to be stored.
        ///
        /// # Events
        ///
        /// - Emits the `ValueStored` event upon successfully storing the value.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            // Checks if the call is signed and gets the account.
            let account = ensure_signed(origin)?;
            // Stores the value associated with the account.
            Values::<T>::insert(&account, value);
            // Emits the event informing that the value was stored.
            Self::deposit_event(Event::ValueStored(account, value));
            Ok(())
        }

        /// Retrieves the stored value associated with an account.
        ///
        /// # Parameters
        ///
        /// - `origin`: The origin of the call. If the `account` parameter is `None`, the signing account will be used.
        /// - `account`: Optional. If provided, the value of the specified account will be retrieved.
        ///
        /// # Behavior
        ///
        /// - If `account` is `Some(account)`, the pallet retrieves the stored value for that account.
        /// - If `account` is `None`, the signing account (obtained by `ensure_signed(origin)`) will be used for the query.
        ///
        /// In both cases, if there is no stored value for the queried account, the call will return the `NoneValue` error.
        ///
        /// # Events
        ///
        /// - Emits the `ValueRetrieved` event after successfully retrieving the value.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn get_value(origin: OriginFor<T>, account: Option<T::AccountId>) -> DispatchResult {
            match account {
                Some(account) => {
                    let value = Values::<T>::get(&account).ok_or(Error::<T>::NoneValue)?;
                    Self::deposit_event(Event::ValueRetrieved(account, value));
                }
                None => {
                    let my_account = ensure_signed(origin)?;
                    let value = Values::<T>::get(&my_account).ok_or(Error::<T>::NoneValue)?;
                    Self::deposit_event(Event::ValueRetrieved(my_account, value));
                }
            };

            Ok(())
        }
    }
}
