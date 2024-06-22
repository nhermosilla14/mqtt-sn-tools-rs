use crate::mqttsn::sensor_nets::UdpSensorNet;
use crate::mqttsn::traits::sensor_net::SensorNet;
use crate::mqttsn::value_objects::SensorNetType;
use crate::mqttsn::value_objects::SensorNetInitArgs;
use crate::mqttsn::sensor_nets::Serial2SensorNet;

pub fn create_sensor_net(sensor_net_type: SensorNetType, init_args: SensorNetInitArgs) -> Box<dyn SensorNet> {
    match sensor_net_type {
        SensorNetType::UDP => {
            let sensor_net = UdpSensorNet::new(init_args);
            Box::new(sensor_net)
        }
        SensorNetType::Serial2 => {
            let sensor_net = Serial2SensorNet::new(init_args);
            Box::new(sensor_net)
        }
    }
}