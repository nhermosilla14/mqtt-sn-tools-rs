

// This module defines the abstraction for parsing MQTT-SN packets.

use crate::mqttsn::traits::Packet;
use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;
use crate::mqttsn::packets::{
    ConnectPacket,
    ConnackPacket,
    PubackPacket,
    PublishPacket,
    RegisterPacket,
    RegackPacket,
    SubscribePacket,
    SubackPacket,
};


pub fn parse_packet(bytes: &Vec<u8>) -> Box<dyn Packet> {
    let packet_type = MqttSnPacketType::from_byte(bytes[0]);
    match packet_type {
        MqttSnPacketType::Connect => Box::new(ConnectPacket::from_bytes(bytes)),
        MqttSnPacketType::Connack => Box::new(ConnackPacket::from_bytes(bytes)),
        MqttSnPacketType::Publish => Box::new(PublishPacket::from_bytes(bytes)),
        MqttSnPacketType::Puback => Box::new(PubackPacket::from_bytes(bytes)),
        MqttSnPacketType::Register => Box::new(RegisterPacket::from_bytes(bytes)),
        MqttSnPacketType::Regack => Box::new(RegackPacket::from_bytes(bytes)),
        MqttSnPacketType::Subscribe => Box::new(SubscribePacket::from_bytes(bytes)),
        MqttSnPacketType::Suback => Box::new(SubackPacket::from_bytes(bytes)),
        _ => Box::new(ConnectPacket::from_bytes(bytes)),
    }
}