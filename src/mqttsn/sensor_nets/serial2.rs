use serial2::{SerialPort, Settings, Parity, CharSize, FlowControl};
use crate::mqttsn::traits::sensor_net::SensorNet;
use crate::mqttsn::value_objects::SensorNetInitArgs;

use log::{info, error};

pub struct Serial2SensorNet {
    port_name: String,
    baud_rate: u32,
    parity: Parity,
    data_bits: CharSize,
    flow_control: FlowControl,
    timeout: u64,
    port: SerialPort,
}

impl SensorNet for Serial2SensorNet {
    fn init(&mut self) {
        // Nothing to do here
    }

    fn get_description(&self) -> String {
        format!("Serial2 SensorNet:\nPort: {}\nBaud rate: {}\nParity: {:?}\nData bits: {:?}\nFlow control: {:?}\nTimeout: {}",
            self.port_name, self.baud_rate, self.parity, self.data_bits, self.flow_control, self.timeout)
    }

    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
        unimplemented!()
    }

    fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        match self.port.write_all(data) {
            Ok(_) => {
                info!("Sent {} bytes", data.len());
                Ok(data.len() as usize)
            },
            Err(e) => {
                error!("Failed to send data: {}", e);
                Err(e)
            }
        }
    }

    fn get_timeout(&self) -> u64 {
        self.timeout
    }
}

impl Serial2SensorNet {
    pub fn new(init_args: SensorNetInitArgs) -> Self {
        match init_args {
            SensorNetInitArgs::Serial2 {
                port_name,
                baud_rate,
                parity,
                data_bits,
                flow_control,
                timeout,
            } => {
                let port = SerialPort::open(&port_name, |mut settings: Settings| {
                    settings.set_raw();
                    settings.set_baud_rate(baud_rate)?;
                    settings.set_parity(parity);
                    settings.set_char_size(data_bits);
                    settings.set_flow_control(flow_control);
                    Ok(settings)
                }).expect("Could not open serial port");

                Serial2SensorNet {
                    port_name,
                    baud_rate,
                    parity,
                    data_bits,
                    flow_control,
                    timeout,
                    port,
                }           
            },
            _ => panic!("Invalid initialization arguments"),
        } 
    }
}