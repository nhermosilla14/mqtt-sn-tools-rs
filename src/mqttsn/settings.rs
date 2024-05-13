
use std::collections::HashMap;

use crate::mqttsn::constants::{
    MQTT_SN_DEFAULT_PORT,
    MQTT_SN_DEFAULT_KEEP_ALIVE,
    MQTT_SN_DEFAULT_TIMEOUT,
    MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH,
    MQTT_SN_MAX_CLIENT_ID_LENGTH
};



// Define a struct to hold the settings

#[derive(Debug)]
pub struct Settings {
    pub mqtt_sn_host: String,
    pub mqtt_sn_port: u16,
    pub keep_alive: u16,
    pub timeout: u64,
    pub sleep_duration: u64,
    pub wireless_node_id: u16,
    pub qos: i8,
    pub source_port: u16,
    pub topic_id: u16,
    pub topic_id_type: u8,
    pub retain: bool,
    pub one_message_per_line: bool,
    pub client_id: String,
    pub forwarder_encapsulation: bool,
    pub debug_level: u8,
    pub file: String,
    pub message: String,
    pub null_message: bool,
    pub read_stdin: bool,
    pub topic: String,
    pub verbose: bool,
    pub topic_map: HashMap<u16, String>,
}


// Define a function to set the default values
pub fn default_settings() -> Settings {
    let client_id_prefix = "mqtt-sn-tools-rs-"; // Append process id
    let mut client_id = format!("{}{}", client_id_prefix, std::process::id());
    if client_id.len() > MQTT_SN_MAX_CLIENT_ID_LENGTH {
        // Truncate the client id
        client_id = client_id[0..MQTT_SN_MAX_CLIENT_ID_LENGTH].to_string();
    }

    Settings {
        mqtt_sn_host: String::from("127.0.0.1"),
        mqtt_sn_port: MQTT_SN_DEFAULT_PORT,
        keep_alive: MQTT_SN_DEFAULT_KEEP_ALIVE,
        timeout: MQTT_SN_DEFAULT_TIMEOUT,
        sleep_duration: 0,
        wireless_node_id: 0,
        qos: 0,
        source_port: 0,
        topic_id: 0,
        topic_id_type: 0,
        retain: false,
        one_message_per_line: false,
        client_id: client_id,
        forwarder_encapsulation: false,
        debug_level: 0,
        file: String::from(""),
        message: String::from(""),
        null_message: false,
        read_stdin: false,
        verbose: false,
        topic: String::from(""),
        topic_map: HashMap::new(),
    }
}

thread_local! {
    static MESSAGE_ID: std::cell::Cell<u16> = std::cell::Cell::new(0);
    static TOPIC_ID: std::cell::Cell<u16> = std::cell::Cell::new(0);
    static WIRELESS_NODE_ID: std::cell::Cell<[u8; MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH]> = std::cell::Cell::new([0; MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH]);
}

pub fn get_next_message_id() -> u16 {
    let current_id = MESSAGE_ID.with(|cell| cell.get());
    let next_id = current_id + 1;
    MESSAGE_ID.with(|cell| cell.set(next_id));
    next_id
}

pub fn reset_message_id() {
    MESSAGE_ID.with(|cell| cell.set(0));
}

pub fn get_topic_id() -> u16 {
    let current_topic_id: u16 = TOPIC_ID.with(|cell| cell.get());
    current_topic_id
}

pub fn set_topic_id(topic_id: u16) {
    TOPIC_ID.with(|cell| cell.set(topic_id));
}

pub fn set_wireless_node_id(wireless_node_id: Vec<u8>) {
    WIRELESS_NODE_ID.with(|cell| {
        let mut wireless_node_id_array = [0; MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH];
        for (i, byte) in wireless_node_id.iter().enumerate() {
            wireless_node_id_array[i] = *byte;
            if i == MQTT_SN_MAX_WIRELESS_NODE_ID_LENGTH - 1 {
                break;
            }
        }
        cell.set(wireless_node_id_array);
    });
}

pub fn get_wireless_node_id() -> Vec<u8> {
    let wireless_node_id = WIRELESS_NODE_ID.with(|cell| cell.get());
    wireless_node_id.to_vec()
}
