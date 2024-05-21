// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 45 --simulator --accounts C

//# run-graphql
{ # Initial query yields only the validator's stake
  objects(filter: { type: "0x3::staking_pool::StakedHaneul" }) {
    edges {
      cursor
      node {
        asMoveObject {
          asStakedHaneul {
            principal
          }
        }
      }
    }
  }

  address(address: "@{C}") {
    stakedHaneuls {
      edges {
        cursor
        node {
          principal
        }
      }
    }
  }
}

//# programmable --sender C --inputs 10000000000 @C
//> SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# run 0x3::haneul_system::request_add_stake --args object(0x5) object(2,0) @validator_0 --sender C

//# create-checkpoint

//# advance-epoch

//# run-graphql
{ # This query should pick up the recently Staked HANEUL as well.
  objects(filter: { type: "0x3::staking_pool::StakedHaneul" }) {
    edges {
      cursor
      node {
        asMoveObject {
          asStakedHaneul {
            principal
            poolId
          }
        }
      }
    }
  }

  address(address: "@{C}") {
    stakedHaneuls {
      edges {
        cursor
        node {
          principal
        }
      }
    }
  }
}
