// This module defines the logic for sending and receiving MQTT-SN packets.

use log::{debug, error, info, warn, LevelFilter};

use crate::mqttsn::constants::{
    //Message types
    // MQTT_SN_ADVERTISE,
    MQTT_SN_CONNACK,
    MQTT_SN_CONNECT,
    MQTT_SN_DISCONNECT,
    MQTT_SN_FLAG_CLEAN,
    MQTT_SN_FLAG_QOS_0,
    MQTT_SN_FLAG_QOS_1,
    MQTT_SN_FLAG_QOS_2,
    MQTT_SN_FLAG_QOS_N1,
    MQTT_SN_FLAG_RETAIN,
    MQTT_SN_FRWDENCAP,
    MQTT_SN_MAX_CLIENT_ID_LENGTH,
    MQTT_SN_MAX_PACKET_LENGTH,
    MQTT_SN_MAX_TOPIC_LENGTH,
    MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH,
    MQTT_SN_PINGREQ,
    MQTT_SN_PINGRESP,
    MQTT_SN_PROTOCOL_ID,
    MQTT_SN_PUBACK,
    MQTT_SN_PUBLISH,
    MQTT_SN_REGACK,
    MQTT_SN_REGISTER,
    MQTT_SN_SUBACK,
    MQTT_SN_SUBSCRIBE,
    MQTT_SN_TOPIC_TYPE_NORMAL,
    MQTT_SN_TOPIC_TYPE_PREDEFINED,
    MQTT_SN_TOPIC_TYPE_SHORT,
};

use crate::mqttsn::packet_types::{
    //Packet types
    mqtt_sn_packet_type_to_str,
    ConnackPacket,
    ConnectPacket,
    DisconnectPacket,
    Packet,
    PingreqPacket,
    PingrespPacket,
    PubackPacket,
    PublishPacket,
    RegackPacket,
    RegisterPacket,
    SubackPacket,
    SubscribePacket,
    Topic,
};

use crate::mqttsn::settings::{
    get_last_message_id, get_next_message_id, get_qos_flag, get_topic_id, reset_message_id,
    set_topic_id, set_wireless_node_id, Settings,
};

use crate::mqttsn::network_abstractions::SensorNetwork;

// Generic send and receive functions

pub fn mqtt_sn_send_packet(sensor_net: &mut dyn SensorNetwork, packet: &dyn Packet) {
    // Use the length to get the bytes
    let packet_bytes = packet.as_bytes();
    let packet_length = packet_bytes[0];
    let safe_buffer: &[u8] = &packet_bytes[0..packet_length as usize];
    // Print the packet, each byte as hex
    let mut hex_buffer = String::new();
    for byte in safe_buffer {
        hex_buffer.push_str(&format!("{:02X} ", byte));
    }
    // Strip final space
    hex_buffer.pop();
    debug!("Sending packet: {:?}", hex_buffer);

    sensor_net.send(safe_buffer);
}

pub fn mqtt_sn_print_publish_packet(packet: &PublishPacket, settings: &Settings) {
    if settings.verbose {
        println!("Publish packet: {:?}", packet);
    }
}

pub fn mqtt_sn_validate_packet(buffer: &[u8], settings: &Settings) -> Option<Box<dyn Packet>> {
    // Check valid packet length
    let length = buffer[0] as usize;
    let inner_length = buffer[buffer[0] as usize] as usize;
    let total_length = length + inner_length;
    let packet_type = buffer[1];

    if length == 0 {
        error!("Invalid packet length: {}", length);
        return None;
    }

    if length == 1 {
        error!("Valid, but unsupported packet length format.");
        return None;
    }

    // Forwarder encapsulation enabled
    // If FWDEncap is enabled, every packet should be a FWDEncap packet
    if settings.forwarder_encapsulation && packet_type != MQTT_SN_FRWDENCAP {
        error!("Forwarder encapsulation is enabled, but received a non-FWDEncap packet");
        return None;
    }

    // Check length if FWDEncap is enabled
    if packet_type == MQTT_SN_FRWDENCAP && length < 4 {
        error!("Invalid packet length for FWDEncap: {}", length);
        return None;
    }

    if packet_type == MQTT_SN_FRWDENCAP {
        if buffer.len() < total_length {
            error!(
                "Received only {} bytes, but expected {} bytes",
                buffer.len(),
                total_length
            );
            return None;
        }
    }
    // Return the packet
    mqtt_sn_rebuild_packet(&buffer.to_vec())
}

