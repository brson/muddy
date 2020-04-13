//! Universal MIDI Packet format

#![allow(unused)]

use anyhow::{Result, Error, bail};
use std::convert::{TryInto, TryFrom};
use num_enum::TryFromPrimitive;

struct Packet {
    data: [u32; 4],
    length: PacketLength,
}

enum PacketLength { One, Two, Three, Four }

impl Packet {
    fn message_type(&self) -> Result<MessageType> {
        let msb4 = self.data[0] >> 28;
        let msb4: u8 = msb4.try_into()?;
        let msg_type: MessageType = msb4.try_into()?;
        Ok(msg_type)
    }
}

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
enum MessageType {
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

// UMP 2.1.4
fn packet_words(mt: MessageType) -> u8 {
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

