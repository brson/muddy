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

        pub fn message_type(&self) -> Result<Message> {
            panic!()
        }

        pub fn num_data_bytes(&self) -> Result<DataBytes> {
            let msg = self.message_type()?;
            use Message::*;
            use ChannelMessage::*;
            use SystemMessage::*;
            use DataBytes::*;
            Ok(match msg {
                Channel(NoteOff) => Fixed(2),
                Channel(NoteOn) => Fixed(2),
                Channel(PolyphonicKeyPressureOrAftertouch) => Fixed(2),
                Channel(ControlChangeOrChannelMode) => Fixed(2),
                Channel(ProgramChange) => Fixed(1),
                Channel(ChannelPressureOrAftertouch) => Fixed(1),
                Channel(PitchBendChange) => Fixed(2),
                System(Exclusive) => UntilEox,
                System(Common) => panic!(),
                System(RealTime) => Fixed(0),
            })
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

    pub enum DataBytes {
        Fixed(u8),
        UntilEox,
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
