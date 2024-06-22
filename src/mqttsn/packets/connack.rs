use crate::mqttsn::traits::packet::MqttSnPacket;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum ConnackReturnCode {
    Accepted = 0x00,
    RejectedCongestion = 0x01,
    RejectedInvalidTopicId = 0x02,
    RejectedNotSupported = 0x03,
}

impl From<u8> for ConnackReturnCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => ConnackReturnCode::Accepted,
            0x01 => ConnackReturnCode::RejectedCongestion,
            0x02 => ConnackReturnCode::RejectedInvalidTopicId,
            0x03 => ConnackReturnCode::RejectedNotSupported,
            _ => ConnackReturnCode::Accepted,
        }
    }
}

impl From<ConnackReturnCode> for u8 {
    fn from(return_code: ConnackReturnCode) -> Self {
        return_code as u8
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ConnackPacket {
    pub msg_type: MqttSnPacketType,
    pub return_code: ConnackReturnCode,
}

// A CONNACK packet should look like this:
// Length: 0x03
// MsgType: CONNACK = 0x05
// ReturnCode: Return code

impl MqttSnPacket for ConnackPacket {
    fn get_length(&self) -> u8 {
        0x03
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Connack
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        ConnackPacket {
            msg_type: bytes[1].into(),
            return_code: bytes[2].into(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.get_length());
        bytes.push(self.msg_type.into());
        bytes.push(self.return_code.clone().into());
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
    fn test_get_length() {
        let packet = ConnackPacket {
            msg_type: MqttSnPacketType::Connack,
            return_code: ConnackReturnCode::Accepted,
        };
        assert_eq!(packet.get_length(), 0x03);
    }

    #[test]
    fn test_get_type() {
        let packet = ConnackPacket {
            msg_type: MqttSnPacketType::Connack,
            return_code: ConnackReturnCode::Accepted,
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Connack);
    }

    #[test]
    fn test_as_bytes() {
        let packet = ConnackPacket {
            msg_type: MqttSnPacketType::Connack,
            return_code: ConnackReturnCode::Accepted,
        };
        assert_eq!(packet.as_bytes(), vec![0x03, 0x05, 0x00]);
    }

    #[test]
    fn test_from_bytes() {
        let packet = ConnackPacket {
            msg_type: MqttSnPacketType::Connack,
            return_code: ConnackReturnCode::Accepted,
        };
        let bytes = vec![0x03, 0x05, 0x00];
        assert_eq!(ConnackPacket::from_bytes(&bytes), packet);
    }

    #[test]
    fn test_as_any() {
        let packet = ConnackPacket {
            msg_type: MqttSnPacketType::Connack,
            return_code: ConnackReturnCode::Accepted,
        };
        assert!(packet.as_any().is::<ConnackPacket>());
    }
}