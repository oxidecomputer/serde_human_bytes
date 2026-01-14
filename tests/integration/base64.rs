// Copyright (c) The serde_human_bytes Contributors
// SPDX-License-Identifier: Apache-2.0

use hex_literal::hex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct MyStruct {
    #[serde(with = "serde_human_bytes::base64_vec")]
    data: Vec<u8>,
}

static FIXTURE: &[u8] = &hex!("0123456789abcdef0123456789abcdef");

// Base64 encoding of the fixture bytes.
static AS_JSON: &str = r#"{"data":"ASNFZ4mrze8BI0VniavN7w=="}"#;

// CBOR: map with one key "data" and a byte string value.
static AS_CBOR: [u8; 23] = hex!("a1646461746150 0123456789abcdef0123456789abcdef");

fn fixture() -> MyStruct {
    MyStruct {
        data: FIXTURE.to_vec(),
    }
}

#[test]
fn base64_serialize() {
    let fixture = fixture();

    assert_eq!(
        serde_json::to_string(&fixture).expect("serializing as JSON succeeded"),
        AS_JSON,
        "JSON matched",
    );

    let mut cbor_actual: Vec<u8> = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor_actual).expect("writing to vec<u8> succeeded");

    assert_eq!(cbor_actual, AS_CBOR, "CBOR matched");
}

#[test]
fn base64_deserialize() {
    let fixture = fixture();

    let json_actual: MyStruct =
        serde_json::from_str(AS_JSON).expect("deserializing from JSON succeeded");
    assert_eq!(fixture, json_actual, "deserializing from JSON matched");

    let cbor_actual: MyStruct =
        ciborium::de::from_reader(&AS_CBOR[..]).expect("deserializing from CBOR succeeded");
    assert_eq!(fixture, cbor_actual, "deserializing from CBOR succeeded");
}
