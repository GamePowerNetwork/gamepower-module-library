use crate::mock::{Event, *};
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

        // Make a valid transfer
        assert_ok!(GamePowerWallet::transfer(
            Origin::signed(2),
            ALICE,
            (CLASS_ID, TOKEN_ID)
        ));
        assert!(OrmlNFT::is_owner(&ALICE, (CLASS_ID, TOKEN_ID)));
    });
}

#[test]
fn transfer_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&BOB, CLASS_ID, vec![1], ()));

        // Try to transfer a token for a class that doesn't exist
        assert_noop!(
            GamePowerWallet::transfer(
                Origin::signed(2),
                ALICE,
                (CLASS_ID_NOT_EXIST, TOKEN_ID_NOT_EXIST)
            ),
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
        assert_ok!(GamePowerWallet::burn(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID)
        ));
    });
}

#[test]
fn burn_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Try to burn a token for a class that doesn't exist
        assert_noop!(
            GamePowerWallet::burn(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID)),
            Error::<Test>::NoPermission
        );
    });
}

#[test]
fn create_listing_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Create a valid listing
        assert_ok!(GamePowerWallet::list(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID),
            100
        ));

        assert_eq!(
            GamePowerWallet::next_listing_id(),
            1,
            "The next listing id is incorrect"
        );
        assert_eq!(
            GamePowerWallet::listing_count(),
            1,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            1,
            "Listing not added to all"
        );
        assert_eq!(
            GamePowerWallet::listings_by_owner(1),
            Some(vec![0]),
            "Listing by owner not added"
        );
        assert_eq!(
            GamePowerWallet::listings(0).is_some(),
            true,
            "Listing not added"
        );
    });
}

#[test]
fn create_listing_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Try to create a listing for a class that doesn't exist
        assert_noop!(
            GamePowerWallet::list(Origin::signed(1), (CLASS_ID_NOT_EXIST, TOKEN_ID), 100),
            Error::<Test>::NoPermission
        );

        assert_eq!(
            GamePowerWallet::listing_count(),
            0,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            0,
            "The number of all listings is incorrect"
        );
    });
}

#[test]
fn unlisting_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Make a valid listing
        assert_ok!(GamePowerWallet::list(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID),
            100
        ));

        // Properly unlist
        assert_ok!(GamePowerWallet::unlist(Origin::signed(1), LISTING_ID));
        assert_eq!(
            GamePowerWallet::listing_count(),
            0,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            0,
            "Listing not removed from all"
        );
        assert_eq!(
            GamePowerWallet::listings_by_owner(1),
            Some(vec![]),
            "Listing by owner not removed"
        );
        assert_eq!(GamePowerWallet::listings(0), None, "Listing not removed");
    });
}

#[test]
fn unlisting_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Make a valid listing
        assert_ok!(GamePowerWallet::list(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID),
            100
        ));

        // Try to unlist a listing that doesn't belong to the original signer
        assert_noop!(
            GamePowerWallet::unlist(Origin::signed(2), LISTING_ID),
            Error::<Test>::NoPermission
        );

        assert_eq!(
            GamePowerWallet::listing_count(),
            1,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            1,
            "The number of all listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::listings_by_owner(1),
            Some(vec![0]),
            "Listing by owner should have a value"
        );
        assert_eq!(
            GamePowerWallet::listings(0).is_some(),
            true,
            "Listing should not be removed"
        );
    });
}

#[test]
fn create_claim_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Create a valid claim
        assert_ok!(GamePowerWallet::create_claim(
            Origin::signed(1),
            BOB,
            (CLASS_ID, TOKEN_ID)
        ));

        assert_eq!(
            GamePowerWallet::next_claim_id(),
            1,
            "The next claim id is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_claims().len(),
            1,
            "Claim not added to all"
        );
        assert_eq!(
            GamePowerWallet::open_claims(BOB, 0).is_some(),
            true,
            "Claim not added"
        );
    });
}

