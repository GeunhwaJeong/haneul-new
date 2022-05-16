// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// test that freezing prevents transfers/mutations

//# init --accounts A

//# run Haneul::ObjectBasics::create --args 10 @A

//# run Haneul::ObjectBasics::freeze_object --args object(104)

//# run Haneul::ObjectBasics::transfer --args object(104) @A

//# run Haneul::ObjectBasics::set_value --args object(104) 1
