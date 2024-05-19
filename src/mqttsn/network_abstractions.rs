// Network abstractions for MQTT-SN
// 
// This module defines the network abstractions for MQTT-SN.
// They allow the user to define their own network layer for MQTT-SN, 
// by implementing the SensorNetwork trait. It is inspired by the
// network abstractions in the PAHO MQTT-SN client library.


use std::{net::UdpSocket, time::Duration};
// SerialPort
use serialport::{SerialPort, DataBits, FlowControl, Parity};
use std::io::prelude::*;

// SensorNetwork trait
pub trait SensorNetwork {
    fn initialize(&self);
    fn get_description(&self) -> String;
    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error>;
    fn send(&mut self, data: &[u8]) -> bool;
    fn get_timeout(&self) -> u64;
    fn close(&self);
}


// SensorNetworkType and SensorNetworkInitArgs

// An enum to represent the different types of sensor networks
pub enum SensorNetworkType {
    UDP,
    SerialPort,
}

// Another enum to represent the different types of initialization arguments
pub enum SensorNetworkInitArgs {
    UDP {
        source_address: String,
        destination_address: String,
        timeout: u64,
    },
    SerialPort {
        port_name: String,
        baud_rate: u32,
        parity: Parity,
        data_bits: DataBits,
        flow_control: FlowControl,
        timeout: u64,
    },
}

// A SensorNetwork factory function

// A function to create a sensor network based on the type
pub fn create_sensor_network(network_type: SensorNetworkType, init_args: SensorNetworkInitArgs) -> Box<dyn SensorNetwork> {
    match network_type {
        SensorNetworkType::UDP => {
            match init_args {
                SensorNetworkInitArgs::UDP { source_address, destination_address, timeout } => {
                    Box::new(UDPSensorNetwork::new(&source_address, &destination_address, timeout))
                }
                _ => panic!("Invalid initialization arguments"),
            }
        }
        SensorNetworkType::SerialPort => {
            match init_args {
                SensorNetworkInitArgs::SerialPort { port_name, baud_rate, parity, data_bits, flow_control, timeout } => {
                    Box::new(SerialPortSensorNetwork::new(port_name, baud_rate, parity, data_bits, flow_control, timeout))
                }
                _ => panic!("Invalid initialization arguments"),
            }
        }
    }
}



// UDPSensorNetwork
pub struct UDPSensorNetwork {
    source_address: String,
    destination_address: String,
    socket: UdpSocket,
    timeout: u64,
}

impl UDPSensorNetwork {
    pub fn new(
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
}

impl SensorNetwork for UDPSensorNetwork {
    fn get_timeout(&self) -> u64 {
        self.timeout
    }

    fn initialize(&self) {
        // Connect to the destination address
        self.socket.connect(&self.destination_address)
            .expect("Could not connect to destination address");

        // Set the timeout
        if self.timeout > 0 {
            self.socket.set_read_timeout(Some(std::time::Duration::from_secs(self.timeout)))
                .expect("Could not set read timeout");
        }
    }

    fn get_description(&self) -> String {
        format!("UDP Sensor Network: Source: {}, Destination: {}", self.source_address, self.destination_address)
    }

   fn send(&mut self, data: &[u8]) -> bool {
        self.socket.send(data)
            .expect("Could not send data");
        true
    }

    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = [0; 1024];
        let (size, _) = self.socket.recv_from(&mut buffer)?;
        Ok(buffer[0..size].to_vec())
    }

    fn close(&self) {
        // Nothing to do here
    }
}


// SerialPortSensorNetwork

pub struct SerialPortSensorNetwork {
    port_name: String,
    baud_rate: u32,
    parity: Parity,
    data_bits: DataBits,
    flow_control: FlowControl,
    timeout: u64,
    port: Box<dyn SerialPort>,
}

impl SerialPortSensorNetwork {
    pub fn new(
        port_name: String,
        baud_rate: u32,
        parity: Parity,
        data_bits: DataBits,
        flow_control: FlowControl,
        timeout: u64,
    ) -> SerialPortSensorNetwork {
        let mut parsed_timeout = Duration::from_secs(timeout);
        if timeout == 0 {
            // Set the timeout to the maximum value
            parsed_timeout = Duration::from_secs(86400);
        }
        let port = serialport::new(port_name.as_str(), baud_rate)
            .data_bits(data_bits)
            .parity(parity)
            .flow_control(flow_control)
            .timeout(parsed_timeout)
            .open()
            .expect("Could not open serial port");

        SerialPortSensorNetwork {
            port_name,
            baud_rate,
            parity,
            data_bits,
            flow_control,
            timeout,
            port,
        }
    }
}

impl SensorNetwork for SerialPortSensorNetwork {    
    fn get_timeout(&self) -> u64 {
        self.timeout
    }

    fn initialize(&self) {
        // Nothing to do here
    }

    fn get_description(&self) -> String {
        format!("Serial Port Sensor Network:\nPort: {}\nBaud Rate: {}\nParity: {:?}\nData Bits: {:?}\nFlow Control: {:?}\nTimeout: {}",
        self.port_name, self.baud_rate, self.parity, self.data_bits, self.flow_control, self.timeout)
    }
    fn send(&mut self, data: &[u8]) -> bool {
        self.port.write_all(data)
            .expect("Could not write data to serial port");
        true    
    }

    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error>{
        let mut buffer = [0; 1024];
        let size = self.port.read(&mut buffer)?;
        Ok(buffer[0..size].to_vec())
    }

    fn close(&self) {
       // Nothing to do here 
    }
}