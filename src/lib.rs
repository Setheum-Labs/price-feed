#![cfg_attr(not(feature = "std"), no_std)]

/// A price feed pallet
use frame_support::{debug::native, decl_error, decl_event, decl_module, decl_storage, dispatch};
use sp_runtime::{
	traits::{AtLeast32Bit, CheckedAdd, CheckedDiv, CheckedMul, MaybeSerializeDeserialize, Member, Zero}, 
	DispatchResult, PerThing, Perbill, RuntimeDebug,
};
use frame_system::{self as system, ensure_signed, pallet_prelude::*};
use stp258_traits::{Stp258Currency};

mod mock;
mod tests;

impl<T: Trait> FetchPrice<u32> for Module<T> {
    fn fetch_price() -> u32 {
        Self::get_price()
    }
}

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The currency ID type
	type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Price {
        Price get(fn get_price): u32 = 1_000;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    {
        NewPrice(u32),

    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        NoOffchainPrice
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 0]
        pub fn set_price(origin, currency_id: CurrencyId, new_price: u32) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            Price::put(currency_id, new_price);

            Self::deposit_event(RawEvent::NewPrice(currency_id, new_price));

            Ok(())
        }
    }
}
