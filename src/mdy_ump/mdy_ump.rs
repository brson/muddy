//! Universal MIDI Packet format
//!
//! As defined in M2-104-UM version 1.0.

#![allow(unused)]

mod types;

mod ci_negotiation {
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

    pub struct Extensions;

    impl NegotiationBytes {
        fn protocol_type(&self) -> Result<ProtocolType> {
            Ok(self.data[0].try_into()?)
        }

        fn protocol_version(&self) -> Result<ProtocolVersion> {
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

        fn validate(&self) -> Result<()> {
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
    }
}
