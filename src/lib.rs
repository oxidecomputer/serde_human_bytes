// Copyright (c) The serde_human_bytes Contributors
// SPDX-License-Identifier: Apache-2.0

#![warn(missing_docs)]

//! Serialize `[u8; N]` as bytes or as human-readable strings, depending on the
//! format.
//!
//! For example, serialize as hex if serializing to JSON, or as an efficient
//! byte sequence with CBOR.

pub mod hex_array;
