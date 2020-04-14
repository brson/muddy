#![allow(unused)]


pub mod v1 {
    use anyhow::{Result, bail};
    use num_enum::TryFromPrimitive;

    pub struct StatusByte(u8);
    pub struct DataByte(u8);

    pub enum MessageType {
        Channel,
        System,
    }

    pub enum ChannelMessageType {
        /// Controls an instrument's voice
        Voice,
        /// Controls an instrument's response to voice messages
        Mode,
    }

    pub enum SystemMessageType {
        /// For all receivers regardless of channel
        Common,
        /// Synchronization
        RealTime,
        /// Manufacturer-specific messages
        Exclusive,
    }

    impl StatusByte {
        pub fn validate(&self) -> Result<()> {
            if self.0 >> 7 == 0 {
                bail!("status byte wrong msb");
            }
            Ok(())
        }

    }

    pub enum Message {
        Channel(ChannelMessage),
        System(SystemMessage),
    }

    #[derive(Copy, Clone, TryFromPrimitive)]
    #[repr(u8)]
    pub enum ChannelMessage {
        NoteOff = 0b1000,
        NoteOn = 0b1001,
        PolyphonicKeyPressureOrAftertouch = 0b1010,
        ControlChangeOrChannelMode = 0b1011,
        ProgramChange = 0b1100,
        ChannelPressureOrAftertouch = 0b1101,
        PitchBendChange = 0b1110,
    }

    #[derive(Copy, Clone)]
    pub enum SystemMessage {
        Exclusive, Common, RealTime,
    }

    impl DataByte {
        pub fn validate(&self) -> Result<()> {
            if self.0 >> 7 == 1 {
                bail!("data byte wrong msb");
            }
            Ok(())
        }
    }

    #[repr(u8)]
    pub enum Mode {
        OmniOnPoly = 1,
        OmniOnMono = 2,
        OmniOffPoly = 3,
        OmniOffMono = 4,
    }

    pub enum VoiceMessageType {
        NoteOn,
        NoteOff,
        ControlChange,
        ProgramChange,
        AfterTouch,
        PitchBendChange,
    }
}
