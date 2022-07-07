---
title: Using the Haneul Wallet Browser Extension
---

Welcome to the [Haneul Wallet Browser Chrome Extension](https://chrome.google.com/webstore/detail/haneul-wallet/albddfdbohgeonpapellnjadnddglhgn?hl=en&authuser=0). The Haneul Wallet Browser Extension acts as your portal to the Web3 world. Follow this guide to install and use the extension.

## Purpose

Initially, the Haneul Wallet Browser Extension is aimed at Haneul developers for testing purposes. As such, the tokens are of no value (just like the rest of [DevNet](../explore/devnet.md)) and will disappear each time we reset the network. In time, the Haneul Wallet Browser Extension will be production ready for real tokens.

This browser extension is a pared-down version of the [Haneul CLI client)](../build/cli-client.md) that provides greater ease of use for the most commonly used features. If you need more advanced features, such as merging/splitting coins and making arbitrary [Move](../build/move.md) calls, instead use the [Haneul CLI client](../build/cli-client.md).

## Features

The Haneul Wallet Browser Extension offers these features:

* Create, import, and persistently store the backup recovery passphrases (mnemonics) and the derived private key
* Create NFTs
* Transfer coins
* See owned fungible tokens and NFTs
* Display recent transactions
* Auto split/merge coins if the address does not have a Coin object with the exact transfer amount
* Go directly to the successful/failed transaction in the [Haneul Explorer](https://explorer.devnet.haneul.io/)
* A demonstration [NFT dApp](https://github.com/GeunhwaJeong/haneul/tree/main/wallet/examples/demo-nft-dapp) available [in the Cloud](http://haneul-wallet-demo.s3-website-us-east-1.amazonaws.com/)

See [Demos](#demos) for depictions of these features in play and [Use](#use) to find these features in navigation.

## Demos

The following animated GIFs walk you through some of the most common workflows in the Haneul Wallet Browser Extension.

### Set up Wallet

Install and configure the Haneul Wallet Browser Extension (covered in detail starting with [Install](#install)):

![Set up Wallet](../../static/onboarding.gif "Set up Wallet")
*Set up the Haneul Wallet Browser Extension*

### Create NFT

From a demo decentralized site, such as our demonstration [NFT dApp](https://github.com/GeunhwaJeong/haneul/tree/main/wallet/examples/demo-nft-dapp) available [in the Cloud](http://haneul-wallet-demo.s3-website-us-east-1.amazonaws.com/), you can connect to your wallet and create a custom NFT:

![Create NFT](../../static/create_NFT.gif "Create NFT")
*Create an NFT in Haneul Wallet by connecting to an external site*

### Transfer NFT

Transfer your NFT to another address using the Haneul Wallet Browser Extension:

![Transfer NFT](../../static/nft_transfer.gif "Transfer NFT")
*Transfer your NFT to another address*

### Transfer token

Transfer your token to another address on the Haneul network using the Haneul Wallet Browser Extension:

![Transfer token](../../static/nft_transfer.gif "Transfer token")
*Transfer tokens to another address*

### View transaction history

View your recent transactions and visit [Haneul Explorer](https://explorer.devnet.haneul.io/), where you can see more details about the corresponding transaction:

![Transaction history and settings](../../static/txn_history_and_settings.gif "Transaction history and settings")
*Under the *Settings* tab, view your account on Haneul Explorer*

## Install

To install the Haneul Wallet Browser Extension:
1. Visit its [link in the Chrome Webstore](https://chrome.google.com/webstore/detail/haneul-wallet/albddfdbohgeonpapellnjadnddglhgn?hl=en&authuser=0).
1. Click **Install**.
1. Optionally, [pin the extension](https://www.howtogeek.com/683099/how-to-pin-and-unpin-extensions-from-the-chrome-toolbar/) to add it to your toolbar for easy access.

## Start up

To begin using the Haneul Wallet Browser Extension:
1. Open the extension and click **Get Started**:
   ![Start up Haneul Wallet](../../static/Haneul-wallet-get-started.png "Start up Haneul Wallet")
   *Start up Haneul Wallet Browser Extension*
1. Click **Create new wallet**:
   ![Create new Haneul Wallet](../../static/Haneul-wallet-new-account.png "Create new Haneul Wallet")
   *Create new wallet with Haneul Wallet Browser Extension*
1. Accept the terms of service and click **Create**:
   ![Accept the terms of service for Haneul Wallet](../../static/Haneul-wallet-ToS.png "Accept ToS")
   *Accept the terms of service for Haneul Wallet Browser Extension*
1. View and capture the distinct mnemonic for the new wallet.
1. Click **Done**.

## Configure

In the Wallet home page, you will see the message _No Tokens Found_:
![No tokens found](../../static/Haneul-wallet-no-tokens.png "[No tokens found")
*Time to populate your wallet*

To finish setting up the Haneul Wallet Browser Extension for testing:
1. From the _Active Account_ in your wallet, copy your **address**:
   ![Copy address from Haneul Wallet](../../static/Haneul-wallet-copy-address.png "Copy address")
   *Copy your address from the Haneul Wallet Browser Extension*
1. Join [Discord](https://discord.gg/haneul) If you haven’t already.
1. Request tokens in the [#devnet-faucet](https://discord.com/channels/916379725201563759/971488439931392130)
   channel per the [HANEUL tokens](../build/install.md#haneul-tokens) install documentation.
1. Optionally, confirm the transaction in Haneul Explorer:
   ![See transfer in Haneul Explorer](../../static/Haneul-explorer-token-transfer.png "See Haneul Explorer")
   *See transfer in Haneul Explorer*

## Use

The Haneul Wallet Browser Extension lets you:

* See your account balance by clicking the **Tokens ($)** icon:
   ![See your account balance](../../static/tokens.png "See tokens")
   *See your account balance in the Haneul Wallet Browser Extension*
* Send coins by clicking **Send** in the _Tokens_ tab:
   ![Send tokens](../../static/token-transfer.png "Send tokens")
   *Send tokens with the Haneul Wallet Browser Extension*
* Transfer NFTs by clicking **Send** on the _NFT_ tab:
   ![Transfer NFTs](../../static/NFT-transfer.png "Send tokens")
   *Send NFTs with the Haneul Wallet Browser Extension*
* View _recent transactions_ by clicking the **Arrow** icon at the top:
   ![View recent transactions](../../static/txn-history.png "View recent transactions")
   *View recent transactions in the Haneul Wallet Browser Extension*
* Sign transactions through a framework connecting Haneul wallet to other DApps:
   ![Sign transactions](../../static/txn-signing.png "View recent transactions")
   *Sign transactions in the Haneul Wallet Browser Extension*
* From the **Settings (gear)** menu, you may:
    * View your account on the Haneul Explorer
    * Mint Demo NFTs
    * See the Haneul terms of service
    * Log out of the Wallet
   ![Access settings](../../static/settings.png "Access wallet settings")
   *Access settings for the Haneul Wallet Browser Extension*
* Go to the [Haneul Explorer](https://explorer.devnet.haneul.io/) view of the current transaction by clicking the external link icon at the bottom right.

## Contribute

If you want to experiment with and contribute to the Haneul Wallet Browser Extension, you can find its source and README at:
https://github.com/GeunhwaJeong/haneul/tree/main/wallet 