pub fn mqtt_sn_rebuild_packet(buffer: &Vec<u8>) -> Option<Box<dyn Packet>> {
    // Read the message type
    let msg_type = buffer[1];

    // Create a packet based on the message type
    let packet: Box<dyn Packet> = match msg_type {
        // All supported message types in order
        //MQTT_SN_ADVERTISE => Box::new(AdvertisePacket::from_bytes(buffer)),
        //MQTT_SN_SEARCHGW => Box::new(SearchgwPacket::from_bytes(buffer)),
        //MQTT_SN_GWINFO => Box::new(GwinfoPacket::from_bytes(buffer)),
        MQTT_SN_CONNECT => Box::new(ConnectPacket::from_bytes(buffer)),
        MQTT_SN_CONNACK => Box::new(ConnackPacket::from_bytes(buffer)),
        //MQTT_SN_WILLTOPICREQ => Box::new(WilltopicreqPacket::from_bytes(buffer)),
        //MQTT_SN_WILLTOPIC => Box::new(WilltopicPacket::from_bytes(buffer)),
        //MQTT_SN_WILLMSGREQ => Box::new(WillmsgreqPacket::from_bytes(buffer)),
        //MQTT_SN_WILLMSG => Box::new(WillmsgPacket::from_bytes(buffer)),
        MQTT_SN_REGISTER => Box::new(RegisterPacket::from_bytes(buffer)),
        MQTT_SN_REGACK => Box::new(RegackPacket::from_bytes(buffer)),
        MQTT_SN_PUBLISH => Box::new(PublishPacket::from_bytes(buffer)),
        MQTT_SN_PUBACK => Box::new(PubackPacket::from_bytes(buffer)),
        //MQTT_SN_PUBCOMP => Box::new(PubcompPacket::from_bytes(buffer)),
        //MQTT_SN_PUBREC => Box::new(PubrecPacket::from_bytes(buffer)),
        //MQTT_SN_PUBREL => Box::new(PubrelPacket::from_bytes(buffer)),
        MQTT_SN_SUBSCRIBE => Box::new(SubscribePacket::from_bytes(buffer)),
        MQTT_SN_SUBACK => Box::new(SubackPacket::from_bytes(buffer)),
        //MQTT_SN_UNSUBSCRIBE => Box::new(UnsubscribePacket::from_bytes(buffer)),
        //MQTT_SN_UNSUBACK => Box::new(UnsubackPacket::from_bytes(buffer)),
        MQTT_SN_PINGREQ => Box::new(PingreqPacket::from_bytes(buffer)),
        MQTT_SN_PINGRESP => Box::new(PingrespPacket::from_bytes(buffer)),
        MQTT_SN_DISCONNECT => Box::new(DisconnectPacket::from_bytes(buffer)),
        //MQTT_SN_WILLTOPICUPD => Box::new(WilltopicupdPacket::from_bytes(buffer)),
        //MQTT_SN_WILLTOPICRESP => Box::new(WilltopicrespPacket::from_bytes(buffer)),
        //MQTT_SN_WILLMSGUPD => Box::new(WillmsgupdPacket::from_bytes(buffer)),
        //MQTT_SN_WILLMSGRESP => Box::new(WillmsgrespPacket::from_bytes(buffer)),
        //MQTT_SN_FRWDENCAP => Box::new(FrwdencapPacket::from_bytes(buffer)),
        _ => return None,
    };

    Some(packet)
}

