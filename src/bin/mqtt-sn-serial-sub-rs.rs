extern crate mqtt_sn_tools_rs;

use log::{
    warn,
    error,
    debug,
    LevelFilter
};

use env_logger::Builder;

use mqtt_sn_tools_rs::mqttsn::constants::{
    MQTT_SN_FLAG_QOS_MASK,
    MQTT_SN_FLAG_QOS_1,
    MQTT_SN_ACCEPTED,
};

use mqtt_sn_tools_rs::mqttsn::settings::{
    Settings,
    default_settings,
};

use mqtt_sn_tools_rs::mqttsn::pubsub::{
    mqtt_sn_connect, mqtt_sn_receive_disconnect, mqtt_sn_receive_publish, mqtt_sn_receive_suback, mqtt_sn_send_disconnect, mqtt_sn_send_puback, mqtt_sn_send_subscribe_topic_id, mqtt_sn_send_subscribe_topic_name
};

use mqtt_sn_tools_rs::mqttsn::network_abstractions::{
    SensorNetwork,
    SensorNetworkInitArgs,
    SensorNetworkType,
    create_sensor_network,
};


fn usage() {
    let defaults = default_settings();
    eprintln!("Usage: mqtt-sn-sub-rs [opts] -t <topic>\n");
    eprintln!("\n");
    eprintln!("  -1             exit after receiving a single message.");
    eprintln!("  -c             disable 'clean session' (store subscription and pending messages when client disconnects).");
    eprintln!("  -d             Increase debug level by one. -d can occur multiple times.");
    eprintln!("  -i <clientid>  ID to use for this client. Defaults to 'mqtt-sn-tools-' with process id.");
    eprintln!("  -k <keepalive> keep alive in seconds for this client. Defaults to {}.", defaults.keep_alive);
    eprintln!("  -e <sleep>     sleep duration in seconds when disconnecting. Defaults to {}.", defaults.sleep_duration);
    eprintln!("  -p <port>      Serial port to connect to. Defaults to '{}'.", defaults.serial_port);
    eprintln!("  -b <baudrate>  Baudrate for serial port. Defaults to '{}'.", defaults.baudrate);
    eprintln!("  -q <qos>       QoS level to subscribe with (0 or 1). Defaults to {}.", defaults.qos);
    eprintln!("  -t <topic>     MQTT-SN topic name to subscribe to. It may repeat multiple times.");
    eprintln!("  -T <topicid>   Pre-defined MQTT-SN topic ID to subscribe to. It may repeat multiple times.");
    eprintln!("  --fe           Enables Forwarder Encapsulation. Mqtt-sn packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.");
    eprintln!("  --wlnid        If Forwarder Encapsulation is enabled, wireless node ID for this client. Defaults to process id (truncating if necessary).");
    eprintln!("  --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to {}.", defaults.source_port);
    eprintln!("  -v             Print messages verbosely, showing the topic name.");
    eprintln!("  -V             Print messages verbosely, showing current time and the topic name.");
    std::process::exit(1);
}

fn parse_args() -> Settings{
    let args: Vec<String> = std::env::args().collect();
    let mut settings = default_settings();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-1" => {
                settings.single_message = true;
            },
            "-c" => {
                settings.clean_session = false;
            },
            "-d" => {
                settings.debug_level += 1;
            },
            "-i" => {
                i += 1;
                settings.client_id = args[i].clone();
            },
            "-k" => {
                i += 1;
                settings.keep_alive = args[i].parse::<u16>().expect("Failed to parse keep alive.");
            },
            "-e" => {
                i += 1;
                settings.sleep_duration = args[i].parse::<u64>().expect("Failed to parse sleep duration.");
            },
            "-p" => {
                i += 1;
                settings.serial_port = args[i].parse().expect("Failed to parse port.");
            },
            "-b" => {
                i += 1;
                settings.baudrate = args[i].parse::<u32>().expect("Failed to parse baudrate.");
            },
            "-q" => {
                i += 1;
                settings.qos = args[i].parse::<i8>().expect("Failed to parse QoS.");
            },
            "-t" => {
                i += 1;
                settings.topic_list.push(args[i].clone());
            },
            "-T" => {
                i += 1;
                settings.topic_id_list.push(args[i].parse::<u16>().expect("Failed to parse topic ID."));
            },
            "--fe" => {
                settings.forwarder_encapsulation = true;
            },
            "--wlnid" => {
                i += 1;
                settings.wireless_node_id = args[i].parse().unwrap();
            },
            "--cport" => {
                i += 1;
                settings.source_port = args[i].parse::<u16>().expect("Failed to parse source port.");
            },
            "-v" => {
                settings.verbose = true;
            },
            "-V" => {
                settings.verbose_time = true;
            },
            _ => {
                error!("Unknown option: {}", args[i]);
                usage();
            }
        }
        i += 1;
    }

    // Check for missing arguments
    
    // The QoS value must be 0, 1 or -1
    if settings.qos != 0 && settings.qos != 1 && settings.qos != -1 {
        error!("Invalid QoS value: {}", settings.qos);
        usage();
    }

    // Either topic or topic_id must be provided
    if settings.topic != "" && settings.topic_id != 0 {
        error!("Both topic and topic_id provided. Only one is allowed.");
        usage();
    }

    // Topic must be valid for QoS -1
    // That means either a short topic name or a pre-defined topic ID
    if (settings.qos == -1) && (settings.topic_id == 0) && (settings.topic.len() != 2) {
        error!("Either a short topic name or a pre-defined topic ID must be provided for QoS -1.");
        usage();
    }

    settings
}

