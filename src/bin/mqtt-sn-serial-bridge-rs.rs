// First an example
//
extern crate mqtt_sn_tools_rs;

use crate::mqtt_sn_tools_rs::mqttsn::network_abstractions::{
    create_sensor_network,
    SensorNetworkInitArgs,
    SensorNetworkType,
};




fn main() {
    let mut net_args = Vec::new();
   
    net_args.push("0.0.0.0:10000".to_string()); // source_address
    net_args.push("127.0.0.1:50000".to_string()); // destination_address
    net_args.push("30".to_string()); // timeout

    print!("{:?}", net_args);
    // net_args.push(args[1].clone()); // port_name
    // net_args.push(args[2].clone()); // baud_rate
    // net_args.push(args[3].clone()); // parity
    // net_args.push(args[4].clone()); // data_bits
    // net_args.push(args[5].clone()); // flow_control
    // net_args.push(args[6].clone()); // timeout

    let mut boxed_sensor_net = create_sensor_network(SensorNetworkType::UDP, SensorNetworkInitArgs::UDP {
        source_address: net_args[0].clone(),
        destination_address: net_args[1].clone(),
        timeout: net_args[2].parse().unwrap(),
    });

    let sensor_net = &mut *boxed_sensor_net;
    sensor_net.initialize();


    // Receive data and send ir back
    loop {
        let data = sensor_net.receive();
        if data.is_ok() {
            let data = data.unwrap();
            let _ = sensor_net.send(&data);
        }
        // Otherwise do nothing
    }
}
