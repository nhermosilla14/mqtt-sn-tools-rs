use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;
use crate::mqttsn::traits::packet::MqttSnPacket;

#[derive(Debug)]
#[derive(Clone)]
pub struct ConnectPacket {
    pub msg_type: MqttSnPacketType,
    pub flags: u8,
    pub protocol_id: u8,
    pub duration: u16,
    pub client_id: Vec<u8>,
}

// A CONNECT packet should look like this:
// Length: 6 + ClientID length
// MsgType: CONNECT = 0x04
// Flags: Only Will and CleanSession flags can be set
// ProtocolID: 0x01 = MQTT-SN
// Duration: Keep alive duration in seconds
// ClientID: Client identifier


impl MqttSnPacket for ConnectPacket {
    fn get_length(&self) -> u8 {
        0x06 + self.client_id.len() as u8
    }

    fn get_type(&self) -> MqttSnPacketType {
        MqttSnPacketType::Connect
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        ConnectPacket {
            msg_type: bytes[1].into(),
            flags: bytes[2],
            protocol_id: bytes[3],
            duration: u16::from_be_bytes([bytes[4], bytes[5]]),
            client_id: bytes[6..].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.get_length());
        bytes.push(self.msg_type.into());
        bytes.push(self.flags);
        bytes.push(self.protocol_id);
        bytes.push((self.duration >> 8) as u8);
        bytes.push(self.duration as u8);
        bytes.extend_from_slice(self.client_id.as_slice());
        bytes
    }
    
    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let packet = ConnectPacket {
            msg_type: MqttSnPacketType::Connect,
            flags: 0,
            protocol_id: 1,
            duration: 500,
            client_id: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(packet.get_length(), 0x0B);
    }

    #[test]
    fn test_get_type() {
        let packet = ConnectPacket {
            msg_type: MqttSnPacketType::Connect,
            flags: 0,
            protocol_id: 1,
            duration: 500,
            client_id: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(packet.get_type(), MqttSnPacketType::Connect);
    }

    #[test]
    fn test_from_bytes() {
        // A connect package with the following data:
        // MsgType: CONNECT = 0x04
        // Flags: 0x00
        // ProtocolID: 0x01 = MQTT-SN
        // Duration: 500 (0x01F4)
        // ClientID: 0x01, 0x02, 0x03, 0x04, 0x05
        //
        // Should be represented as the following bytes:
        // 0x0B, 0x04, 0x00, 0x01, 
        let bytes = vec![0x0B, 0x04, 0x00, 0x01, 0x01, 0xF4, 0x01, 0x02, 0x03, 0x04, 0x05];
        let packet = ConnectPacket::from_bytes(&bytes);
        assert_eq!(packet.get_length(), 0x0B);
        assert_eq!(packet.msg_type, MqttSnPacketType::Connect);
        assert_eq!(packet.flags, 0x00);
        assert_eq!(packet.protocol_id, 0x01);
        assert_eq!(packet.duration, 500);
        assert_eq!(packet.client_id, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_as_bytes() {
        let packet = ConnectPacket {
            msg_type: MqttSnPacketType::Connect,
            flags: 0x00,
            protocol_id: 0x01,
            duration: 500,
            client_id: vec![1, 2, 3, 4, 5],
        };
        let bytes = packet.as_bytes();
        assert_eq!(bytes, vec![0x0B, 0x04, 0x00, 0x01, 0x01, 0xF4, 0x01, 0x02, 0x03, 0x04, 0x05]);
    }
}