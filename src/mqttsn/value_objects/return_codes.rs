
#[derive(Debug, PartialEq)]
pub enum MqttSnReturnCode {
    Accepted,
    RejectedCongestion,
    RejectedInvalidTopicId,
    RejectedNotSupported,
}


impl From<u8> for MqttSnReturnCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => MqttSnReturnCode::Accepted,
            0x01 => MqttSnReturnCode::RejectedCongestion,
            0x02 => MqttSnReturnCode::RejectedInvalidTopicId,
            0x03 => MqttSnReturnCode::RejectedNotSupported,
            _ => panic!("Invalid return code"),
        }
    }
}

impl From<MqttSnReturnCode> for u8 {
    fn from(return_code: MqttSnReturnCode) -> Self {
        match return_code {
            MqttSnReturnCode::Accepted => 0x00,
            MqttSnReturnCode::RejectedCongestion => 0x01,
            MqttSnReturnCode::RejectedInvalidTopicId => 0x02,
            MqttSnReturnCode::RejectedNotSupported => 0x03,
        }
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let return_code = MqttSnReturnCode::from(0x00);
        assert_eq!(return_code, MqttSnReturnCode::Accepted);
    }

    #[test]
    fn test_as_bytes() {
        let return_code = MqttSnReturnCode::Accepted;
        let return_code_byte: u8 = return_code.into();
        assert_eq!(return_code_byte, 0x00);
    }
}
