use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::Proofs;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn create_claim_failed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let claim1 = vec![1, 2, 3, 4, 5, 6];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()), 
			Error::<Test>::ProofAlreadyExist
		);
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim1.clone()), 
			Error::<Test>::ClaimTooLarge
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn revoke_claim_failed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let claim1 = vec![1, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim1.clone()), 
			Error::<Test>::ClaimNotExist
		);

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()), 
			Error::<Test>::NotClaimOwner
		);
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 100));
		assert_eq!(Proofs::<Test>::get(&claim), Some((100, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn transfer_claim_failed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let claim1 = vec![1, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim1.clone(), 100), 
			Error::<Test>::ClaimNotExist
		);

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 100), 
			Error::<Test>::NotClaimOwner
		);
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn test_convert() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::test(Origin::signed(1)));
	})
}