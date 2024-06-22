use serial2::{Parity, CharSize, FlowControl};


#[derive(Debug, PartialEq)]
pub enum SensorNetType {
    UDP,
    Serial2,
}

impl SensorNetType {
    pub fn from_str(s: &str) -> Result<SensorNetType, String> {
        match s {
            "UDP" => Ok(SensorNetType::UDP),
            "Serial2" => Ok(SensorNetType::Serial2),
            _ => Err(format!("Invalid sensor network type: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SensorNetType::UDP => "UDP",
            SensorNetType::Serial2 => "Serial2",
        }
    }
}


pub enum SensorNetInitArgs {
    UDP {
        source_address: String,
        destination_address: String,
        timeout: u64,
    },
    Serial2 {
        port_name: String,
        baud_rate: u32,
        parity: Parity,
        data_bits: CharSize,
        flow_control: FlowControl,
        timeout: u64,
    },
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(SensorNetType::from_str("UDP").unwrap(), SensorNetType::UDP);
        assert_eq!(SensorNetType::from_str("Serial2").unwrap(), SensorNetType::Serial2);
        assert!(SensorNetType::from_str("Invalid").is_err());
    }

    #[test]
    fn test_as_str() {
        assert_eq!(SensorNetType::UDP.as_str(), "UDP");
        assert_eq!(SensorNetType::Serial2.as_str(), "Serial2");
    }

    #[test]
    fn test_sensor_net_init_args() {
        let udp_args = SensorNetInitArgs::UDP {
            source_address: "0.0.0.0".to_string(),
            destination_address: "127.0.0.1".to_string(),
            timeout: 1000,
        };
        match udp_args {
            SensorNetInitArgs::UDP {
                source_address,
                destination_address,
                timeout,
            } => {
                assert_eq!(source_address, "0.0.0.0");
                assert_eq!(destination_address, "127.0.0.1");
                assert_eq!(timeout, 1000);
            }
            _ => panic!("Invalid sensor network initialization arguments"),
        }

        let serial2_args = SensorNetInitArgs::Serial2 {
            port_name: "COM1".to_string(),
            baud_rate: 9600,
            parity: serial2::Parity::None,
            data_bits: serial2::CharSize::Bits8,
            flow_control: serial2::FlowControl::None,
            timeout: 1000,
        };

        match serial2_args {
            SensorNetInitArgs::Serial2 {
                port_name,
                baud_rate,
                parity,
                data_bits,
                flow_control,
                timeout,
            } => {
                assert_eq!(port_name, "COM1");
                assert_eq!(baud_rate, 9600);
                assert_eq!(parity, serial2::Parity::None);
                assert_eq!(data_bits, serial2::CharSize::Bits8);
                assert_eq!(flow_control, serial2::FlowControl::None);
                assert_eq!(timeout, 1000);
            }
            _ => panic!("Invalid sensor network initialization arguments"),
        }
    }
}