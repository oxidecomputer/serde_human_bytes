// Copyright (c) The serde_human_bytes Contributors
// SPDX-License-Identifier: Apache-2.0

use hex_literal::hex;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct MyStruct {
    #[serde(with = "serde_human_bytes::hex")]
    x: [u8; 16],
}

static FIXTURE: MyStruct = MyStruct {
    x: hex!("0123456789abcdef0123456789abcdef"),
};

static AS_JSON: &str = r#"{"x":"0123456789abcdef0123456789abcdef"}"#;
static AS_CBOR: [u8; 20] = hex!("a16178500123456789abcdef0123456789abcdef");

#[test]
fn hex_serialize() {
    assert_eq!(
        serde_json::to_string(&FIXTURE).expect("serializing as JSON succeeded"),
        AS_JSON,
        "JSON matched",
    );
    let mut cbor_actual: Vec<u8> = Vec::new();
    ciborium::ser::into_writer(&FIXTURE, &mut cbor_actual).expect("writing to vec<u8> succeeded");

    assert_eq!(cbor_actual, AS_CBOR, "CBOR matched");
}

#[test]
fn hex_deserialize() {
    let json_actual: MyStruct =
        serde_json::from_str(AS_JSON).expect("deserializing from JSON succeeded");
    assert_eq!(FIXTURE, json_actual, "deserializing from JSON matched");

    let cbor_actual: MyStruct =
        ciborium::de::from_reader(&AS_CBOR[..]).expect("deserializing from CBOR succeeded");
    assert_eq!(FIXTURE, cbor_actual, "deserializing from CBOR succeeded");
}
