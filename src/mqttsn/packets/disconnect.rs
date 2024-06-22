use crate::mqttsn::traits::packet::MqttSnPacket;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;

// A DISCONNECT packet should look like this:
// Length: 0x02
// MsgType: DISCONNECT = 0x14
// Duration: Duration in seconds the client will be offline
//

pub struct DisconnectPacket {
    pub msg_type: MqttSnPacketType,
    pub duration: Option<u16>
}

impl MqttSnPacket for DisconnectPacket {
    fn get_length(&self) -> u8 {
        match self.duration {
            Some(_) => 0x04,
            None => 0x02,
        }
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Disconnect
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        match bytes.len() {
            4 => DisconnectPacket {
                msg_type: MqttSnPacketType::Disconnect,
                duration: Some(u16::from_be_bytes([bytes[2], bytes[3]])),
            },
            2 => DisconnectPacket {
                msg_type: MqttSnPacketType::Disconnect,
                duration: None,
            },
            _ => panic!("Invalid DISCONNECT packet length"),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let msg_type_byte: u8 = self.msg_type.into();
        bytes.push(self.get_length());
        bytes.push(msg_type_byte);
        match self.duration {
            Some(duration) => {
                bytes.push((duration >> 8) as u8);
                bytes.push(duration as u8);
            },
            None => {},
        }
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
        let packet = DisconnectPacket {
            msg_type: MqttSnPacketType::Disconnect,
            duration: None,
        };
        assert_eq!(packet.get_length(), 0x02);
    }

    #[test]
    fn test_length_with_duration() {
        let packet = DisconnectPacket {
            msg_type: MqttSnPacketType::Disconnect,
            duration: Some(10),
        };
        assert_eq!(packet.get_length(), 0x04);
    }

    #[test]
    fn test_type() {
        let packet = DisconnectPacket {
            msg_type: MqttSnPacketType::Disconnect,
            duration: None,
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Disconnect);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0x02, 0x14];
        let packet = DisconnectPacket::from_bytes(&bytes);
        assert_eq!(packet.msg_type, MqttSnPacketType::Disconnect);
        assert_eq!(packet.duration, None);
    }

    #[test]
    fn test_from_bytes_with_duration() {
        let bytes = vec![0x04, 0x14, 0x00, 0x0A];
        let packet = DisconnectPacket::from_bytes(&bytes);
        assert_eq!(packet.msg_type, MqttSnPacketType::Disconnect);
        assert_eq!(packet.duration, Some(10));
    }

    #[test]
    fn test_as_bytes() {
        let packet = DisconnectPacket {
            msg_type: MqttSnPacketType::Disconnect,
            duration: None,
        };
        assert_eq!(packet.as_bytes(), vec![0x02, 0x18]);
    }

    #[test]
    fn test_as_bytes_with_duration() {
        let packet = DisconnectPacket {
            msg_type: MqttSnPacketType::Disconnect,
            duration: Some(10),
        };
        assert_eq!(packet.as_bytes(), vec![0x04, 0x18, 0x00, 0x0A]);
    }
}