// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --accounts A --simulator

//# run-graphql
{
  epoch(epochId: 0) {
    epochId
    validatorSet {
      activeValidators {
        address
        balance(coinType: "0x2::haneul::HANEUL") {
          totalBalance
        }
        balances {
          __typename
        }
        # todo (ewall) populate defaultHaneulnsName
        defaultHaneulnsName
        multiGetBalances(keys: ["0x2::haneul::HANEUL"]) {
          totalBalance
        }
        objects {
          __typename
        }
        credentials { ...VC }
        # todo (ewall) populate nextEpochCredentials
        nextEpochCredentials { ...VC }
        name
        # todo (ewall) populate description
        description
        # todo (ewall) populate imageUrl
        imageUrl
        # todo (ewall) populate projectUrl
        projectUrl
        stakingPoolId
        exchangeRatesSize
        stakingPoolActivationEpoch
        stakingPoolHaneulBalance
        # todo (ewall) populate rewardsPool
        rewardsPool
        poolTokenBalance
        # todo (ewall) populate pendingStake
        pendingStake
        # todo (ewall) populate pendingTotalHaneulWithdraw
        pendingTotalHaneulWithdraw
        # todo (ewall) populate pendingPoolTokenWithdraw
        pendingPoolTokenWithdraw
        votingPower
        gasPrice
        commissionRate
        nextEpochStake
        nextEpochGasPrice
        nextEpochCommissionRate
      }
    }
  }
}

fragment VC on ValidatorCredentials {
  protocolPubKey
  networkPubKey
  workerPubKey
  proofOfPossession
  netAddress
  p2PAddress
  primaryAddress
  workerAddress
}
