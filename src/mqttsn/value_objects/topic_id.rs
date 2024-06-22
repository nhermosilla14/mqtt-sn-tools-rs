
#[derive(Debug, PartialEq)]
pub enum TopicId {
    Predefined(u16),
    Normal(u16),
    Short(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TopicIdType {
    Normal,
    Predefined,
    Short,
}

impl From<u8> for TopicIdType {
    fn from(byte: u8) -> Self {
        match byte {
            0 => TopicIdType::Normal,
            1 => TopicIdType::Predefined,
            2 => TopicIdType::Short,
            _ => panic!("Invalid TopicIdType"),
        }
    }
}

impl From<TopicIdType> for u8 {
    fn from(topic_id_type: TopicIdType) -> Self {
        match topic_id_type {
            TopicIdType::Normal => 0,
            TopicIdType::Predefined => 1,
            TopicIdType::Short => 2,
        }
    }
}


impl TopicId {
    pub fn from_bytes(bytes: &[u8], topic_id_type: TopicIdType) -> Self {
        match topic_id_type {
            TopicIdType::Normal => TopicId::Normal(u16::from_be_bytes([bytes[0], bytes[1]])),
            TopicIdType::Predefined => TopicId::Predefined(u16::from_be_bytes([bytes[0], bytes[1]])),
            TopicIdType::Short => TopicId::Short(bytes[0]),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            TopicId::Normal(id) => vec![((*id >> 8) as u8), (*id as u8)],
            TopicId::Predefined(id) => vec![((*id >> 8) as u8), (*id as u8)],
            TopicId::Short(id) => vec![*id],
        }
    }
}

