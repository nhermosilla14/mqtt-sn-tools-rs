use std::net::UdpSocket;
use log::{info, error};
use crate::mqttsn::traits::sensor_net::SensorNet;
use crate::mqttsn::value_objects::sensor_net_types::SensorNetInitArgs;

pub struct UdpSensorNet {
    source_address: String,
    destination_address: String,
    timeout: u64,
    socket: Option<UdpSocket>,
}

impl UdpSensorNet {
    pub fn new(init_args: SensorNetInitArgs) -> Self {
        match init_args {
            SensorNetInitArgs::UDP {
                source_address,
                destination_address,
                timeout,
            } => UdpSensorNet {
                source_address,
                destination_address,
                timeout,
                socket: None,
            },
            _ => panic!("Invalid initialization arguments"),
        }
    }
}

impl SensorNet for UdpSensorNet {
    fn init(&mut self) {
        let socket = UdpSocket::bind(&self.source_address).expect("Could not bind to address");
        self.socket = Some(socket);
    }

    fn get_description(&self) -> String {
        format!("UDP sensor network: source_address={}, destination_address={}", self.source_address, self.destination_address)
    }

    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
        match self.socket {
            Some(ref mut socket) => {
                let mut buf = [0; 1024];
                match socket.recv(&mut buf) {
                    Ok(size) => {
                        info!("Received {} bytes", size);
                        Ok(buf[..size].to_vec())
                    }
                    Err(e) => {
                        error!("Error receiving data: {}", e);
                        Err(e)
                    }
                }
            }
            None => panic!("Socket not initialized"),
        }
    }

    fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        match self.socket {
            Some(ref mut socket) => {
                match socket.send(data) {
                    Ok(size) => {
                        info!("Sent {} bytes", size);
                        Ok(size)
                    }
                    Err(e) => {
                        error!("Error sending data: {}", e);
                        Err(e)
                    }
                }
            }
            None => panic!("Socket not initialized"),
        }
    }

    fn get_timeout(&self) -> u64 {
        self.timeout
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // New UDP sensor network
        // from 0.0.0.0 to 127.0.0.0 with a timeout of 5 seconds
        let sensor_net = UdpSensorNet::new(
            SensorNetInitArgs::UDP {
                source_address: "0.0.0.0".to_string(),
                destination_address: "127.0.0.0".to_string(),
                timeout: 5,
            },
        );

        assert_eq!(sensor_net.source_address, "0.0.0.0");
        assert_eq!(sensor_net.destination_address, "127.0.0.0");
        assert_eq!(sensor_net.timeout, 5);
    }

    #[test]
    fn test_get_description() {
        let init_args = SensorNetInitArgs::UDP {
            source_address: "0.0.0.0".to_string(),
            destination_address: "127.0.0.1".to_string(),
            timeout: 5,
        };
        let sensor_net = UdpSensorNet::new(init_args);
        assert_eq!(
            sensor_net.get_description(),
            "UDP sensor network: source_address=0.0.0.0, destination_address=127.0.0.1"
        );
    }
}
