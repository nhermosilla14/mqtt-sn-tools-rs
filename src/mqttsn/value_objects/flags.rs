use super::topic_id::TopicIdType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Qos {
    QOSN1,
    QOS0,
    QOS1,
    QOS2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Flags {
    pub dup: bool,
    pub qos: Qos,
    pub retain: bool,
    pub will: bool,
    pub clean_session: bool,
    pub topic_id_type: TopicIdType,
}

impl Flags {
    pub fn from_byte(byte: u8) -> Self {
        Flags {
            dup: (byte & 0b1000_0000) != 0,
            qos: match (byte & 0b0110_0000) >> 5 {
                0 => Qos::QOS0,
                1 => Qos::QOS1,
                2 => Qos::QOS2,
                _ => Qos::QOSN1,
            },
            retain: (byte & 0b0001_0000) != 0,
            will: (byte & 0b0000_1000) != 0,
            clean_session: (byte & 0b0000_0100) != 0,
            topic_id_type: match byte & 0b0000_0011 {
                0 => TopicIdType::Normal,
                1 => TopicIdType::Predefined,
                _ => TopicIdType::Short,
            },
        }
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte = 0;
        if self.dup {
            byte |= 0b1000_0000;
        }
        byte |= match self.qos {
            Qos::QOS0 => 0,
            Qos::QOS1 => 1 << 5,
            Qos::QOS2 => 2 << 5,
            Qos::QOSN1 => 3 << 5,
        };
        if self.retain {
            byte |= 0b0001_0000;
        }
        if self.will {
            byte |= 0b0000_1000;
        }
        if self.clean_session {
            byte |= 0b0000_0100;
        }
        byte |= match self.topic_id_type {
            TopicIdType::Normal => 0,
            TopicIdType::Predefined => 1,
            TopicIdType::Short => 2,
        };
        byte
    }

    pub fn new(
        dup: bool,
        qos: Qos,
        retain: bool,
        will: bool,
        clean_session: bool,
        topic_id_type: TopicIdType,
    ) -> Self {
        Flags {
            dup,
            qos,
            retain,
            will,
            clean_session,
            topic_id_type,
        }
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags::from_byte(byte)
    }
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        flags.as_byte()
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte() {
        let flags = Flags::from_byte(0b0000_0000);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b1000_0000);
        assert_eq!(flags.dup, true);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b0100_0000);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS2);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b0001_0000);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, true);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b0000_1000);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, true);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b0000_0100);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, true);
        assert_eq!(flags.topic_id_type, TopicIdType::Normal);

        let flags = Flags::from_byte(0b0000_0011);
        assert_eq!(flags.dup, false);
        assert_eq!(flags.qos, Qos::QOS0);
        assert_eq!(flags.retain, false);
        assert_eq!(flags.will, false);
        assert_eq!(flags.clean_session, false);
        assert_eq!(flags.topic_id_type, TopicIdType::Short);
    }

    #[test]
    fn test_as_byte() {
        let flags = Flags {
            dup: false,
            qos: Qos::QOS0,
            retain: false,
            will: false,
            clean_session: false,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b0000_0000);

        let flags = Flags {
            dup: true,
            qos: Qos::QOS0,
            retain: false,
            will: false,
            clean_session: false,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b1000_0000);

        let flags = Flags {
            dup: false,
            qos: Qos::QOS2,
            retain: false,
            will: false,
            clean_session: false,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b0100_0000);

        let flags = Flags {
            dup: false,
            qos: Qos::QOS0,
            retain: true,
            will: false,
            clean_session: false,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b0001_0000);

        let flags = Flags {
            dup: false,
            qos: Qos::QOS0,
            retain: false,
            will: true,
            clean_session: false,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b0000_1000);

        let flags = Flags {
            dup: false,
            qos: Qos::QOS0,
            retain: false,
            will: false,
            clean_session: true,
            topic_id_type: TopicIdType::Normal,
        };
        assert_eq!(flags.as_byte(), 0b0000_0100);

        let flags = Flags {
            dup: false,
            qos: Qos::QOS0,
            retain: false,
            will: false,
            clean_session: false,
            topic_id_type: TopicIdType::Short,
        };
        assert_eq!(flags.as_byte(), 0b0000_0010);
    }
}