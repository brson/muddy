#![allow(unused)]


pub mod v1 {
    use anyhow::{Result, bail};

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
        fn validate(&self) -> Result<()> {
            if self.0 & (1_u8 << 7) != 0 {
                bail!("status byte without msb");
            }
            Ok(())
        }
    }
}
