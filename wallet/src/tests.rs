use crate::mock::{Event, *};
use crate::{Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn transfer_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::transfer(Origin::signed(2), ALICE, (CLASS_ID, TOKEN_ID)));
    assert!(OrmlNFT::is_owner(&ALICE, (CLASS_ID, TOKEN_ID)));
  });
}

fn transfer_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

    assert_noop!(
      GamePowerWallet::transfer(Origin::signed(2), ALICE, (CLASS_ID_NOT_EXIST, TOKEN_ID)), 
      Error::<Test>::NoPermission
    );
  });
}

#[test]
fn burn_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID, TOKEN_ID)));
  });
}

#[test]
fn burn_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_noop!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID)), Error::<Test>::NoPermission);
  });
}

#[test]
fn create_listing_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));
  });
}

#[test]
fn create_listing_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_noop!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID), 100), Error::<Test>::NoPermission);
  });
}

#[test]
fn create_claim_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::create_claim(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)));
  });
}

#[test]
fn create_claim_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_noop!(GamePowerWallet::create_claim(Origin::signed(2), BOB, (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
  });
}

#[test]
fn buy_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

    assert_ok!(GamePowerWallet::buy(Origin::signed(2), ALICE, 0));
  });
}

#[test]
fn buy_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

    assert_noop!(GamePowerWallet::buy(Origin::signed(2), BOB, 0), Error::<Test>::AssetNotFound);
  });
}

#[test]
fn locked_asset_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

    assert_ok!(GamePowerWallet::create_claim(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)));

    assert_noop!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
    assert_noop!(GamePowerWallet::transfer(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
  });
}