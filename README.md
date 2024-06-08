# mqtt-sn-tools-rs
A reimplementation of mqtt-sn-tools written in Rust.
[![Rust](https://github.com/nhermosilla14/mqtt-sn-tools-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/nhermosilla14/mqtt-sn-tools-rs/actions/workflows/rust.yml)

MQTT-SN Tools in Rust
=====================

Command line tools written in Rust for the MQTT-SN (MQTT for Sensor Networks) protocol. This set of tools aims to be a near drop-in replacement for the ones written in C by Nicholas Humfrey (https://github.com/njh/mqtt-sn-tools).


# Context

This project served me as a way to exercise and learn Rust, which I've been trying to for quite a while now. I didn't properly get started until I came across the need for MQTT-SN Tools in a Windows machine. Those tools do work just fine using Cygwin, but then again, not really a proper native solution. So I began this in order to get the tools I needed. That's why the code might be uglier than it should, so I'm quite eager to get some feedback and PRs. Issues reports are also welcome.


# Supported features

Since this aims for feature parity with the original tools, the currently supported features (and planed roadmap) is the same as them. The state of development is as follows:

## Publisher (mqtt-sn-pub-rs)

- [X] QoS 0, 1 and -1
- [X] Keep alive pings
- [X] Publishing retained messages
- [X] Publishing empty messages
- [X] Publishing to named topic (registering it first)
- [X] Clean / unclean sessions
- [X] Manual and automatic client ID generation
- [X] Pre-defined topic IDs and short topic names
- [ ] Forwarder encapsulation according to MQTT-SN Protocol Specification v1.2.

## Subscriber (mqtt-sn-sub-rs)

- [X] QoS 0, 1 and -1
- [X] Keep alive pings
- [X] Publishing retained messages
- [X] Publishing empty messages
- [X] Subscribing to named topic
- [X] Clean / unclean sessions
- [X] Manual and automatic client ID generation
- [X] Displaying topic name with wildcard subscriptions
- [X] Pre-defined topic IDs and short topic names
- [ ] Forwarder encapsulation according to MQTT-SN Protocol Specification v1.2.

## Serial port bridge (mqtt-sn-serial-bridge-rs)

- [ ] QoS 0, 1 and -1
- [ ] Keep alive pings
- [ ] Publishing retained messages
- [ ] Publishing empty messages
- [ ] Subscribing to named topic
- [ ] Clean / unclean sessions
- [ ] Manual and automatic client ID generation
- [ ] Displaying topic name with wildcard subscriptions
- [ ] Pre-defined topic IDs and short topic names
- [ ] Forwarder encapsulation according to MQTT-SN Protocol Specification v1.2.

## Dumping (mqtt-sn-dump-rs)

- [ ] QoS 0, 1 and -1
- [ ] Keep alive pings
- [ ] Publishing retained messages
- [ ] Publishing empty messages
- [ ] Subscribing to named topic
- [ ] Clean / unclean sessions
- [ ] Manual and automatic client ID generation
- [ ] Displaying topic name with wildcard subscriptions
- [ ] Pre-defined topic IDs and short topic names
- [ ] Forwarder encapsulation according to MQTT-SN Protocol Specification v1.2.



Now, given I was already writting a whole new set of tools, I though I might also support some additional stuff. This extended feature set (most of which is only planned so far, not implemented) goes like this:

- [X] Loop publishing. This allows to publish a message in a loop, with a given delay between each message. This is useful for testing purposes, and it's quite easy to implement.

- [X] Agnostic network layer. The original tools are quite tied to the UDP protocol, which is fine for most cases. But I wanted to make it possible to use other network protocols, such as TCP or even serial connections. This is done by defining a trait for the network layer, which is implemented by the UDP network layer(and there's also a WIP serial implementation). This is already implemented, and it's quite easy to implement new network layers.

- [X] NEW!! Serial publisher and subscriber. The original set of tools provides a bridge, which is quite useful to connect a device sending and receiving data over a serial port with a gateway listening over UDP. This additional tool will help debug connections and provide a way to send a raw stream of data over a serial connection (to emulate a SN device, and other possible use cases).


# Limitations

As of now, limitations are just the same as in the original set of tools:

- Packets must be 255 or less bytes long. I might remove it later on (MQTT-SN supports longer packets, but it requires an underlying network able to send them **without fragmentation**).
- No Last Will and Testament. So far I have no use for it, so it's highly unlikely I fix it.
- No QoS 2. This could be done eventually, not a high priority though.
- No automatic re-sending of lost packets.
- No automatic gateway discovery. This might be a possibility in the future.

# Notable differences with the original tools

This version, being written in Rust, does some things the original set of tools can't, and make some slight changes in some defaults. Among them:

- Native versions for Windows and Linux. They might also work on FreeBSD and macOS, I haven't tried yet.
- Mostly thread safe code. This usually happens when using Rust, just by the way it was designed. There might be some unsafe stuff here and there yet, but I'm actively trying to replace it with safer methods.
- Default port for MQTT-SN 10000 instead of 1883.
- The aforementioned extended features will, by definition, make this quite different from the original tools. The idea is to avoid breaking compatibility, though.

# Building

It's Rust, so pretty unsurprisingly you can build it with:

```shell
cargo build
```

# Usage

## UDP Publishing

    Usage: mqtt-sn-pub-rs [opts] -t <topic> -m <message>

      -d             Increase debug level by one. -d can occur multiple times.
      -f <file>      A file to send as the message payload.
      -h <host>      MQTT-SN host to connect to. Defaults to '127.0.0.1'.
      -i <clientid>  ID to use for this client. Defaults to 'mqtt-sn-tools-' with process id.
      -k <keepalive> keep alive in seconds for this client. Defaults to 10.
      -e <sleep>     sleep duration in seconds when disconnecting. Defaults to 0.
      -m <message>   Message payload to send.
      -l             Read from STDIN, one message per line.
      -n             Send a null (zero length) message.
      -p <port>      Network port to connect to. Defaults to 10000.
      -q <qos>       Quality of Service value (0, 1 or -1). Defaults to 0.
      -r             Message should be retained.
      -s             Read one whole message from STDIN.
      -t <topic>     MQTT-SN topic name to publish to.
      -T <topicid>   Pre-defined MQTT-SN topic ID to publish to.
      --fe           Enables Forwarder Encapsulation. MQTT-SN packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.
      --wlnid        If Forwarder Encapsulation is enabled, wireless node ID for this client. Defaults to process id.
      --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to 0.

    Some extended options:
      --loop-freq    Frequency in Hz to send messages. Defaults to 0 (disabled).
      --count        Number of messages to send in loop. Defaults to 0 (loops forever).


## UDP Subscribing

      -1             exit after receiving a single message.
      -c             disable 'clean session' (store subscription and pending messages when client disconnects).
      -d             Increase debug level by one. -d can occur multiple times.
      -h <host>      MQTT-SN host to connect to. Defaults to '127.0.0.1'.
      -i <clientid>  ID to use for this client. Defaults to 'mqtt-sn-tools-' with process id.
      -k <keepalive> keep alive in seconds for this client. Defaults to 3.
      -e <sleep>     sleep duration in seconds when disconnecting. Defaults to 0.
      -p <port>      Network port to connect to. Defaults to '10000'.
      -q <qos>       QoS level to subscribe with (0 or 1). Defaults to 0.
      -t <topic>     MQTT-SN topic name to subscribe to. It may repeat multiple times.
      -T <topicid>   Pre-defined MQTT-SN topic ID to subscribe to. It may repeat multiple times.
      --fe           Enables Forwarder Encapsulation. Mqtt-sn packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.
      --wlnid        If Forwarder Encapsulation is enabled, wireless node ID for this client. Defaults to process id (truncating if necessary).
      --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to 0.
      -v             Print messages verbosely, showing the topic name. 
      -V             Print messages verbosely, showing current time and the topic name. Currently, only id.

## UDP Dumping

This is still a WIP

## Serial Port Bridge

This is still a WIP.


# Roadmap
- [ ] Implement the serial port bridge.
- [ ] Implement the UDP dumping tool.
- [ ] Implement the serial publisher and subscriber.
- [ ] General refactoring and cleanup.
- [ ] Add proper tests.
- [ ] Add proper documentation.

# License

This project is licensed under the [GPLv3 License].

[GPLv3 License]: https://opensource.org/license/gpl-3-0

# Acknowledgements

- **Nicholas Humfrey** (https://github.com/njh): For writing the original set of tools and releasing them as free software. This project relied heavily on them for the first stage, overall architecture and design. I also pretty much cloned the CLI parameters to keep compatibility, and the usage messages are quite similar too.


# Author

This project was created by Nicolás Hermosilla P.

You can find my other projects on my GitHub page: https://github.com/nhermosilla14/

# Contributors

There are no other contributors yet, but this is oficially open for PRs and issue reporting. Feel free to contribute in any way you like.
