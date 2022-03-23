// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Module providing debug functionality.
module Std::Debug {
    native public fun print<T>(x: &T);

    native public fun print_stack_trace();
}
