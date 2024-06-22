use std::any::Any;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;

// Define a generic Packet trait
pub trait MqttSnPacket: Any {
    fn get_length(&self) -> u8 { 0 }
    fn get_type(&self) -> MqttSnPacketType { MqttSnPacketType::Unknown }
    fn as_bytes(&self) -> Vec<u8> { Vec::new() }
    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized;
    fn as_any(&self) -> &(dyn Any + 'static);
}

// Define a method to downcast a Packet trait object to a specific type
pub fn downcast_packet<T: MqttSnPacket>(packet: &dyn MqttSnPacket) -> Option<&T> {
    packet.as_any().downcast_ref::<T>()
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    // Define a dummy packet type for testing
    #[derive(Debug)]
    struct DummyPacket;

    impl MqttSnPacket for DummyPacket {
        fn get_type(&self) -> MqttSnPacketType {
            MqttSnPacketType::Advertise
        }

        fn as_any(&self) -> &(dyn Any + 'static) {
            self
        }

        fn from_bytes(_bytes: &Vec<u8>) -> Self {
            DummyPacket
        }
    }

    #[test]
    fn test_get_length() {
        let packet = DummyPacket;
        assert_eq!(packet.get_length(), 0);
    }

    #[test]
    fn test_get_type() {
        let packet = DummyPacket;
        assert_eq!(packet.get_type(), MqttSnPacketType::Advertise);
    }

    #[test]
    fn test_as_bytes() {
        let packet = DummyPacket;
        assert_eq!(packet.as_bytes(), Vec::new());
    }

    #[test]
    fn test_as_any() {
        let packet = DummyPacket;
        assert!(packet.as_any().is::<DummyPacket>());
    }

    #[test]
    fn test_downcast_packet() {
        let packet: &dyn MqttSnPacket = &DummyPacket;
        assert!(downcast_packet::<DummyPacket>(packet).is_some());
    }
}