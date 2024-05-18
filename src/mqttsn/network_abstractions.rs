// Network abstractions for MQTT-SN
// 
// This module defines the network abstractions for MQTT-SN.
// They allow the user to define their own network layer for MQTT-SN, 
// by implementing the SensorNetwork trait. It is inspired by the
// network abstractions in the PAHO MQTT-SN client library.

// For now, only use UDP

use std::net::UdpSocket;
// SerialPort
use serialport::SerialPort;
use std::io::prelude::*;
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

    let timeout = std::time::Duration::from_secs(settings.timeout);
    socket.set_read_timeout(Some(timeout))
        .expect("Could not set read timeout");

    socket
}

pub trait SensorNetwork {
    fn new(source_address: &str, destination_address: &str, timeout: u64) -> Self;
    fn initialize(&self);
    fn get_description(&self) -> String;
    fn receive(&mut self) -> Vec<u8>;
    fn send(&mut self, data: &[u8]) -> bool;
    fn get_timeout(&self) -> u64;
    fn close(&self);
}


// SensorNetworks

pub struct UDPSensorNetwork {
    source_address: String,
    destination_address: String,
    socket: UdpSocket,
    timeout: u64,
}

impl SensorNetwork for UDPSensorNetwork {
    fn new(
        source_address: &str,
        destination_address: &str,
        timeout: u64,
    ) -> UDPSensorNetwork {
        UDPSensorNetwork {
            source_address: String::from(source_address),
            destination_address: String::from(destination_address),
            socket: UdpSocket::bind(source_address)
                .expect("Could not bind to address"),
            timeout,
        }
    }

    fn get_timeout(&self) -> u64 {
        self.timeout
    }

    fn initialize(&self) {
        // Connect to the destination address
        self.socket.connect(&self.destination_address)
            .expect("Could not connect to destination address");

        // Set the timeout
        self.socket.set_read_timeout(Some(std::time::Duration::from_secs(self.timeout)))
            .expect("Could not set read timeout");

        // Set the socket to non-blocking
        self.socket.set_nonblocking(true)
            .expect("Could not set socket to non-blocking");
    }

    fn get_description(&self) -> String {
        format!("UDP Sensor Network: Source: {}, Destination: {}", self.source_address, self.destination_address)
    }

   fn send(&mut self, data: &[u8]) -> bool {
        self.socket.send(data)
            .expect("Could not send data");
        true
    }

    fn receive(&mut self) -> Vec<u8> {
        let mut buffer = [0; 1024];
        let (size, _) = self.socket.recv_from(&mut buffer)
            .expect("Could not receive data");
        buffer[0..size].to_vec()
    }

    fn close(&self) {
        // Nothing to do here
    }
}


// SerialPortSensorNetwork

pub struct SerialPortSensorNetwork {
    source_address: String, // Baudrate
    destination_address: String, // Serial port name
    timeout: u64,
    port: Box<dyn SerialPort>,
}

impl SensorNetwork for SerialPortSensorNetwork {
    fn new(
        source_address: &str,
        destination_address: &str,
        timeout: u64,
    ) -> SerialPortSensorNetwork {
        let baudrate = source_address.parse::<u32>().expect("Could not parse baudrate");
        let port = serialport::new(destination_address, baudrate)
            .timeout(std::time::Duration::from_secs(timeout))
            .open()
            .expect("Could not open serial port");

        SerialPortSensorNetwork {
            source_address: String::from(source_address),
            destination_address: String::from(destination_address),
            timeout,
            port,
        }
    }

    fn get_timeout(&self) -> u64 {
        self.timeout
    }

    fn initialize(&self) {
        // Nothing to do here
    }

    fn get_description(&self) -> String {
        format!("Serial Port Sensor Network: Source: {}, Destination: {}", self.source_address, self.destination_address)
    }

    fn send(&mut self, data: &[u8]) -> bool {
        self.port.write_all(data)
            .expect("Could not write data to serial port");
        true    
    }

    fn receive(&mut self) -> Vec<u8> {
        let mut buffer = [0; 1024];
        let size = self.port.read(&mut buffer)
            .expect("Could not read data from serial port");
        buffer[0..size].to_vec()
    }

    fn close(&self) {
       // Nothing to do here 
    }
}