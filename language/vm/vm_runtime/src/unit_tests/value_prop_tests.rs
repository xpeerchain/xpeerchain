// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::value::Value;
use proptest::prelude::*;

proptest! {
    #[test]
    fn flat_struct_test(value in Value::struct_strategy()) {
        let struct_def = value.to_struct_def_FOR_TESTING();
        let blob = value.simple_serialize().expect("must serialize");
        let value1 = Value::simple_deserialize(&blob, struct_def).expect("must deserialize");
        assert_eq!(value, value1);
    }
}
