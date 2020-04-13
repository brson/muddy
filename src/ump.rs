//! Universal MIDI Packet format

#![allow(unused)]

use anyhow::{Result, Error, bail};
use std::convert::{TryInto, TryFrom};
use num_enum::TryFromPrimitive;


const NIBBLE_MASK: u32 = 0xF;


//+ M2-104-UM 2.1 UMP Basic Packet and Message Format

pub struct Packet {
    /// Native-endian words
    data: [u32; 4],
}


//+ M2-104-UM 2.1.2 UMP Format Universal Fields

impl Packet {
    fn message_type_bits(&self) -> u32 {
        self.data[0] >> 28
    }

    fn group_bits(&self) -> u32 {
        self.data[0] >> 24 & NIBBLE_MASK
    }
}

impl Packet {
    pub fn message_type(&self) -> Result<MessageType> {
        let msg_type = self.message_type_bits();
        let msg_type: u8 = msg_type.try_into()?;
        let msg_type: MessageType = msg_type.try_into()?;
        Ok(msg_type)
    }

    pub fn group(&self) -> Result<Group> {
        let group = self.group_bits();
        let group: u8 = group.try_into()?;
        let group: Group = group.try_into()?;
        Ok(group)
    }
}

/// There are 16 groups of 16 channels
#[derive(Copy, Clone)]
pub struct Group(u8);

impl TryFrom<u8> for Group {
    type Error = Error;

    fn try_from(v: u8) -> Result<Group> {
        if v <= 16 {
            Ok(Group(v))
        } else {
            bail!("invalid group: {}", v);
        }
    }
}

impl Into<u8> for Group {
    fn into(self) -> u8 { self.0 }
}


//+ M2-104-UM 2.1.4 Message Type (MT) Allocation

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum MessageType {
    Utility = 0x0,
    SystemRealTimeAndCommon = 0x1,
    Midi1ChannelVoice = 0x2,
    DataInclSystemExclusive = 0x3,
    Midi2ChannelVoice = 0x4,
    Data = 0x5,
    Reserved6 = 0x6,
    Reserved7 = 0x7,
    Reserved8 = 0x8,
    Reserved9 = 0x9,
    ReservedA = 0xA,
    ReservedB = 0xB,
    ReservedC = 0xC,
    ReservedD = 0xD,
    ReservedE = 0xE,
    ReservedF = 0xF,
}

fn mt_packet_words(mt: MessageType) -> u8 {
    use MessageType::*;
    match mt {
        Utility => 1,
        SystemRealTimeAndCommon => 1,
        Midi1ChannelVoice => 1,
        DataInclSystemExclusive => 2,
        Midi2ChannelVoice => 2,
        Data => 4,
        Reserved6 => 1,
        Reserved7 => 1,
        Reserved8 => 2,
        Reserved9 => 2,
        ReservedA => 2,
        ReservedB => 3,
        ReservedC => 3,
        ReservedD => 4,
        ReservedE => 4,
        ReservedF => 4,
    }
}

impl Packet {
    pub fn packet_words(&self) -> Result<u8> {
        Ok(mt_packet_words(self.message_type()?))
    }
}