pub fn mqtt_sn_wait_for(
    sensor_net: &mut dyn SensorNetwork,
    packet_type: u8,
    settings: &Settings,
) -> Option<Box<dyn Packet>> {
    // Save current time to calculate next keep alive
    let start = std::time::Instant::now();
    let mut last_transmission = start.clone();
    debug!("Start waiting for {} packet", mqtt_sn_packet_type_to_str(packet_type));

    loop {
        if settings.keep_alive > 0
            && last_transmission.elapsed().as_secs() >= settings.keep_alive as u64
        {
            // Send a PINGREQ packet
            debug!("Sending PINGREQ packet");
            mqtt_sn_send_pingreq(sensor_net);
            last_transmission = std::time::Instant::now();
        }
        // Receive a packet
        let packet = mqtt_sn_receive_packet(sensor_net);

        let safe_packet: Box<dyn Packet> = match packet {
            Some(packet) => packet,
            None => {
                warn!("Network timeout reached while waiting for packet");
                warn!("Retrying...");
                continue;
            }
        };

        if safe_packet.msg_type() == packet_type {
            debug!("Received expected packet: {:?}", safe_packet);
            return Some(safe_packet);
        } else if safe_packet.msg_type() == MQTT_SN_DISCONNECT {
            let disconnect = safe_packet.as_disconnect().unwrap();
            error!("Received DISCONNECT packet from gateway: {:?}", disconnect);
            // Exit and return -1
            std::process::exit(-1);
        } else {
            warn!(
                "Was expecting {} packet but received {}",
                mqtt_sn_packet_type_to_str(packet_type),
                mqtt_sn_packet_type_to_str(safe_packet.msg_type())
            );
        }

        // Check if the timeout has been reached
        if settings.timeout > 0 && start.elapsed().as_secs() >= settings.timeout {
            println!("Timeout reached while waiting for packet");
            println!("Timeout: {}", settings.timeout);
            break;
        }
    }
    None
}

pub fn mqtt_receive_frwdencap_packet(sensor_net: &mut dyn SensorNetwork, settings: &Settings) -> Option<Box<dyn Packet>> {
    // Create a buffer to hold the data, with a maximun size given by:
    // MQTT_SN_MAX_PACKET_LENGTH
    // MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH
    // + 4 (for the protocol overhead)
    const MAX_SIZE: usize = MQTT_SN_MAX_PACKET_LENGTH + MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH + 4;
    let mut buffer: Vec<u8>;

    debug!("Waiting to receive packet...");

    // Read the packet into the buffer safely
    let read_result = sensor_net.receive();
    match read_result {
        Ok(data) => {
            buffer = data;
        }
        Err(e) => {
            error!("Failed to read from sensor_net: {:?}", e);
            return None;
        }
    } 

    let mut bytes_read = buffer[0] as usize;

    if bytes_read > MAX_SIZE as usize {
        error!("Received packet is too long: {}", bytes_read);
        warn!("Truncating packet to {} bytes", MAX_SIZE);
        bytes_read = MAX_SIZE as usize;
        buffer.resize(bytes_read, 0);
    }

    debug!("Received {} bytes", bytes_read);

    if bytes_read == 0 {
        panic!("Failed to read from sensor_net");
    }

    let log_level = log::max_level();
    if log_level >= LevelFilter::Debug {
        // Check if the packet is a forwarder encapsulation packet
        if buffer[1] == MQTT_SN_FRWDENCAP {
            debug!(
                "Received {} bytes from sensor_net. Packet type {} -> {}",
                bytes_read,
                mqtt_sn_packet_type_to_str(buffer[2]),
                mqtt_sn_packet_type_to_str(buffer[3]),
            );
        } else {
            debug!(
                "Received {} bytes from sensor_net. Packet type {}",
                bytes_read,
                mqtt_sn_packet_type_to_str(buffer[1]),
            );
        }
    }

    // Validate the packet
    let generic_packet = mqtt_sn_validate_packet(&buffer, settings);
    if generic_packet.is_none() {
        error!("Failed to validate packet");
        return None;
    } else {
        debug!("Packet validated");
    }

    // Add a null terminator to the buffer
    buffer[bytes_read] = 0;

    if buffer[1] == MQTT_SN_FRWDENCAP {
        // Forwarder encapsulation packet;
        // Rebuild the packet
        let packet = generic_packet.unwrap();
        let frwencap_packet = packet.as_frwdencap().unwrap();
        set_wireless_node_id(frwencap_packet.wireless_node_id.clone());
        let inner_packet_data = frwencap_packet.inner_packet.clone();
        let inner_packet = mqtt_sn_rebuild_packet(&inner_packet_data);
        // Return the inner packet
        inner_packet
    } else {
        // Regular packet
        mqtt_sn_rebuild_packet(&buffer.to_vec())
    }
}

