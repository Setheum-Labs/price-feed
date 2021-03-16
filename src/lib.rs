//! A simple price feed pallet
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use sp_std::prelude::*;

use frame_support::{pallet_prelude::*, dispatch::PostDispatchInfo,};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member
	},
};
use frame_system::{ensure_signed, pallet_prelude::*};

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
        
        /// The balance type
        type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize;

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
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The New Price of Currency. [currency_id, price]
        NewPrice(T::CurrencyId, u32),
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

            Price::insert(currency_id, new_price);

            Self::deposit_event(Event::NewPrice(currency_id, new_price));
            Ok(().into())
        }
    }
}