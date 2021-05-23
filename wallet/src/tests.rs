use crate::mock::{Event, *};
use crate::{Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn transfer_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

	// Make a valid transfer
    assert_ok!(GamePowerWallet::transfer(Origin::signed(2), ALICE, (CLASS_ID, TOKEN_ID)));
    assert!(OrmlNFT::is_owner(&ALICE, (CLASS_ID, TOKEN_ID)));
  });
}

fn transfer_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

	// Try to transfer a token for a class that doesn't exist
    assert_noop!(
      GamePowerWallet::transfer(Origin::signed(2), ALICE, (CLASS_ID_NOT_EXIST, TOKEN_ID)),
      Error::<Test>::NoPermission
    );
  });
}

#[test]
fn burn_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Make a valid burn
    assert_ok!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID, TOKEN_ID)));
  });
}

#[test]
fn burn_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Try to burn a token for a class that doesn't exist
    assert_noop!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID)), Error::<Test>::NoPermission);
  });
}

#[test]
fn create_listing_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Create a valid listing
    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));
  });
}

#[test]
fn create_listing_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Try to create a listing for a class that doesn't exist
    assert_noop!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID), 100), Error::<Test>::NoPermission);
  });
}

#[test]
fn unlisting_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Make a valid listing
    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

	// Properly unlist
	assert_ok!(GamePowerWallet::unlist(Origin::signed(1), LISTING_ID));
  });
}

#[test]
fn unlisting_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Make a valid listing
    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

	// Try to unlist a listing that doesn't belong to the original signer
	assert_noop!(GamePowerWallet::unlist(Origin::signed(2), LISTING_ID), Error::<Test>::AssetNotFound);
  });
}

#[test]
fn create_claim_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Create a valid claim
    assert_ok!(GamePowerWallet::create_claim(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)));
  });
}

#[test]
fn create_claim_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Try to create a claim for a token you don't own
    assert_noop!(GamePowerWallet::create_claim(Origin::signed(2), BOB, (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
  });
}

#[test]
fn buy_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Create a valid listing
    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

	// Make a valid purchase
    assert_ok!(GamePowerWallet::buy(Origin::signed(2), ALICE, LISTING_ID));
  });
}

#[test]
fn buy_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Create a valid listing
    assert_ok!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100));

	// Try to buy a listing not being sold my the original seller
    assert_noop!(GamePowerWallet::buy(Origin::signed(2), BOB, LISTING_ID), Error::<Test>::AssetNotFound);
  });
}

#[test]
fn emote_should_work() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Post a valid emote
    assert_ok!(GamePowerWallet::emote(Origin::signed(2), (CLASS_ID, TOKEN_ID), "fish".as_bytes().to_vec()));
  });
}

#[test]
fn emote_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Post an invalid emote for a class that doesn't exist
    assert_noop!(
		GamePowerWallet::emote(
			Origin::signed(2),
			(CLASS_ID, TOKEN_ID),
			"fasdfasdfaish".as_bytes().to_vec()
		),
		Error::<Test>::InvalidEmote
	);
  });
}

#[test]
fn emote_should_fail_for_invalid_token() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Post an invalid emote for a class that doesn't exist
    assert_noop!(
		GamePowerWallet::emote(
			Origin::signed(2),
			(CLASS_ID_NOT_EXIST, TOKEN_ID),
			"fish".as_bytes().to_vec()
		),
		Error::<Test>::AssetNotFound
	);
  });
}


#[test]
fn locked_asset_should_fail() {
  new_test_ext().execute_with(|| {
	// Create NFT
    assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
    assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

	// Make a valid claim
    assert_ok!(GamePowerWallet::create_claim(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)));

	// All calls that require an unlocked token should give no permission error
    assert_noop!(GamePowerWallet::burn(Origin::signed(1), (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
    assert_noop!(GamePowerWallet::transfer(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)), Error::<Test>::NoPermission);
	assert_noop!(GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100), Error::<Test>::NoPermission);
  });
}
