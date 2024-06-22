use crate::mqttsn::value_objects::packet_types::MqttSnPacketType;
use crate::mqttsn::value_objects::topic_id::TopicId;
use crate::mqttsn::value_objects::MqttSnReturnCode;



pub struct PubackPacket {
    pub msg_type: MqttSnPacketType,
    pub topic_id: TopicId,
    pub msg_id: u16,
    pub return_code: MqttSnReturnCode,
}