// Copyright (c) The serde_human_bytes Contributors
// SPDX-License-Identifier: Apache-2.0

#![warn(missing_docs)]
#![no_std]

//! Serialize byte arrays and vectors as bytes or as human-readable strings,
//! depending on the format.
//!
//! For example, serialize as hex if serializing to JSON, or as an efficient
//! byte sequence with CBOR.

#[cfg(feature = "alloc")]
pub mod base64_vec;
pub mod hex_array;
