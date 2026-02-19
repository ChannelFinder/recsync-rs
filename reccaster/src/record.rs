// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use std::collections::HashMap;

/// Represents a single PV (Process Variable) record to be registered with the server.
#[derive(Debug, Clone)]
pub struct Record {
    /// The PV name (e.g. `"DEV:AI:1"`).
    pub name: String,
    /// The EPICS record type (e.g. `"ai"`, `"bo"`).
    pub r#type: String,
    /// An optional alias name for this record.
    pub alias: Option<String>,
    /// Arbitrary key-value metadata attached to this record.
    pub properties: HashMap<String, String>,
}

impl Record {
    /// Create a new record with the given name and type, no alias, and empty properties.
    pub fn new(name: String, r#type: String) -> Record {
        let map: HashMap<String, String> = HashMap::new();
        Record { name, r#type, alias: None, properties: map}
    }
}
