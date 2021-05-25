# GamePower Module Library
An open library for integrating substrate runtimes with the [GamePower Network](https://www.gamepower.network)


## Project Overview :page_facing_up:

The `GamePower Wallet` is a multi-chain non-custodial mobile wallet which will allow users to claim, store and manage an unlimited number of tokenized assets from any Substrate based blockchain. This project will consist of 3 major parts: the mobile wallet for users, the javascript sdk for front-end developers to create NFTs and a substrate pallet for connecting chains with the wallet.

<u>**Users:**</u> By scanning QR codes, users will be able to claim NFTs into their wallet. These NFTs can be consumed (burned), listed for sale or sent to others. Users will also have the ability to mint NFTs from within the mobile app. The mobile app will be downloadable from Apple's App Store and Google's Play Store.

<u>**Substrate developers:**</u> By adding the `gamepower-wallet` pallet to their substrate runtime, developers will allow the GamePower Collectibles Wallet to connect to their blockchain. The pallet will expose an interface for the wallet to manage NFTs.

<u>**Front-end developers:**</u> By using the `gamepower-wallet-javascript-sdk`, javascript developers can create a UI for creating new NFTs. This javascript sdk will be able to connect to any substrate node via Polkadot.js.


### Overview

The reason we are creating the NFT Collectibles Wallet is to allow users of GamePower Network (https://www.gamepower.network) to claim NFTs from games published on the platform. We could have made the wallet closed sourced such as other projects (Enjin Wallet), but we decided since we are the new kids on the block, it is better for us to contribute to the Substrate/Polkadot/Kusama community. That is what excites us so much about this project.

Our team is very passionate about gaming and NFTs. We believe the use case for NFTs in gaming is one of the most valuable in crypto right now. The problem we see with NFTs is that explaining NFTs to the general consumer and giving them a streamlined and friendly place to use those NFTs is lacking. We want to solve this with the NFT Collectibles Wallet.

By allowing Substrate developers to integrate our module into their runtime, they can also take advantage of this wallet and offers users of their blockchain the same streamlined experience we will offer users for the GamePower Network.

## Project Details
---
## **Mobile Wallet Details:**

The mobile wallet will be built using `React Native`. We feel this will allow us to use a coding language (javascript) we've used for years and build performant mobile applications. Using React Native also allows us to code once and deploy on multiple mobile platforms.

Mobile Stack:
- React Native
- Polkadot.js
- react-qr-scanner

A mockup of our mobile wallet UI. This mockup outlines the wallet creation, QR scanning and collectibles viewer.
![img](https://github.com/GamePowerNetwork/nft-collectibles-wallet/raw/open-grant/images/Mobile_App_Wireframe.png)

## **Substrate Pallet Details:**

The `nft-wallet-pallet` will use ORML (open runtime modules library: https://github.com/open-web3-stack/open-runtime-module-library) which will provide us with some underlying NFT code. The pallet will also talk to the balances pallet to handle any minting and consuming which is needed since each NFT is minted with a type of currency native to the blockchain it is on.

Substrate Stack:
- Substrate
- ORML

These methods will serve as an interface for the NFT Wallet to communicate with any substrate runtime. `nft-wallet-pallet` expects ORML's nft pallet to be a part of the runtime since it will be used to handle all NFT related functions.

We will expose TRANSFER, BURN and CLAIM callbacks so that the runtime can do any domain specific logic when sending or burning an NFT.


## **Javascript SDK Details:**

The front-end UI will be built using React + Polkadot.js. This will be a straight-forward and clean UI to allow the creation and management of NFTs. This UI is not a front-end for consumers but for developers to create NFTs. The underlying SDK for the front-end can be used to create any type of custom NFT management UI.

Web Stack:
- React
- Polkadot.js

Mockup of the admin frontend.
![img](https://github.com/GamePowerNetwork/nft-collectibles-wallet/raw/open-grant/images/Admin.png)


### Ecosystem Fit

The NFT Collectibles Wallet provides the ecosystem with a streamlined and standard way to create, manage and exchange NFTs. By allowing the wallet to connect to any substrate based chain, users can freely move around the ecosystem without downloading multiple wallets for each chain, while still having a wallet that focuses specifically on collectibles.
