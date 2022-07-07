// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::event {

    /// Add `t` to the event log of this transaction
    // TODO(https://github.com/GeunhwaJeong/haneul/issues/19):
    // restrict to internal types once we can express this in the ability system
    public native fun emit<T: copy + drop>(event: T);

    // Cost calibration functions
    #[test_only]
    public fun calibrate_emit<T: copy + drop>(obj: T) {
        emit(obj)
    }
    #[test_only]
    public fun calibrate_emit_nop<T: copy + drop>(_obj: T) {
    }
}
