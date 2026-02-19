// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.


#![allow(missing_docs)]
use std::collections::HashMap;
use reccaster::{record::Record, Reccaster};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let mut record = Record::new("DEV:RECASTER:RUST".to_string(), "ai".to_string());
    record.properties.insert("recordDesc".to_string(), "Rust Recaster".to_string());
    let records: Vec<Record> = vec![record];

    let mut props:  HashMap<String, String> = HashMap::new();
    props.insert("ENGINEER".into(), "Rust Recaster".into());
    props.insert("HOSTNAME".into(), "Example-Host-Machine".into());

    let mut caster = Reccaster::new(records, Some(props)).await;
    caster.run().await;
}
