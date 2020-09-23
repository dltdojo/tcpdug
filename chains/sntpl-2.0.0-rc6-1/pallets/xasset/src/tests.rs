use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_eq!(Balances::free_balance(1), 1000);
		assert_eq!(Balances::free_balance(2), 1000);
		assert_ok!(TemplateModule::transfer_funds(Origin::signed(1), 2, 100));
		// Read pallet storage and assert an expected result.
		assert_eq!(Balances::free_balance(1), 900);
		assert_eq!(Balances::free_balance(2), 1100);
	});
}

#[test]
fn transfer_token() {
	new_test_ext().execute_with(|| {
		let asset99 = 99;
		let account1 = 1;
		let account2 = 2;
		assert_eq!(GenericAsset::free_balance(&asset99,&account1), 8899);
		assert_eq!(Balances::free_balance(account1), 1000);
		assert_ok!(TemplateModule::transfer_token(Origin::signed(account1), account2, asset99, 7 ));
		assert_eq!(Balances::free_balance(account1), 1000);
		assert_eq!(GenericAsset::free_balance(&asset99,&account1), 8892);
		assert_eq!(GenericAsset::free_balance(&asset99,&account2), 8906);
	});
}