// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::tx_context_tests {
    use haneul::id;
    use haneul::tx_context;

    #[test]
    fun test_id_generation() {
        let ctx = tx_context::dummy();
        assert!(tx_context::get_ids_created(&ctx) == 0, 0);

        let id1 = tx_context::new_id(&mut ctx);
        let id2 = tx_context::new_id(&mut ctx);

        // new_id should always produce fresh ID's
        assert!(&id1 != &id2, 1);
        assert!(tx_context::get_ids_created(&ctx) == 2, 2);
        id::delete(id1);
        id::delete(id2);
    }

}
