// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

// test cases for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!{
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        }
    })
}

#[test]
fn create_claim_failed_when_claim_too_long() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_noop! {
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        }
    })
}

#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim2 = vec![0, 1, 2, 3];
        assert_noop! {
            PoeModule::revoke_claim(Origin::signed(1), claim2.clone()),
            Error::<Test>::ClaimNotExist
        }
    })
}

#[test]
fn revoke_claim_failed_when_wrong_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop! {
            PoeModule::revoke_claim(Origin::signed(3), claim.clone()),
            Error::<Test>::NotClaimOwner
        }
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert!(Proofs::<Test>::contains_key(&claim));
        assert_eq!(Proofs::<Test>::get(&claim).0, 2);
    })
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!{
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        }
    })
}

#[test]
fn transfer_claim_failed_when_wrong_ower() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop! {
            PoeModule::transfer_claim(Origin::signed(3), claim.clone(), 2),
            Error::<Test>::NotClaimOwner
        }
    })
}