pub fn mqtt_sn_receive_packet(sensor_net: &mut dyn SensorNetwork) -> Option<Box<dyn Packet>> {
    // Create a buffer to hold the packet data
    let mut buffer: Vec<u8>;
    info!("Waiting to receive packet...");

    // Read the packet into the buffer safely
    let read_result = sensor_net.receive();

    match read_result {
        Ok(data) => {
            buffer = data;
        }
        Err(e) => {
            error!("Failed to read from sensor_net: {:?}", e);
            return None;
        }
    }    

    // Check the number of bytes read
    let mut bytes_read = buffer[0] as usize;

    if bytes_read > MQTT_SN_MAX_PACKET_LENGTH as usize {
        error!("Received packet is too long: {}", bytes_read);
        warn!("Truncating packet to {} bytes", MQTT_SN_MAX_PACKET_LENGTH);
        bytes_read = MQTT_SN_MAX_PACKET_LENGTH as usize;
        buffer.resize(bytes_read, 0);
    }

    debug!("Received {} bytes", bytes_read);

    if bytes_read == 0 {
        error!("Failed to read from sensor_net");
        return None;
    }

    // Rebuild the packet
    let packet = mqtt_sn_rebuild_packet(&buffer.to_vec());
    // Return the packet
    packet
}

// Specific send and receive functions

