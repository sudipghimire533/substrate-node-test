use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn expected_to_rollback() {
	new_test_ext().execute_with(|| {
		// Let's first set both storage to value 10
		assert_ok!(TemplateModule::change_both_passing(Origin::signed(1), 10));

		// Ensure both are as expected
		assert_eq!(10, TemplateModule::get_first_storage());
		assert_eq!(10, TemplateModule::get_second_storage());

		// Call an mid-failing tx
		// it should fail with an error
		assert!(TemplateModule::change_both_failing(Origin::signed(1), 20).is_err());

		// We expect the transaction to rollback
		// so both storage should be 10 still
		assert_eq!(10, TemplateModule::get_first_storage());
		assert_eq!(10, TemplateModule::get_second_storage());
	})
}

#[test]
fn expected_to_not_rollback() {
	new_test_ext().execute_with(|| {
		// Let's first set both storage to value 10
		assert_ok!(TemplateModule::change_both_passing(Origin::signed(1), 10));

		// Ensure both are as expected
		assert_eq!(10, TemplateModule::get_first_storage());
		assert_eq!(10, TemplateModule::get_second_storage());

		// Call an mid-failing tx
		// it should fail with an error
		assert!(TemplateModule::change_both_failing(Origin::signed(1), 20).is_err());

		// We expect the transaction to rollback
		// so both storage should be 10 still
		assert_eq!(20, TemplateModule::get_first_storage());
		assert_eq!(10, TemplateModule::get_second_storage());
	})
}
