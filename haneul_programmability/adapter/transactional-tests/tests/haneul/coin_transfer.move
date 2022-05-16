// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Test basic coin transfer

//# init --accounts A B

//# view-object 100

//# run Haneul::Coin::transfer_ --type-args Haneul::HANEUL::HANEUL --args object(100) 10 @B

//# view-object 100

//# view-object 105
