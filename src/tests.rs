use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn owner_of_unexistent_collection_is_none() {
		new_test_ext().execute_with(|| {
			assert_eq!(LivingAssetsModule::owner_of_collection(0), None);
			assert_eq!(LivingAssetsModule::owner_of_collection(1), None);
		});
	}

	#[test]
	fn create_new_collection() {
		new_test_ext().execute_with(|| {
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0));
			assert_eq!(LivingAssetsModule::owner_of_collection(0), Some(1));
		});
	}

	#[test]
	fn create_an_existing_collection_should_fail() {
		new_test_ext().execute_with(|| {
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0));
			assert_noop!(
				LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0),
				Error::<Test>::CollectionAlreadyExists
			);
		});
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);
			// Dispatch a signed extrinsic.
			assert_ok!(LivingAssetsModule::do_something(RuntimeOrigin::signed(1), 42));
			// Read pallet storage and assert an expected result.
			assert_eq!(LivingAssetsModule::something(), Some(42));
			// Assert that the correct event was deposited
			System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
		});
	}

	#[test]
	fn correct_error_for_none_value() {
		new_test_ext().execute_with(|| {
			// Ensure the expected error is thrown when no value is present.
			assert_noop!(
				LivingAssetsModule::cause_error(RuntimeOrigin::signed(1)),
				Error::<Test>::NoneValue
			);
		});
	}
}
