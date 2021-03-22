#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

/// A price feed pallet
use frame_support::{pallet_prelude::*, dispatch};
use frame_system::{ensure_signed, pallet_prelude::*};
use stp258_traits::FetchPrice;
use serp_example_ocw::FetchPriceFor;

/// Expected price oracle interface. `fetch_price` must return the amount of Stablecoins exchanged for the tracked value.
impl<T: Trait> FetchPrice<u64> for Module<T> {
    fn fetch_price() -> u64 {
        Self::get_price()
    }
}

/// The pallet's configuration trait.
#[pallet::config]
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    type OffchainPrice: FetchPriceFor;
}

// This pallet's storage items.
#[pallet::storage]
#[pallet::getter(fn get_price)]
#[pallet::metadata(PriceUnit: u64 = "1_000")]
pub(crate) type Price<T: Config> = StorageMap<_, Twox64Concat, CurrencyId, PriceUnit, u64, ValueQuery>;


// The pallet's events
#[pallet::event]
#[pallet::generate_deposit(pub(crate) fn deposit_event)]
#[pallet::metadata(AccountId = "AccountId")]
pub enum Event<T: Config> {NewPrice(u64)}

// The pallet's errors
#[pallet::error]
pub enum Error<T> {
    /// Some wrong behavior
    NoOffchainPrice
}

// The pallet's dispatchable functions.
#[pallet::call]
impl<T: Config> Pallet<T> {
    type Error = Error<T>;

    fn deposit_event() = default;

    #[weight = 0]
    pub fn set_price(origin, new_price: u64) -> dispatch::DispatchResult {
        let _who = ensure_signed(origin)?;

        Price::put(new_price);

        Self::deposit_event(RawEvent::NewPrice(new_price));

        Ok(())
    }

    #[weight = 0]
    pub fn get_offchain_price(origin) -> dispatch::DispatchResult {
        let _who = ensure_signed(origin)?;
        let price = T::OffchainPrice::fetch_price().unwrap();

        native::info!("JUSD/USD offchain price: {}", price);
        Price::put(price);

        Self::deposit_event(RawEvent::NewPrice(price));

        Ok(())
    }
}
