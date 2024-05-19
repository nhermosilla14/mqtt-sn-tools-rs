// This module contains the constants used in many places over the code.


// Defaults
pub const MQTT_SN_DEFAULT_PORT: u16 = 10000;
pub const MQTT_SN_DEFAULT_TIMEOUT: u64 = 10;
pub const MQTT_SN_DEFAULT_KEEP_ALIVE: u16 = 3;

// Limits
pub const MQTT_SN_MAX_PACKET_LENGTH: usize = 255;
pub const MQTT_SN_MAX_PAYLOAD_LENGTH: usize = MQTT_SN_MAX_PACKET_LENGTH - 7;
pub const MQTT_SN_MAX_TOPIC_LENGTH: usize = MQTT_SN_MAX_PACKET_LENGTH - 6;
pub const MQTT_SN_MAX_CLIENT_ID_LENGTH: usize = 23;
pub const MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH: usize = 252;

// Message types
pub const MQTT_SN_ADVERTISE: u8 = 0x00;
pub const MQTT_SN_SEARCHGW: u8 = 0x01;
pub const MQTT_SN_GWINFO: u8 = 0x02;
pub const MQTT_SN_CONNECT: u8 = 0x04;
pub const MQTT_SN_CONNACK: u8 = 0x05;
pub const MQTT_SN_WILLTOPICREQ: u8 = 0x06;
pub const MQTT_SN_WILLTOPIC: u8 = 0x07;
pub const MQTT_SN_WILLMSGREQ: u8 = 0x08;
pub const MQTT_SN_WILLMSG: u8 = 0x09;
pub const MQTT_SN_REGISTER: u8 = 0x0A;
pub const MQTT_SN_REGACK: u8 = 0x0B;
pub const MQTT_SN_PUBLISH: u8 = 0x0C;
pub const MQTT_SN_PUBACK: u8 = 0x0D;
pub const MQTT_SN_PUBCOMP: u8 = 0x0E;
pub const MQTT_SN_PUBREC: u8 = 0x0F;
pub const MQTT_SN_PUBREL: u8 = 0x10;
pub const MQTT_SN_SUBSCRIBE: u8 = 0x12;
pub const MQTT_SN_SUBACK: u8 = 0x13;
pub const MQTT_SN_UNSUBSCRIBE: u8 = 0x14;
pub const MQTT_SN_UNSUBACK: u8 = 0x15;
pub const MQTT_SN_PINGREQ: u8 = 0x16;
pub const MQTT_SN_PINGRESP: u8 = 0x17;
pub const MQTT_SN_DISCONNECT: u8 = 0x18;
pub const MQTT_SN_WILLTOPICUPD: u8 = 0x1A;
pub const MQTT_SN_WILLTOPICRESP: u8 = 0x1B;
pub const MQTT_SN_WILLMSGUPD: u8 = 0x1C;
pub const MQTT_SN_WILLMSGRESP: u8 = 0x1D;
pub const MQTT_SN_FRWDENCAP: u8 = 0xFE;


// Return codes
pub const MQTT_SN_ACCEPTED: u8 = 0x00;
pub const MQTT_SN_REJECTED_CONGESTION: u8 = 0x01;
pub const MQTT_SN_REJECTED_INVALID_TOPIC_ID: u8 = 0x02;
pub const MQTT_SN_REJECTED_NOT_SUPPORTED: u8 = 0x03;

// Flags

pub const MQTT_SN_FLAG_DUP: u8 = 0x80;
pub const MQTT_SN_FLAG_QOS_MASK: u8 = 0x3 << 5;
pub const MQTT_SN_FLAG_QOS_0: u8 = 0x00;
pub const MQTT_SN_FLAG_QOS_1: u8 = 0x20;
pub const MQTT_SN_FLAG_QOS_2: u8 = 0x40;
pub const MQTT_SN_FLAG_QOS_N1: u8 = 0x60;
pub const MQTT_SN_FLAG_RETAIN: u8 = 0x10;
pub const MQTT_SN_FLAG_WILL: u8 = 0x08;
pub const MQTT_SN_FLAG_CLEAN: u8 = 0x04;

// Topic types
pub const MQTT_SN_TOPIC_TYPE_NORMAL: u8 = 0x00;
pub const MQTT_SN_TOPIC_TYPE_PREDEFINED: u8 = 0x01;
pub const MQTT_SN_TOPIC_TYPE_SHORT: u8 = 0x02;

// Protocol ID
pub const MQTT_SN_PROTOCOL_ID: u8 = 0x01;
