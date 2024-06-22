use crate::mqttsn::traits::packet::MqttSnPacket;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;
use crate::mqttsn::value_objects::topic_id::TopicId;
use crate::mqttsn::value_objects::flags::Flags;

// A PUBLISH packet should look like this:
// Length: 7 + Data length
// MsgType: PUBLISH = 0x0C
// Flags: DUP, QoS, Retain, TopicIdType. Will and CleanSession flags are unused
// TopicID: Topic identifier (TopicID value or Short Topic Name)
// MessageID: Message identifier (only if QoS > 0)
// Data: The published data

pub struct PublishPacket {
    pub msg_type: MqttSnPacketType,
    pub flags: Flags,
    pub topic_id: TopicId,
    pub msg_id: u16,
    pub data: Vec<u8>,
}

impl MqttSnPacket for PublishPacket {
    fn get_length(&self) -> u8 {
        0x07 + self.data.len() as u8
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Publish
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        let flags = Flags::from(bytes[2]);
        PublishPacket {
            msg_type: MqttSnPacketType::Publish,
            flags: flags,
            topic_id: TopicId::from_bytes(&bytes[3..5].to_vec(), flags.topic_id_type),
            msg_id: u16::from_be_bytes([bytes[5], bytes[6]]),
            data: bytes[7..].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.get_length());
        bytes.push(self.msg_type.into());
        bytes.push(self.flags.clone().into());
        bytes.extend_from_slice(self.topic_id.as_bytes().as_slice());
        bytes.push((self.msg_id >> 8) as u8);
        bytes.push(self.msg_id as u8);
        bytes.extend_from_slice(self.data.as_slice());
        bytes
    }
    
    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }
}


// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_length() {
        let packet = PublishPacket {
            msg_type: MqttSnPacketType::Publish,
            flags: Flags::from(0),
            topic_id: TopicId::Normal(1),
            msg_id: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        assert_eq!(packet.get_length(), 0x0D);
    }

    #[test]
    fn test_get_type() {
        let packet = PublishPacket {
            msg_type: MqttSnPacketType::Publish,
            flags: Flags::from(0),
            topic_id: TopicId::Normal(1),
            msg_id: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Publish);
    }

    #[test]
    fn test_as_bytes() {
        let packet = PublishPacket {
            msg_type: MqttSnPacketType::Publish,
            flags: Flags::from(0),
            topic_id: TopicId::Normal(1),
            msg_id: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let flags = Flags::from(0);
        // Length, MsgType, Flags,  TopicID,     MsgID,              Data
        //  0x0B,   0x0C,   0x00,  0x00, 0x01, 0x00, 0x02,  0x01, 0x02, 0x03, 0x04, 0x05
        let flags_byte: u8 = flags.into();
        assert_eq!(packet.as_bytes(), vec![0x0D, 0x0C, flags_byte, 0x00, 0x01, 0x00, 0x02, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0x0D, 0x0C, 0x00, 0x00, 0x01, 0x00, 0x02, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let packet = PublishPacket::from_bytes(&bytes);
        assert_eq!(packet.msg_type, MqttSnPacketType::Publish);
        assert_eq!(packet.flags, Flags::from(0));
        assert_eq!(packet.topic_id, TopicId::Normal(1));
        assert_eq!(packet.msg_id, 2);
        assert_eq!(packet.data, vec![1, 2, 3, 4, 5, 6]);
    }
}