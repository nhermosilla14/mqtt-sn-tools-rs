extern crate mqtt_sn_tools_rs;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::net::UdpSocket;
use log::{
    warn,
    error,
    debug,
    LevelFilter
};
use env_logger::Builder;

use mqtt_sn_tools_rs::mqttsn::constants::{
    MQTT_SN_MAX_PAYLOAD_LENGTH, 
    MQTT_SN_TOPIC_TYPE_NORMAL, 
    MQTT_SN_TOPIC_TYPE_PREDEFINED, 
    MQTT_SN_TOPIC_TYPE_SHORT
};

use mqtt_sn_tools_rs::mqttsn::settings::{
    Settings,
    default_settings,
};

use mqtt_sn_tools_rs::mqttsn::pubsub::{
    mqtt_sn_send_publish,
    mqtt_sn_send_connect,
    mqtt_sn_send_register,
    mqtt_sn_receive_connack,
    mqtt_sn_receive_disconnect,
    mqtt_sn_receive_regack,
    mqtt_sn_send_disconnect,
};

use mqtt_sn_tools_rs::mqttsn::network_abstractions::mqtt_sn_create_connection;


fn usage() {
    let defaults = default_settings();
    eprintln!("Usage: mqtt-sn-sub-rs [opts] -t <topic>\n");
    eprintln!("\n");
    eprintln!("  -1             exit after receiving a single message.");
    eprintln!("  -c             disable 'clean session' (store subscription and pending messages when client disconnects).");
    eprintln!("  -d             Increase debug level by one. -d can occur multiple times.");
    eprintln!("  -h <host>      MQTT-SN host to connect to. Defaults to '{}'.", defaults.mqtt_sn_host);
    eprintln!("  -i <clientid>  ID to use for this client. Defaults to 'mqtt-sn-tools-' with process id.");
    eprintln!("  -k <keepalive> keep alive in seconds for this client. Defaults to {}.", defaults.keep_alive);
    eprintln!("  -e <sleep>     sleep duration in seconds when disconnecting. Defaults to {}.", defaults.sleep_duration);
    eprintln!("  -p <port>      Network port to connect to. Defaults to '{}'.", defaults.mqtt_sn_port);
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
            "-h" => {
                i += 1;
                settings.host = args[i].clone();
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
                settings.port = args[i].parse::<u16>().expect("Failed to parse port.");
            },
            "-q" => {
                i += 1;
                settings.qos = args[i].parse::<i8>().expect("Failed to parse QoS.");
            },
            "-t" => {
                i += 1;
                settings.topic = args[i].clone();
            },
            "-T" => {
                i += 1;
                settings.topic_id = args[i].parse::<u16>().expect("Failed to parse topic ID.");
            },
            "--fe" => {
                settings.forwarder_encapsulation = true;
            },
            "--wlnid" => {
                i += 1;
                settings.wireless_node_id = args[i].clone();
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
    // The required arguments are topic_name or topic_id, and message or
    // file.
    if (settings.topic == "" && settings.topic_id == 0) || ((settings.message == "" && !settings.null_message) && settings.file == "") {
        error!("Missing required arguments.");
        usage();
    }

    // Check for invalid arguments
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

    // Only a message or a file can be provided
    if settings.message != "" && settings.file != "" {
        error!("Both message and file provided. Only one is allowed.");
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


// Placeholder for publish_file
fn publish_file(socket: &UdpSocket, settings: &Settings) {
    let mut message: String;
    // Open the file
    // If it is -, read from STDIN
    // Otherwise, read from the file
    let mut file: Box<dyn BufRead> = match settings.file.as_str() {
        "-" => Box::new(BufReader::new(std::io::stdin())),
        _ => Box::new(BufReader::new(File::open(settings.file.as_str()).expect("Failed to open file.")))
    };
    // Check if you are supposed to read one message per line
    // If so, do it
    // Otherwise, read the whole file
    if settings.one_message_per_line {
        for line in file.lines() {
            let line = line.unwrap();
            // Check if the line is empty
            // If so, skip it
            if line == "" {
                continue;
            }
            // Check if the line is too long
            // If so, truncate it
            if line.len() > MQTT_SN_MAX_PAYLOAD_LENGTH {
                warn!("Line too long. Truncating to {} bytes.", MQTT_SN_MAX_PAYLOAD_LENGTH);
                message  = line[..MQTT_SN_MAX_PAYLOAD_LENGTH].to_string();
            } else {
                message = line;
            }
            // Publish
            mqtt_sn_send_publish(socket, &settings, &message);
        }
    } else {
        // Read the file up to MQTT_SN_MAX_PAYLOAD_LENGTH
        let mut buffer = vec![0; MQTT_SN_MAX_PAYLOAD_LENGTH];
        let bytes_read = file.read(&mut buffer).expect("Failed to read file.");

        // Strip the buffer of any null bytes
        if bytes_read < MQTT_SN_MAX_PAYLOAD_LENGTH {
            buffer.truncate(bytes_read);
        }
        // Publish
        let message = String::from_utf8(buffer).expect("Failed to convert buffer to string.");
        // Publish
        mqtt_sn_send_publish(socket, &settings, message.as_str());
    }
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

    // Print the settings
    debug!("{:?}", settings);

    // First open a UDP socket
    let socket: UdpSocket = mqtt_sn_create_connection(&settings);

    if settings.qos >= 0 {
        // Send a CONNECT message
        mqtt_sn_send_connect(&socket, &settings, true);
        mqtt_sn_receive_connack(&socket);
        
        // Then check if the topic is a pre-defined topic ID
        if settings.topic_id != 0 {
            // Use a pre-defined topic ID
            settings.topic_id_type = MQTT_SN_TOPIC_TYPE_PREDEFINED;
        } else if settings.topic.len() == 2 {
            // Use a short topic name
            settings.topic_id_type = MQTT_SN_TOPIC_TYPE_SHORT;
            // Convert the 2 character topic name into a 2 byte topic ID
            settings.topic_id = ((settings.topic.as_bytes()[0] as u16) << 8) | (settings.topic.as_bytes()[1] as u16);
        } else if settings.qos >= 0 {
            // Send a REGISTER message
            mqtt_sn_send_register(&socket, &settings);
            mqtt_sn_receive_regack(&socket);
            settings.topic_id_type = MQTT_SN_TOPIC_TYPE_NORMAL;
        }

       // Publish the message to the topic
         if settings.file != "" {
              publish_file(&socket, &settings);
         } else {
              mqtt_sn_send_publish(&socket, &settings, ""); 
         }

         // Disconnect
         if settings.qos >= 0 {
             mqtt_sn_send_disconnect(&socket, &settings);
             mqtt_sn_receive_disconnect(&socket);
         }

    }
}