#[test]
fn create_claim_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Try to create a claim for a token you don't own
        assert_noop!(
            GamePowerWallet::create_claim(Origin::signed(2), BOB, (CLASS_ID, TOKEN_ID)),
            Error::<Test>::NoPermission
        );

        assert_eq!(
            GamePowerWallet::next_claim_id(),
            0,
            "The next claim id is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_claims().len(),
            0,
            "Claim not added to all"
        );
        assert_eq!(
            GamePowerWallet::open_claims(BOB, 0).is_some(),
            false,
            "Claim should not be added"
        );
    });
}

#[test]
fn buy_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // There should be no listing
        assert_eq!(GamePowerWallet::listings(0), None, "Listing was not empty");

        // Create a valid listing
        assert_ok!(GamePowerWallet::list(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID),
            100
        ));

        assert_eq!(
            GamePowerWallet::all_listings().len(),
            1,
            "Listing not created"
        );
        // Make a valid purchase
        assert_ok!(GamePowerWallet::buy(Origin::signed(2), LISTING_ID));
        assert_eq!(
            GamePowerWallet::listing_count(),
            0,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            0,
            "Listing not removed from all!"
        );
        assert_eq!(
            GamePowerWallet::listings_by_owner(1),
            Some(vec![]),
            "Listing by owner not removed"
        );
        assert_eq!(GamePowerWallet::listings(0), None, "Listing not removed");

        // Check Balances
        assert_eq!(Balances::free_balance(ALICE), 1000000 + 100);
        assert_eq!(Balances::free_balance(BOB), 1000000 - 100);
    });
}

#[test]
fn buy_should_fail() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Create a valid listing
        assert_ok!(GamePowerWallet::list(
            Origin::signed(1),
            (CLASS_ID, TOKEN_ID),
            100
        ));

        // Try to buy a listing not being sold
        assert_noop!(
            GamePowerWallet::buy(Origin::signed(2), LISTING_ID_NOT_EXIST),
            Error::<Test>::ListingNotFound
        );

        assert_eq!(
            GamePowerWallet::listing_count(),
            1,
            "The total number of listings is incorrect"
        );
        assert_eq!(
            GamePowerWallet::all_listings().len(),
            1,
            "Listing should not be removed from all!"
        );
        assert_eq!(
            GamePowerWallet::listings_by_owner(ALICE),
            Some(vec![0]),
            "Listing by owner should not be removed"
        );
        assert_eq!(
            GamePowerWallet::listings(0).is_some(),
            true,
            "Listing should not be removed"
        );

        // Check Balances
        assert_eq!(Balances::free_balance(ALICE), 1000000);
        assert_eq!(Balances::free_balance(BOB), 1000000);
    });
}

#[test]
fn emote_should_work() {
    new_test_ext().execute_with(|| {
        // Create NFT
        assert_ok!(OrmlNFT::create_class(&ALICE, vec![1], ()));
        assert_ok!(OrmlNFT::mint(&ALICE, CLASS_ID, vec![1], ()));

        // Post a valid emote
        assert_ok!(GamePowerWallet::emote(
            Origin::signed(2),
            (CLASS_ID, TOKEN_ID),
            "fish".as_bytes().to_vec()
        ));

        assert_eq!(
            GamePowerWallet::emotes((CLASS_ID, TOKEN_ID), BOB).len(),
            1,
            "Emote should be added"
        );
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

        assert_eq!(
            GamePowerWallet::emotes((CLASS_ID, TOKEN_ID), BOB).len(),
            0,
            "Emote should not be added"
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
        assert_ok!(GamePowerWallet::create_claim(
            Origin::signed(1),
            BOB,
            (CLASS_ID, TOKEN_ID)
        ));

        // All calls that require an unlocked token should give no permission error
        assert_noop!(
            GamePowerWallet::burn(Origin::signed(1), (CLASS_ID, TOKEN_ID)),
            Error::<Test>::NoPermission
        );
        assert_noop!(
            GamePowerWallet::transfer(Origin::signed(1), BOB, (CLASS_ID, TOKEN_ID)),
            Error::<Test>::NoPermission
        );
        assert_noop!(
            GamePowerWallet::list(Origin::signed(1), (CLASS_ID, TOKEN_ID), 100),
            Error::<Test>::NoPermission
        );
    });
}
