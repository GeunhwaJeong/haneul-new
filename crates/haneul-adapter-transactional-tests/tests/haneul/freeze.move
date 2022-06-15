// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// test that freezing prevents transfers/mutations

//# init --accounts A

//# run haneul::object_basics::create --args 10 @A

//# run haneul::object_basics::freeze_object --args object(104)

//# run haneul::object_basics::transfer --args object(104) @A

//# run haneul::object_basics::set_value --args object(104) 1