fn main(){
    // Print the usage if no arguments are provided
    if std::env::args().len() == 1 {
        usage();
    }
    let mut settings = parse_args();

    // Initialize the logger
    let mut builder = Builder::from_default_env();
    // Check the log level
    let log_level: LevelFilter = match settings.debug_level {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace
    };
    builder.filter(None, log_level);
    builder.init();
    
    settings.timeout = settings.keep_alive as u64 / 2;

    // Print the settings
    debug!("{:?}", settings);
    // First open a connection
    let mut boxed_sensor_network: Box<dyn SensorNetwork> = 
        create_sensor_network(
            SensorNetworkType::SerialPort,
            SensorNetworkInitArgs::SerialPort {
                port_name: settings.serial_port.clone(),
                baud_rate: settings.baudrate,
                parity: serialport::Parity::None,
                data_bits: serialport::DataBits::Eight,
                flow_control: serialport::FlowControl::None,
                timeout: std::time::Duration::from_millis(settings.network_timeout as u64),
            }
        );

    let sensor_net = &mut *boxed_sensor_network;
    sensor_net.initialize();

    

    // Send a CONNECT message
    debug!("Sending CONNECT message");
    mqtt_sn_connect(sensor_net, &settings); 

    // Subscribe to the topics by topic name
    for topic in settings.topic_list.iter() {
        debug!("Subscribing to topic: {}", topic);
        mqtt_sn_send_subscribe_topic_name(sensor_net, &settings, topic);
        let topic_id = mqtt_sn_receive_suback(sensor_net, &settings);

        if topic_id != 0  && topic.len() > 2 {
            settings.topic_map.insert(topic_id, topic.clone());
        }
    }

    // Subscribe to the topics by topic ID
    for topic_id in settings.topic_id_list.iter() {
        debug!("Subscribing to topic ID: {}", topic_id);
        mqtt_sn_send_subscribe_topic_id(sensor_net, &settings, *topic_id);
        mqtt_sn_receive_suback(sensor_net, &settings);
    }

    loop {
        // Receive messages
        debug!("Waiting for a message");
        let unsafe_packet = mqtt_sn_receive_publish(sensor_net, &settings);

        let packet = match unsafe_packet {
            Some(packet) => packet,
            None => {
                warn!("Received an empty packet. Ignoring.");
                continue;
            }
        };

        if packet.data.len() == 0 {
            warn!("Received an empty packet. Ignoring.");
            continue;    
        }
        
        let msg_qos = packet.flags & MQTT_SN_FLAG_QOS_MASK;
        if msg_qos == MQTT_SN_FLAG_QOS_1 {
            // Send a PUBACK
            mqtt_sn_send_puback(sensor_net, &packet, MQTT_SN_ACCEPTED);
        //} else if msg_qos == MQTT_SN_FLAG_QOS_2 {
        //    // Send a PUBREC
        //    mqtt_sn_send_pubrec(sensor_net, &settings, &packet);
        }

        if settings.single_message {
            break;
        }

    }

    // Send a DISCONNECT message
    mqtt_sn_send_disconnect(sensor_net, &settings);
    debug!("Sending DISCONNECT message");
    mqtt_sn_receive_disconnect(sensor_net, &settings);
    
}
