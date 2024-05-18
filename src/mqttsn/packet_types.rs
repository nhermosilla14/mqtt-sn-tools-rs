#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt::Debug;

// This module contains structs and enums for the different MQTT-SN packet types.
// Import constants
use crate::mqttsn::constants::*;

// A simple function to convert a packet type from 
// its byte representation to a string slice
pub fn mqtt_sn_packet_type_to_str(packet_type: u8) -> &'static str {
    match packet_type {
        MQTT_SN_ADVERTISE => "ADVERTISE",
        MQTT_SN_SEARCHGW => "SEARCHGW",
        MQTT_SN_GWINFO => "GWINFO",
        MQTT_SN_CONNECT => "CONNECT",
        MQTT_SN_CONNACK => "CONNACK",
        MQTT_SN_WILLTOPICREQ => "WILLTOPICREQ",
        MQTT_SN_WILLTOPIC => "WILLTOPIC",
        MQTT_SN_WILLMSGREQ => "WILLMSGREQ",
        MQTT_SN_WILLMSG => "WILLMSG",
        MQTT_SN_REGISTER => "REGISTER",
        MQTT_SN_REGACK => "REGACK",
        MQTT_SN_PUBLISH => "PUBLISH",
        MQTT_SN_PUBACK => "PUBACK",
        MQTT_SN_PUBCOMP => "PUBCOMP",
        MQTT_SN_PUBREC => "PUBREC",
        MQTT_SN_PUBREL => "PUBREL",
        MQTT_SN_SUBSCRIBE => "SUBSCRIBE",
        MQTT_SN_SUBACK => "SUBACK",
        MQTT_SN_UNSUBSCRIBE => "UNSUBSCRIBE",
        MQTT_SN_UNSUBACK => "UNSUBACK",
        MQTT_SN_PINGREQ => "PINGREQ",
        MQTT_SN_PINGRESP => "PINGRESP",
        MQTT_SN_DISCONNECT => "DISCONNECT",
        MQTT_SN_WILLTOPICUPD => "WILLTOPICUPD",
        MQTT_SN_WILLTOPICRESP => "WILLTOPICRESP",
        MQTT_SN_WILLMSGUPD => "WILLMSGUPD",
        MQTT_SN_WILLMSGRESP => "WILLMSGRESP",
        MQTT_SN_FRWDENCAP => "FRWDENCAP",
        _ => "UNKNOWN",
    }
}



// Define a generic Packet trait
pub trait Packet: Debug {
    fn length(&self) -> u8 { 0 }
    fn msg_type(&self) -> u8 { 0 }
    fn as_bytes(&self) -> Vec<u8> { Vec::new() }
    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized{
        unimplemented!()
    }
    fn as_connect(&self) -> Option<&ConnectPacket> { None }
    fn as_register(&self) -> Option<&RegisterPacket> { None }
    fn as_publish(&self) -> Option<&PublishPacket> { None }
    fn as_subscribe(&self) -> Option<&SubscribePacket> { None }
    fn as_regack(&self) -> Option<&RegackPacket> { None }
    fn as_suback(&self) -> Option<&SubackPacket> { None }
    fn as_puback(&self) -> Option<&PubackPacket> { None }
    fn as_connack(&self) -> Option<&ConnackPacket> { None }
    fn as_disconnect(&self) -> Option<&DisconnectPacket> { None }
    fn as_frwdencap(&self) -> Option<&FWDEncapPacket> { None }
}



// Connect
#[derive(Debug)]
#[derive(Clone)]
pub struct ConnectPacket {
    pub length: u8,
    pub msg_type: u8,
    pub flags: u8,
    pub protocol_id: u8,
    pub duration: u16,
    pub client_id: Vec<u8>,
}

impl Packet for ConnectPacket {
    fn length(&self) -> u8 {
        0x06 + self.client_id.len() as u8
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_CONNECT
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        ConnectPacket {
            length: bytes[0],
            msg_type: bytes[1],
            flags: bytes[2],
            protocol_id: bytes[3],
            duration: u16::from_be_bytes([bytes[4], bytes[5]]),
            client_id: bytes[6..].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.flags);
        bytes.push(self.protocol_id);
        bytes.push((self.duration >> 8) as u8);
        bytes.push(self.duration as u8);
        bytes.extend_from_slice(self.client_id.as_slice());
        bytes
    }

}

// Connack
#[derive(Debug)]
#[derive(Clone)]
pub struct ConnackPacket {
    pub length: u8,
    pub msg_type: u8,
    pub return_code: u8,
}

