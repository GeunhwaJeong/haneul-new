// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Test basic coin transfer

//# init --accounts A B C

//# view-object 100

//# run Haneul::Coin::split_and_transfer --type-args Haneul::HANEUL::HANEUL --args object(100) 10 @B --sender A

//# view-object 100

//# view-object 106

//# run Haneul::Coin::transfer --type-args Haneul::HANEUL::HANEUL --args object(100) @C --sender B

//# view-object 100

//# view-object 107
