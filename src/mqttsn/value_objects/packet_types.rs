use std::fmt::Debug;
use std::clone::Clone;
use std::string::ToString;
use std::cmp::PartialEq;

// Define a PacketType enum
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MqttSnPacketType{
    Advertise,
    Searchgw,
    Gwinfo,
    Connect,
    Connack,
    Willtopicreq,
    Willtopic,
    Willmsgreq,
    Willmsg,
    Register,
    Regack,
    Publish,
    Puback,
    Pubcomp,
    Pubrec,
    Pubrel,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
    Willtopicupd,
    Willtopicresp,
    Willmsgupd,
    Willmsgresp,
    Encapsulated,
    Frwdencap,
    Unknown,
}

impl From<u8> for MqttSnPacketType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => MqttSnPacketType::Advertise,
            0x01 => MqttSnPacketType::Searchgw,
            0x02 => MqttSnPacketType::Gwinfo,
            0x04 => MqttSnPacketType::Connect,
            0x05 => MqttSnPacketType::Connack,
            0x06 => MqttSnPacketType::Willtopicreq,
            0x07 => MqttSnPacketType::Willtopic,
            0x08 => MqttSnPacketType::Willmsgreq,
            0x09 => MqttSnPacketType::Willmsg,
            0x0A => MqttSnPacketType::Register,
            0x0B => MqttSnPacketType::Regack,
            0x0C => MqttSnPacketType::Publish,
            0x0D => MqttSnPacketType::Puback,
            0x0E => MqttSnPacketType::Pubcomp,
            0x0F => MqttSnPacketType::Pubrec,
            0x10 => MqttSnPacketType::Pubrel,
            0x12 => MqttSnPacketType::Subscribe,
            0x13 => MqttSnPacketType::Suback,
            0x14 => MqttSnPacketType::Unsubscribe,
            0x15 => MqttSnPacketType::Unsuback,
            0x16 => MqttSnPacketType::Pingreq,
            0x17 => MqttSnPacketType::Pingresp,
            0x18 => MqttSnPacketType::Disconnect,
            0x1A => MqttSnPacketType::Willtopicupd,
            0x1B => MqttSnPacketType::Willtopicresp,
            0x1C => MqttSnPacketType::Willmsgupd,
            0x1D => MqttSnPacketType::Willmsgresp,
            0x1E => MqttSnPacketType::Encapsulated,
            0x1F => MqttSnPacketType::Frwdencap,
            _ => MqttSnPacketType::Unknown,
        }
    }
}

