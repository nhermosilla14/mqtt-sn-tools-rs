use crate::mqttsn::traits::packet::MqttSnPacket;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;

pub struct RegisterPacket {
    pub topic_id: u16,
    pub message_id: u16,
    pub topic_name: String,
}

impl MqttSnPacket for RegisterPacket {
    fn get_length(&self) -> u8 {
        0x07 + self.topic_name.len() as u8
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Register
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        RegisterPacket {
            topic_id: u16::from_be_bytes([bytes[1], bytes[2]]),
            message_id: u16::from_be_bytes([bytes[3], bytes[4]]),
            topic_name: String::from_utf8(bytes[5..].to_vec()).unwrap(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.get_length());
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.extend_from_slice(self.topic_name.as_bytes());
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
        let packet = RegisterPacket {
            topic_id: 1,
            message_id: 2,
            topic_name: "test".to_string(),
        };
        assert_eq!(packet.get_length(), 0x0B);
    }

    #[test]
    fn test_get_type() {
        let packet = RegisterPacket {
            topic_id: 1,
            message_id: 2,
            topic_name: "test".to_string(),
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Register);
    }

    #[test]
    fn test_as_bytes() {
        let packet = RegisterPacket {
            topic_id: 1,
            message_id: 2,
            topic_name: "test".to_string(),
        };
        assert_eq!(packet.as_bytes(), vec![0x0B, 0x00, 0x01, 0x00, 0x02, 0x74, 0x65, 0x73, 0x74]);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0x0B, 0x00, 0x01, 0x00, 0x02, 0x74, 0x65, 0x73, 0x74];
        let packet = RegisterPacket::from_bytes(&bytes);
        assert_eq!(packet.topic_id, 1);
        assert_eq!(packet.message_id, 2);
        assert_eq!(packet.topic_name, "test".to_string());
    }
}