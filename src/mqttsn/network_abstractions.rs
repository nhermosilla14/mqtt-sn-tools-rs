// Network abstractions for MQTT-SN
// 
// This module defines the network abstractions for MQTT-SN.
// They allow the user to define their own network layer for MQTT-SN, 
// by implementing the SensorNetwork trait. It is inspired by the
// network abstractions in the PAHO MQTT-SN client library.

// For now, only use UDP

use std::net::UdpSocket;
use crate::mqttsn::settings::Settings;

// Create a connection to the MQTT-SN broker
// Uses the setting struct to get the host and port, and
// optionally the source port
// Returns a UdpSocket
pub fn mqtt_sn_create_connection(settings: &Settings) -> UdpSocket {
    let local_address = format!("0.0.0.0:{}", settings.source_port);
    let remote_address = format!("{}:{}", settings.mqtt_sn_host, settings.mqtt_sn_port);

    let socket = UdpSocket::bind(&local_address)
        .expect("Could not bind to address");

    socket.connect(&remote_address)
        .expect("Could not connect to MQTT-SN broker");

    socket
}