impl ToString for MqttSnPacketType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<MqttSnPacketType> for u8 {
    fn from(packet_type: MqttSnPacketType) -> Self {
        match packet_type {
            MqttSnPacketType::Advertise => 0x00,
            MqttSnPacketType::Searchgw => 0x01,
            MqttSnPacketType::Gwinfo => 0x02,
            MqttSnPacketType::Connect => 0x04,
            MqttSnPacketType::Connack => 0x05,
            MqttSnPacketType::Willtopicreq => 0x06,
            MqttSnPacketType::Willtopic => 0x07,
            MqttSnPacketType::Willmsgreq => 0x08,
            MqttSnPacketType::Willmsg => 0x09,
            MqttSnPacketType::Register => 0x0A,
            MqttSnPacketType::Regack => 0x0B,
            MqttSnPacketType::Publish => 0x0C,
            MqttSnPacketType::Puback => 0x0D,
            MqttSnPacketType::Pubcomp => 0x0E,
            MqttSnPacketType::Pubrec => 0x0F,
            MqttSnPacketType::Pubrel => 0x10,
            MqttSnPacketType::Subscribe => 0x12,
            MqttSnPacketType::Suback => 0x13,
            MqttSnPacketType::Unsubscribe => 0x14,
            MqttSnPacketType::Unsuback => 0x15,
            MqttSnPacketType::Pingreq => 0x16,
            MqttSnPacketType::Pingresp => 0x17,
            MqttSnPacketType::Disconnect => 0x18,
            MqttSnPacketType::Willtopicupd => 0x1A,
            MqttSnPacketType::Willtopicresp => 0x1B,
            MqttSnPacketType::Willmsgupd => 0x1C,
            MqttSnPacketType::Willmsgresp => 0x1D,
            MqttSnPacketType::Encapsulated => 0x1E,
            MqttSnPacketType::Frwdencap => 0x1F,
            MqttSnPacketType::Unknown => 0xFF,
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(MqttSnPacketType::from(0x00), MqttSnPacketType::Advertise);
        assert_eq!(MqttSnPacketType::from(0x01), MqttSnPacketType::Searchgw);
        assert_eq!(MqttSnPacketType::from(0x02), MqttSnPacketType::Gwinfo);
        assert_eq!(MqttSnPacketType::from(0x04), MqttSnPacketType::Connect);
        assert_eq!(MqttSnPacketType::from(0x05), MqttSnPacketType::Connack);
        assert_eq!(MqttSnPacketType::from(0x06), MqttSnPacketType::Willtopicreq);
        assert_eq!(MqttSnPacketType::from(0x07), MqttSnPacketType::Willtopic);
        assert_eq!(MqttSnPacketType::from(0x08), MqttSnPacketType::Willmsgreq);
        assert_eq!(MqttSnPacketType::from(0x09), MqttSnPacketType::Willmsg);
        assert_eq!(MqttSnPacketType::from(0x0A), MqttSnPacketType::Register);
        assert_eq!(MqttSnPacketType::from(0x0B), MqttSnPacketType::Regack);
        assert_eq!(MqttSnPacketType::from(0x0C), MqttSnPacketType::Publish);
        assert_eq!(MqttSnPacketType::from(0x0D), MqttSnPacketType::Puback);
        assert_eq!(MqttSnPacketType::from(0x0E), MqttSnPacketType::Pubcomp);
        assert_eq!(MqttSnPacketType::from(0x0F), MqttSnPacketType::Pubrec);
        assert_eq!(MqttSnPacketType::from(0x10), MqttSnPacketType::Pubrel);
        assert_eq!(MqttSnPacketType::from(0x12), MqttSnPacketType::Subscribe);
        assert_eq!(MqttSnPacketType::from(0x13), MqttSnPacketType::Suback);
        assert_eq!(MqttSnPacketType::from(0x14), MqttSnPacketType::Unsubscribe);
        assert_eq!(MqttSnPacketType::from(0x15), MqttSnPacketType::Unsuback);
        assert_eq!(MqttSnPacketType::from(0x16), MqttSnPacketType::Pingreq);
        assert_eq!(MqttSnPacketType::from(0x17), MqttSnPacketType::Pingresp);
        assert_eq!(MqttSnPacketType::from(0x18), MqttSnPacketType::Disconnect);
        assert_eq!(MqttSnPacketType::from(0x1A), MqttSnPacketType::Willtopicupd);
        assert_eq!(MqttSnPacketType::from(0x1B), MqttSnPacketType::Willtopicresp);
        assert_eq!(MqttSnPacketType::from(0x1C), MqttSnPacketType::Willmsgupd);
        assert_eq!(MqttSnPacketType::from(0x1D), MqttSnPacketType::Willmsgresp);
        assert_eq!(MqttSnPacketType::from(0x1E), MqttSnPacketType::Encapsulated);
        assert_eq!(MqttSnPacketType::from(0x1F), MqttSnPacketType::Frwdencap);
        assert_eq!(MqttSnPacketType::from(0xFF), MqttSnPacketType::Unknown);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(MqttSnPacketType::Advertise.to_string(), "Advertise");
        assert_eq!(MqttSnPacketType::Searchgw.to_string(), "Searchgw");
        assert_eq!(MqttSnPacketType::Gwinfo.to_string(), "Gwinfo");
        assert_eq!(MqttSnPacketType::Connect.to_string(), "Connect");
        assert_eq!(MqttSnPacketType::Connack.to_string(), "Connack");
        assert_eq!(MqttSnPacketType::Willtopicreq.to_string(), "Willtopicreq");
        assert_eq!(MqttSnPacketType::Willtopic.to_string(), "Willtopic");
        assert_eq!(MqttSnPacketType::Willmsgreq.to_string(), "Willmsgreq");
        assert_eq!(MqttSnPacketType::Willmsg.to_string(), "Willmsg");
        assert_eq!(MqttSnPacketType::Register.to_string(), "Register");
        assert_eq!(MqttSnPacketType::Regack.to_string(), "Regack");
        assert_eq!(MqttSnPacketType::Publish.to_string(), "Publish");
        assert_eq!(MqttSnPacketType::Puback.to_string(), "Puback");
        assert_eq!(MqttSnPacketType::Pubcomp.to_string(), "Pubcomp");
        assert_eq!(MqttSnPacketType::Pubrec.to_string(), "Pubrec");
        assert_eq!(MqttSnPacketType::Pubrel.to_string(), "Pubrel");
        assert_eq!(MqttSnPacketType::Subscribe.to_string(), "Subscribe");
        assert_eq!(MqttSnPacketType::Suback.to_string(), "Suback");
        assert_eq!(MqttSnPacketType::Unsubscribe.to_string(), "Unsubscribe");
        assert_eq!(MqttSnPacketType::Unsuback.to_string(), "Unsuback");
        assert_eq!(MqttSnPacketType::Pingreq.to_string(), "Pingreq");
        assert_eq!(MqttSnPacketType::Pingresp.to_string(), "Pingresp");
        assert_eq!(MqttSnPacketType::Disconnect.to_string(), "Disconnect");
        assert_eq!(MqttSnPacketType::Willtopicupd.to_string(), "Willtopicupd");
        assert_eq!(MqttSnPacketType::Willtopicresp.to_string(), "Willtopicresp");
        assert_eq!(MqttSnPacketType::Willmsgupd.to_string(), "Willmsgupd");
        assert_eq!(MqttSnPacketType::Willmsgresp.to_string(), "Willmsgresp");
        assert_eq!(MqttSnPacketType::Encapsulated.to_string(), "Encapsulated");
        assert_eq!(MqttSnPacketType::Frwdencap.to_string(), "Frwdencap");
        assert_eq!(MqttSnPacketType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_into_u8() {
        assert_eq!(u8::from(MqttSnPacketType::Advertise), 0x00);
        assert_eq!(u8::from(MqttSnPacketType::Searchgw), 0x01);
        assert_eq!(u8::from(MqttSnPacketType::Gwinfo), 0x02);
        assert_eq!(u8::from(MqttSnPacketType::Connect), 0x04);
        assert_eq!(u8::from(MqttSnPacketType::Connack), 0x05);
        assert_eq!(u8::from(MqttSnPacketType::Willtopicreq), 0x06);
        assert_eq!(u8::from(MqttSnPacketType::Willtopic), 0x07);
        assert_eq!(u8::from(MqttSnPacketType::Willmsgreq), 0x08);
        assert_eq!(u8::from(MqttSnPacketType::Willmsg), 0x09);
        assert_eq!(u8::from(MqttSnPacketType::Register), 0x0A);
        assert_eq!(u8::from(MqttSnPacketType::Regack), 0x0B);
        assert_eq!(u8::from(MqttSnPacketType::Publish), 0x0C);
        assert_eq!(u8::from(MqttSnPacketType::Puback), 0x0D);
        assert_eq!(u8::from(MqttSnPacketType::Pubcomp), 0x0E);
        assert_eq!(u8::from(MqttSnPacketType::Pubrec), 0x0F);
        assert_eq!(u8::from(MqttSnPacketType::Pubrel), 0x10);
        assert_eq!(u8::from(MqttSnPacketType::Subscribe), 0x12);
        assert_eq!(u8::from(MqttSnPacketType::Suback), 0x13);
        assert_eq!(u8::from(MqttSnPacketType::Unsubscribe), 0x14);
        assert_eq!(u8::from(MqttSnPacketType::Unsuback), 0x15);
        assert_eq!(u8::from(MqttSnPacketType::Pingreq), 0x16);
        assert_eq!(u8::from(MqttSnPacketType::Pingresp), 0x17);
        assert_eq!(u8::from(MqttSnPacketType::Disconnect), 0x18);
        assert_eq!(u8::from(MqttSnPacketType::Willtopicupd), 0x1A);
        assert_eq!(u8::from(MqttSnPacketType::Willtopicresp), 0x1B);
        assert_eq!(u8::from(MqttSnPacketType::Willmsgupd), 0x1C);
        assert_eq!(u8::from(MqttSnPacketType::Willmsgresp), 0x1D);
        assert_eq!(u8::from(MqttSnPacketType::Encapsulated), 0x1E);
        assert_eq!(u8::from(MqttSnPacketType::Frwdencap), 0x1F);
        assert_eq!(u8::from(MqttSnPacketType::Unknown), 0xFF);
    }
}
