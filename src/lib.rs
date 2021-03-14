//! A simple price feed pallet
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use sp_std::prelude::*;

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::{
		AccountIdConversion, AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member,
		Saturating, StaticLookup, Zero,
	},
	DispatchError, DispatchResult, ModuleId, RuntimeDebug,
};
use frame_system::{self as system, ensure_signed, pallet_prelude::*};
use stp258_traits::{Stp258Currency};

mod mock;
mod tests;

pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

    #[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

    /// The pallet's configuration trait.
   #[pallet::config]
	pub trait Config: frame_system::Config {
        // Add other types and constants required to configure this pallet.

        /// The overarching event type.
       type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
        /// The currency ID type
        type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord;
    }

    // This pallet's storage items.
    /// The Price of a currency type.
    #[pallet::storage]
    #[pallet::getter(fn price)]
    pub type Price<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::CurrencyId,
        Twox64Concat,
        T::Balance,
        ValueQuery,
    >;

    // The pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        /// The New Price of Currency. [currency_id, price]
        NewPrice(u32),
    }


    // The pallet's errors
    #[pallet::error]
    pub enum Error<T> {
        /// Some wrong behavior
        Wrong,
        /// Something went very wrong and the price of the currency is zero.
        ZeroPrice,
        /// No offchain price available.
        NoOffchainPrice,
        /// While trying to contract the supply, it underflowed.
        SupplyUnderflow,
    }

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}
    
    #[pallet::call]
	impl<T: Config> Pallet<T> {

        #[pallet::weight(0)]
        pub fn set_price(origin: OriginFor<T>, currency_id: T::CurrencyId, new_price: u32) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            Price::put(currency_id, new_price);

            Self::deposit_event(Event::NewPrice(currency_id, new_price));

            Ok(())
        }
    }
}