pub fn mqtt_sn_send_connect(sensor_net: &mut dyn SensorNetwork, settings: &Settings, clean_session: bool) {
    // Check client id length
    if settings.client_id.len() > MQTT_SN_MAX_CLIENT_ID_LENGTH {
        panic!(
            "Client ID is too long. Maximum length is {}",
            MQTT_SN_MAX_CLIENT_ID_LENGTH
        );
    }

    let msg_type = MQTT_SN_CONNECT;
    let flags = if clean_session { MQTT_SN_FLAG_CLEAN } else { 0 };
    let protocol_id = MQTT_SN_PROTOCOL_ID;
    let duration = settings.keep_alive.to_be();

    // Copy the client ID into the packet
    let client_id = settings.client_id.as_bytes().to_vec();

    // Get the length of the packet
    let length = 0x06 + settings.client_id.len() as u8;

    // Assemble the packet
    let packet = ConnectPacket {
        length,
        msg_type,
        flags,
        protocol_id,
        duration,
        client_id,
    };

    info!("Sending CONNECT {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}

pub fn mqtt_sn_receive_connack(sensor_net: &mut dyn SensorNetwork, settings: &Settings) {
    mqtt_sn_wait_for(sensor_net, MQTT_SN_CONNACK, settings);
}

pub fn mqtt_sn_send_register(sensor_net: &mut dyn SensorNetwork, settings: &Settings) {
    // Check topic name length
    if settings.topic.len() > MQTT_SN_MAX_TOPIC_LENGTH {
        panic!(
            "Topic name is too long. Maximum length is {}",
            MQTT_SN_MAX_TOPIC_LENGTH
        );
    }

    let msg_type = MQTT_SN_REGISTER;
    let topic_id = 0; // Not used
    let message_id = get_next_message_id();

    // Copy the topic name into the packet
    let topic_name = settings.topic.as_bytes().to_vec();

    // Get the packet length
    let length = 0x06 + settings.topic.len() as u8;

    // Assemble the packet
    let packet = RegisterPacket {
        length,
        msg_type,
        topic_id,
        message_id,
        topic_name,
    };

    info!("Sending REGISTER packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}

pub fn mqtt_sn_receive_regack(sensor_net: &mut dyn SensorNetwork, settings: &Settings) -> RegackPacket {
    let packet = mqtt_sn_wait_for(sensor_net, MQTT_SN_REGACK, settings);

    if let Some(regack) = packet.unwrap().as_regack() {
        debug!("Updated topic ID: {}", regack.topic_id);
        let reordered = regack.topic_id.to_be();
        set_topic_id(reordered);
        regack.clone()
    } else {
        panic!("Received packet is not a REGACK packet");
    }
}

pub fn mqtt_sn_send_subscribe_topic_name(sensor_net: &mut dyn SensorNetwork, settings: &Settings, topic: &str) {
    // Check topic name length
    if topic.len() > MQTT_SN_MAX_TOPIC_LENGTH {
        panic!(
            "Topic name is too long. Maximum length is {}",
            MQTT_SN_MAX_TOPIC_LENGTH
        );
    }

    let msg_type = MQTT_SN_SUBSCRIBE;
    let message_id = get_next_message_id();
    let mut flags = 0;
    flags |= get_qos_flag(settings.qos);

    // Copy the topic name into the packet
    let topic_name: Topic = Topic::TopicName(topic.as_bytes().to_vec());

    let length = 5 + topic.len() as u8;
    // Get the packet length
    if topic.len() == 2 {
        // Short topic name
        flags |= MQTT_SN_TOPIC_TYPE_SHORT;
    } else {
        // Normal topic name
        flags |= MQTT_SN_TOPIC_TYPE_NORMAL;
    }

    // Assemble the packet
    let packet = SubscribePacket {
        length,
        msg_type,
        flags,
        message_id,
        topic: topic_name,
    };

    info!("Sending SUBSCRIBE packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}

pub fn mqtt_sn_send_subscribe_topic_id(sensor_net: &mut dyn SensorNetwork, settings: &Settings, topic_id: u16) {
    let msg_type = MQTT_SN_SUBSCRIBE;
    let message_id = get_next_message_id();
    let mut flags = 0;
    flags |= MQTT_SN_TOPIC_TYPE_PREDEFINED;
    flags |= get_qos_flag(settings.qos);

    // Copy the topic ID into the packet
    let topic: Topic = Topic::TopicId(topic_id);

    // Get the packet length
    let length = 0x05 + 2;

    // Assemble the packet
    let packet = SubscribePacket {
        length,
        msg_type,
        flags,
        message_id,
        topic,
    };

    info!("Sending SUBSCRIBE packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}

pub fn mqtt_sn_receive_suback(sensor_net: &mut dyn SensorNetwork, settings: &Settings) -> u16 {
    let packet = mqtt_sn_wait_for(sensor_net, MQTT_SN_SUBACK, settings);

    if let Some(suback) = packet.unwrap().as_suback() {
        debug!("Received SUBACK packet: {:?}", suback);
        // Check the returned code
        if suback.return_code != 0 {
            error!("SUBACK failed with return code: {}", suback.return_code);
            return 0;
        } else {
            // Check message ID
            if suback.message_id != get_last_message_id() {
                error!(
                    "Received SUBACK with unexpected message ID: {}",
                    suback.message_id
                );
                return 0;
            } else {
                info!("SUBACK successful");
                info!("Topic ID: {}", suback.topic_id);
                suback.topic_id
            }
        }
    } else {
        panic!("Received packet is not a SUBACK packet");
    }
}

pub fn mqtt_sn_send_disconnect(sensor_net: &mut dyn SensorNetwork, settings: &Settings) {
    let msg_type = MQTT_SN_DISCONNECT;
    if settings.sleep_duration == 0 {
        let length = 0x02;
        let packet = DisconnectPacket {
            length,
            msg_type,
            duration: 0,
        };
        info!("Sending DISCONNECT packet: {:?}", packet);
        mqtt_sn_send_packet(sensor_net, &packet);
    } else {
        let length: u8 = 0x04;
        let duration: u16 = settings.sleep_duration as u16;
        let packet = DisconnectPacket {
            length,
            msg_type,
            duration,
        };
        info!("Sending DISCONNECT packet: {:?}", packet);
        mqtt_sn_send_packet(sensor_net, &packet);
    }
}

pub fn mqtt_sn_receive_disconnect(sensor_net: &mut dyn SensorNetwork, settings: &Settings) {
    mqtt_sn_wait_for(sensor_net, MQTT_SN_DISCONNECT, settings);
}

pub fn mqtt_sn_send_publish(sensor_net: &mut dyn SensorNetwork, settings: &Settings, message: &str) {
    // Check message length
    const MAX_MESSAGE_LENGTH: usize = MQTT_SN_MAX_PACKET_LENGTH - 7;
    if message.len() > MAX_MESSAGE_LENGTH {
        panic!(
            "Message is too long. Maximum length is {}",
            MAX_MESSAGE_LENGTH
        );
    }

    let msg_type = MQTT_SN_PUBLISH;
    let mut flags = 0;

    if settings.retain != false {
        flags |= MQTT_SN_FLAG_RETAIN;
    }

    // Get QoS flags
    let qos_flag: u8 = match settings.qos {
        0 => MQTT_SN_FLAG_QOS_0,
        1 => MQTT_SN_FLAG_QOS_1,
        2 => MQTT_SN_FLAG_QOS_2,
        _ => MQTT_SN_FLAG_QOS_N1,
    };

    flags |= qos_flag;

    // Topic type
    let topic_id_type = settings.topic_id_type;
    flags |= topic_id_type & 0x03;

    // Topic id
    let topic_id = get_topic_id().to_be();

    // Message ID
    let mut message_id: u16 = 0;
    if settings.qos > 0 {
        // Increment the message ID
        message_id = get_next_message_id();
    } else {
        // Set the message ID to 0
        reset_message_id();
    }
    let default_message = settings.message.clone();
    let mut data = default_message.as_str();
    // Get message
    if message != "" {
        data = message;
    }

    // Get the packet length
    let length = 0x07 + data.len() as u8;

    // Assemble the packet
    let packet = PublishPacket {
        length,
        msg_type,
        flags,
        topic_id,
        message_id: message_id,
        data: data.as_bytes().to_vec(),
    };

    info!("Sending PUBLISH packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);

    if settings.qos == 1 {
        // Wait for PUBACK
        let packet = mqtt_sn_wait_for(sensor_net, MQTT_SN_PUBACK, settings);
        match packet {
            Some(real_packet) => {
                if real_packet.msg_type() == MQTT_SN_PUBACK {
                    let puback = real_packet.as_puback().unwrap();
                    info!("Received PUBACK packet: {:?}", puback);
                }
            }
            None => {
                warn!("Failed to receive PUBACK packet");
            }
        }
    }
}

pub fn mqtt_sn_receive_publish(sensor_net: &mut dyn SensorNetwork, settings: &Settings) -> Option<PublishPacket> {
    let packet = mqtt_sn_wait_for(sensor_net, MQTT_SN_PUBLISH, settings);
    match packet {
        Some(publish) => {
            if let Some(publish) = publish.as_publish() {
                mqtt_sn_print_publish_packet(&publish, settings);
                Some(publish.clone())
            } else {
                warn!("Received packet is not a PUBLISH packet");
                None
            }
        }
        None => {
            warn!("Timeout reached. Retrying...");
            None
        }
    }
}

pub fn mqtt_sn_send_puback(
    sensor_net: &mut dyn SensorNetwork,
    packet: &PublishPacket,
    return_code: u8,
) {
    let msg_type = MQTT_SN_PUBACK;
    let message_id = packet.message_id;
    let topic_id = packet.topic_id;

    let length = 0x07;

    let packet = PubackPacket {
        length,
        msg_type,
        topic_id,
        message_id,
        return_code,
    };

    info!("Sending PUBACK packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}

fn mqtt_sn_send_pingreq(sensor_net: &mut dyn SensorNetwork) {
    let msg_type = MQTT_SN_PINGREQ;
    let length = 0x02;
    let packet = PingreqPacket { length, msg_type };
    info!("Sending PINGREQ packet: {:?}", packet);
    mqtt_sn_send_packet(sensor_net, &packet);
}
