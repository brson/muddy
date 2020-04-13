#![allow(unused)]

use anyhow::{Result, bail};
use num_enum::TryFromPrimitive;
use std::convert::TryInto;

//+ M2-104-UM 3.1.2.1 MIDI-CI Protocol Negotiation

pub struct NegotiationBytes {
    data: [u8; 5],
}

#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ProtocolType {
    Midi1 = 0x01,
    Midi2 = 0x02,
}

#[derive(Eq, PartialEq)]
pub enum ProtocolVersion {
    Midi1,
    Midi2v1,
}

impl NegotiationBytes {
    pub fn protocol_type(&self) -> Result<ProtocolType> {
        Ok(self.data[0].try_into()?)
    }

    pub fn protocol_version(&self) -> Result<ProtocolVersion> {
        let vbyte = self.data[1];
        if vbyte != 0x0 {
            bail!("unrecognized protocol version: {}", vbyte);
        }
        let type_ = self.protocol_type()?;
        Ok(match type_ {
            ProtocolType::Midi1 => ProtocolVersion::Midi1,
            ProtocolType::Midi2 => ProtocolVersion::Midi2v1,
        })
    }

    pub fn validate(&self) -> Result<()> {
        let reserved4 = self.data[3];
        let reserved5 = self.data[4];
        if reserved4 != 0 {
            bail!("incorrect reserved byte 4 in negotiation: {}", reserved4);
        }
        if reserved5 != 0 {
            bail!("incorrect reserved byte 5 in negotiation: {}", reserved5);
        }
        Ok(())
    }

    pub fn midi1_extensions(&self) -> Result<Midi1Extensions> {
        if self.protocol_type()? != ProtocolType::Midi1 {
            bail!("midi 1 extensions requested for MIDI 2 protocol negotiation");
        }
        let ext_byte = self.data[2];
        Ok(Midi1Extensions {
            large_packets: ext_byte & 2 != 0,
            jitter_reduction: ext_byte & 1 != 0,
        })
    }
}

pub struct Midi1Extensions {
    pub large_packets: bool,
    pub jitter_reduction: bool,
}