impl Packet for ConnackPacket {
    fn length(&self) -> u8 {
        3
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_CONNACK
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        ConnackPacket {
            length: bytes[0],
            msg_type: bytes[1],
            return_code: bytes[2],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.return_code);
        bytes
    }

    fn as_connack(&self) -> Option<&ConnackPacket> {
        Some(self)
    }
}

// Register
#[derive(Debug)]
#[derive(Clone)]
pub struct RegisterPacket {
    pub length: u8,
    pub msg_type: u8,
    pub topic_id: u16,
    pub message_id: u16,
    pub topic_name: Vec<u8>,
}

impl Packet for RegisterPacket {
    fn length(&self) -> u8 {
        0x06 + self.topic_name.len() as u8
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_REGISTER
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        RegisterPacket {
            length: bytes[0],
            msg_type: bytes[1],
            topic_id: u16::from_be_bytes([bytes[2], bytes[3]]),
            message_id: u16::from_be_bytes([bytes[4], bytes[5]]),
            topic_name: bytes[6..].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.extend_from_slice(self.topic_name.as_slice());
        bytes

    }
}

// Regack
#[derive(Debug)]
#[derive(Clone)]
pub struct RegackPacket {
    pub length: u8,
    pub msg_type: u8,
    pub topic_id: u16,
    pub message_id: u16,
    pub return_code: u8,
}

impl Packet for RegackPacket {
    fn length(&self) -> u8 {
        7
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_REGACK
    }

    fn from_bytes(bytes: &Vec<u8>) -> RegackPacket {
        RegackPacket {
            length: bytes[0],
            msg_type: bytes[1],
            topic_id: u16::from_be_bytes([bytes[2], bytes[3]]),
            message_id: u16::from_be_bytes([bytes[4], bytes[5]]),
            return_code: bytes[6],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.push(self.return_code);
        bytes
    }

    fn as_regack(&self) -> Option<&RegackPacket> {
        Some(self)
    }
}

// Publish
#[derive(Debug)]
#[derive(Clone)]
pub struct PublishPacket {
    pub length: u8,
    pub msg_type: u8,
    pub flags: u8,
    pub topic_id: u16,
    pub message_id: u16,
    pub data: Vec<u8>,
}

impl Packet for PublishPacket {
    fn length(&self) -> u8 {
        0x07 + self.data.len() as u8
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_PUBLISH
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        let length = bytes[0] as usize;
        PublishPacket {
            length: bytes[0],
            msg_type: bytes[1],
            flags: bytes[2],
            topic_id: u16::from_be_bytes([bytes[3], bytes[4]]),
            message_id: u16::from_be_bytes([bytes[5], bytes[6]]),
            data: bytes[7..length].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.flags);
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.extend_from_slice(&self.data);
        bytes
    }

    fn as_publish(&self) -> Option<&PublishPacket> {
        Some(self)
    }
}

// Puback
#[derive(Debug)]
#[derive(Clone)]
pub struct PubackPacket {
    pub length: u8,
    pub msg_type: u8,
    pub topic_id: u16,
    pub message_id: u16,
    pub return_code: u8,
}

impl Packet for PubackPacket {
    fn length(&self) -> u8 {
        7
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_PUBACK
    }

