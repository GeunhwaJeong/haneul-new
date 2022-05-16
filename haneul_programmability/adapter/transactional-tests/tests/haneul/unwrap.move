// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Exercise test functions that wrap and object and subsequently unwrap it
// Ensure that the object's version is consistent

//# init --accounts A

//# run Haneul::ObjectBasics::create --args 10 @A

//# view-object 104

//# run Haneul::ObjectBasics::wrap --args object(104) --sender A

//# run Haneul::ObjectBasics::unwrap --args object(106) --sender A

//# view-object 104
