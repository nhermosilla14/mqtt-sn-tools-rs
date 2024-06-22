use crate::mqttsn::traits::packet::MqttSnPacket;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;

pub struct RegackPacket {
    pub topic_id: u16,
    pub msg_id: u16,
    pub return_code: u8,
}

impl MqttSnPacket for RegackPacket {
    fn get_length(&self) -> u8 {
        0x07
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Regack
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        RegackPacket {
            topic_id: u16::from_be_bytes([bytes[1], bytes[2]]),
            msg_id: u16::from_be_bytes([bytes[3], bytes[4]]),
            return_code: bytes[5],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.get_length());
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.msg_id >> 8) as u8);
        bytes.push(self.msg_id as u8);
        bytes.push(self.return_code);
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
        let packet = RegackPacket {
            topic_id: 1,
            msg_id: 2,
            return_code: 0,
        };
        assert_eq!(packet.get_length(), 0x07);
    }

    #[test]
    fn test_get_type() {
        let packet = RegackPacket {
            topic_id: 1,
            msg_id: 2,
            return_code: 0,
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Regack);
    }

    #[test]
    fn test_as_bytes() {
        let packet = RegackPacket {
            topic_id: 1,
            msg_id: 2,
            return_code: 0,
        };
        assert_eq!(packet.as_bytes(), vec![0x07, 0x00, 0x01, 0x00, 0x02, 0x00]);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0x07, 0x00, 0x01, 0x00, 0x02, 0x00];
        let packet = RegackPacket::from_bytes(&bytes);
        assert_eq!(packet.topic_id, 1);
        assert_eq!(packet.msg_id, 2);
        assert_eq!(packet.return_code, 0);
    }
}