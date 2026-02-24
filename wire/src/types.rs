// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use std::net::Ipv4Addr;

/// AddRecord message type discriminant.
pub enum AddRecordType {
    /// A regular PV record.
    Record = 0,
    /// An alias for an existing record.
    Alias = 1,
}

/// UDP Announcement message structure.
#[derive(Debug)]
pub struct Announcement {
    /// Magic ID identifying this as a RecSync announcement.
    pub id: u16,
    /// IPv4 address of the announcing server.
    pub server_addr: Ipv4Addr,
    /// TCP port the server is listening on.
    pub server_port: u16,
    /// Server-generated session key.
    pub server_key: u32,
}

/// Message type identifiers used in the wire protocol header.
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum MessageID {
    /// Server greeting sent after a client connects.
    ServerGreet = 0x8001,
    /// Client greeting sent in response to a server greeting.
    ClientGreet = 0x0001,
    /// Keepalive ping sent by the server.
    Ping = 0x8002,
    /// Keepalive pong sent by the client in response to a ping.
    Pong = 0x0002,
    /// Adds a PV record to the server's database.
    AddRecord = 0x0003,
    /// Removes a PV record from the server's database.
    DelRecord = 0x0004,
    /// Signals that the client has finished uploading records.
    UploadDone = 0x0005,
    /// Attaches metadata to a record.
    AddInfo = 0x0006,
}

impl From<u16> for MessageID {
    fn from(value: u16) -> Self {
        match value {
            0x8001 => MessageID::ServerGreet,
            0x0001 => MessageID::ClientGreet,
            0x8002 => MessageID::Ping,
            0x0002 => MessageID::Pong,
            0x0003 => MessageID::AddRecord,
            0x0004 => MessageID::DelRecord,
            0x0005 => MessageID::UploadDone,
            0x0006 => MessageID::AddInfo,
            _ => unimplemented!("Unknown Message ID"),
        }
    }
}

impl From<MessageID> for u16 {
    fn from(msg_id: MessageID) -> u16 {
        match msg_id {
            MessageID::ServerGreet => 0x8001,
            MessageID::ClientGreet => 0x0001,
            MessageID::Ping => 0x8002,
            MessageID::Pong => 0x0002,
            MessageID::AddRecord => 0x0003,
            MessageID::DelRecord => 0x0004,
            MessageID::UploadDone => 0x0005,
            MessageID::AddInfo => 0x0006,
        }
    }
}

// Define all the message structs and enums here

/// Server greeting payload (no additional fields).
#[derive(Debug, Clone, PartialEq)]
pub struct ServerGreet;

/// Keepalive ping payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    /// Random nonce that the client must echo back in the Pong.
    pub nonce: u32,
}

/// Client greeting payload.
#[derive(Debug, Clone, PartialEq)]
pub struct ClientGreet {
    /// Server key received in the UDP announcement.
    pub serv_key: u32,
}

/// Keepalive pong payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
    /// Nonce copied from the corresponding Ping.
    pub nonce: u32,
}

/// Payload for registering a PV record or alias on the server.
#[derive(Debug, Clone, PartialEq)]
pub struct AddRecord {
    /// Record identifier assigned by the client.
    pub recid: u32,
    /// Record type discriminant (`AddRecordType::Record` or `AddRecordType::Alias`).
    pub atype: u8,
    /// Length of the record type string in bytes.
    pub rtlen: u8,
    /// Length of the record name string in bytes.
    pub rnlen: u16,
    /// Record type string (e.g. `"ai"`).
    pub rtype: String,
    /// Record name or alias string.
    pub rname: String,
}

/// Payload for removing a previously registered PV record.
#[derive(Debug, Clone, PartialEq)]
pub struct DelRecord {
    /// Record identifier to remove.
    pub recid: u32,
}

/// Payload signalling that the client has finished uploading records.
#[derive(Debug, Clone, PartialEq)]
pub struct UploadDone;

/// Payload for attaching a key-value metadata entry to a record.
#[derive(Debug, Clone, PartialEq)]
pub struct AddInfo {
    /// Record identifier this info belongs to (0 for client-level info).
    pub recid: u32,
    /// Length of the key string in bytes.
    pub keylen: u8,
    /// Length of the value string in bytes.
    pub valen: u16,
    /// Metadata key.
    pub key: String,
    /// Metadata value.
    pub value: String,
}

/// All messages that can be exchanged over the wire.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Server greeting.
    ServerGreet(ServerGreet),
    /// Keepalive ping from the server.
    Ping(Ping),
    /// Client greeting.
    ClientGreet(ClientGreet),
    /// Keepalive pong from the client.
    Pong(Pong),
    /// Add a PV record or alias.
    AddRecord(AddRecord),
    /// Remove a PV record.
    DelRecord(DelRecord),
    /// Signal end of record upload.
    UploadDone(UploadDone),
    /// Attach metadata to a record.
    AddInfo(AddInfo),
}