    fn from_bytes(bytes: &Vec<u8>) -> PubackPacket {
        PubackPacket {
            length: bytes[0],
            msg_type: bytes[1],
            topic_id: u16::from_be_bytes([bytes[2], bytes[3]]),
            message_id: u16::from_be_bytes([bytes[4], bytes[5]]),
            return_code: bytes[6],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.push(self.return_code);
        bytes
    }

    fn as_puback(&self) -> Option<&PubackPacket> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct PingreqPacket {
    pub length: u8,
    pub msg_type: u8,
}

impl Packet for PingreqPacket {
    fn length(&self) -> u8 {
        0x02
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_PINGREQ
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        PingreqPacket {
            length: bytes[0],
            msg_type: bytes[1],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes
    }
}


#[derive(Debug)]
pub struct PingrespPacket {
    pub length: u8,
    pub msg_type: u8,
}

impl Packet for PingrespPacket {
    fn length(&self) -> u8 {
        0x02
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_PINGRESP
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        PingrespPacket {
            length: bytes[0],
            msg_type: bytes[1],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes
    }
}


// Topics enum
// This could be an array of bytes or a topic id
#[derive(Debug, Clone)]
pub enum Topic {
    TopicId(u16),
    TopicName(Vec<u8>),
}

// Subscribe
#[derive(Debug)]
#[derive(Clone)]
pub struct SubscribePacket {
    pub length: u8,
    pub msg_type: u8,
    pub flags: u8,
    pub message_id: u16,
    pub topic: Topic,
}

impl Packet for SubscribePacket {
    fn length(&self) -> u8 {
        0x05 + match self.topic {
            Topic::TopicId(_) => 2,
            Topic::TopicName(ref name) => name.len() as u8,
        }
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_SUBSCRIBE
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        let topic = if (bytes[2] & MQTT_SN_FLAG_QOS_MASK)  == MQTT_SN_TOPIC_TYPE_PREDEFINED {
            Topic::TopicId(u16::from_be_bytes([bytes[5], bytes[6]]))
        } else {
            Topic::TopicName(bytes[5..].to_vec())
        };

        SubscribePacket {
            length: bytes[0],
            msg_type: bytes[1],
            flags: bytes[2],
            message_id: u16::from_be_bytes([bytes[5], bytes[6]]),
            topic: topic,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.flags);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        match self.topic {
            Topic::TopicId(id) => {
                bytes.push((id >> 8) as u8);
                bytes.push(id as u8);
            },
            Topic::TopicName(ref name) => {
                bytes.extend_from_slice(name.as_slice());
            },
        }
        bytes
    }
}

// Suback
#[derive(Debug)]
#[derive(Clone)]
pub struct SubackPacket {
    pub length: u8,
    pub msg_type: u8,
    pub flags: u8,
    pub topic_id: u16,
    pub message_id: u16,
    pub return_code: u8,
}

impl Packet for SubackPacket {
    fn length(&self) -> u8 {
        8
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_SUBACK
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        SubackPacket {
            length: bytes[0],
            msg_type: bytes[1],
            flags: bytes[2],
            topic_id: u16::from_be_bytes([bytes[3], bytes[4]]),
            message_id: u16::from_be_bytes([bytes[5], bytes[6]]),
            return_code: bytes[7],
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.flags);
        bytes.push((self.topic_id >> 8) as u8);
        bytes.push(self.topic_id as u8);
        bytes.push((self.message_id >> 8) as u8);
        bytes.push(self.message_id as u8);
        bytes.push(self.return_code);
        bytes
    }

    fn as_suback(&self) -> Option<&SubackPacket> {
        Some(self)
    }
}

// Disconnect
#[derive(Debug)]
#[derive(Clone)]
pub struct DisconnectPacket {
    pub length: u8,
    pub msg_type: u8,
    pub duration: u16,
}

impl Packet for DisconnectPacket {
    fn length(&self) -> u8 {
        if self.duration == 0 {
            0x02
        } else {
            0x04
        }
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_DISCONNECT
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        DisconnectPacket {
            length: bytes[0],
            msg_type: bytes[1],
            duration: u16::from_be_bytes([bytes[2], bytes[3]]),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push((self.duration >> 8) as u8);
        bytes.push(self.duration as u8);
        bytes
    }

    fn as_disconnect(&self) -> Option<&DisconnectPacket> {
        Some(self)
    }
}

// FWDEncap
#[derive(Debug)]
#[derive(Clone)]
pub struct FWDEncapPacket {
    pub length: u8,
    pub msg_type: u8,
    pub ctrl: u8,
    pub wireless_node_id: Vec<u8>,
    pub inner_packet: Vec<u8>,
}

impl Packet for FWDEncapPacket {
    fn length(&self) -> u8 {
        self.length
    }

    fn msg_type(&self) -> u8 {
        MQTT_SN_FRWDENCAP
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized {
        let last_index = bytes[0] as usize;
        let inner_packet_length = bytes[last_index] as usize;
        let inner_packet_end = last_index + inner_packet_length;
        FWDEncapPacket {
            length: bytes[0],
            msg_type: bytes[1],
            ctrl: bytes[2],
            wireless_node_id: bytes[3..last_index].to_vec(),
            inner_packet: bytes[last_index..inner_packet_end].to_vec(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        // Create an array of bytes and push the struct fields into it
        let mut bytes = Vec::new(); 
        bytes.push(self.length);
        bytes.push(self.msg_type);
        bytes.push(self.ctrl);
        bytes.extend_from_slice(self.wireless_node_id.as_slice());
        bytes.extend_from_slice(self.inner_packet.as_slice());
        bytes
    }

    fn as_frwdencap(&self) -> Option<&FWDEncapPacket> {
        Some(self)
    }
}