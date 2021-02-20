// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn set_price_works() {
	new_test_ext().execute_with(|| {
		// Just a set_price test for the `set_price` function `
		// calling the `set_price` function with a value 42
		assert_ok!(PriceModule::set_price(Origin::signed(1), 42));
		// asserting that the stored value is equal to what we stored
		assert_eq!(PriceModule::get_price(), 42);
	});
}
