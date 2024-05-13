# mqtt-sn-tools-rs
A reimplementation of mqtt-sn-tools written in Rust.


MQTT-SN Tools in Rust
=============

Command line tools written in Rust for the MQTT-SN (MQTT for Sensor Networks) protocol. This set of tools aims to be a near drop-in replacement for the ones written in C by Nicholas Humfrey.

Context
-------

This project served me as a way to exercise and learn Rust, which I've been trying to for quite a while now. I didn't properly get started until I came across the need for MQTT-SN Tools in a Windows machine. Those tools do work just fine using Cygwin, but then again, not really a proper native solution. So I began this in order to get the tools I needed. That's why the code might be uglier than it should, so I'm quite eager to get some feedback and PRs. Issues reports are also welcome.


Supported features
------------------

Since this aims for feature parity with the original tools, the currently supported features (and planed roadmap) is as follows:

### Publisher (mqtt-sn-pub-rs)

- [X] QoS 0, 1 and -1
- [X] Keep alive pings
- [X] Publishing retained messages
- [X] Publishing empty messages
- [X] Publishing to named topic (registering it first)
- [X] Clean / unclean sessions
- [X] Manual and automatic client ID generation
- [X] Pre-defined topic IDs and short topic names
- [X] Forwarder encapsulation according to MQTT-SN Protocol Specification v1.2.

### Subscriber (mqtt-sn-sub-rs)

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

### Serial port bridge (mqtt-sn-serial-bridge-rs)

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

### Dumping (mqtt-sn-dump-rs)

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

- [ ] Network agnostic publication. Something like the SensorNetwork abstraction in the PAHO MQTT-SN Gateway. That way you could, theoretically, send and receive messages using any underlying protocol, even a custom one (I'm already working on a serial implementation, starting from the code for the bridge) 


Limitations
-----------

Most of the limitations are just the same as in the original set of tools:

- Packets must be 255 or less bytes long. This is a limitation on the original set of tools, I might remove it later on (MQTT-SN supports longer packets, but it requires an underlying network able to send them **without fragmentation**).
- No Last Will and Testament. This, also, is a limitation of the original implementation. So far I have no use for it, so it's highly unlikely I fixe it.
- No QoS 2. This could be done eventually, not a high priority though.
- No automatic re-sending of lost packets.
- No automatic gateway discovery. This might be a possibility in the future.
- Although you can, just like in the original version, add `-d` several times, you can't put it all in the same parameter. I mean: `-d -d`, not `-dd`.

Limitations overcome in this version
------------------------------------

This version, being written in Rust, does some things the original set of tools can't. Among them:

- Native versions for Windows and Linux. They might also work on FreeBSD and macOS, I haven't tried yet.
- Mostly thread safe code. This usually happens when using Rust, just by the way it was designed. There might be some unsafe stuff here and there yet, but I'm actively trying to replace it with safer methods.

Building
--------

It's Rust, so pretty unsurprisingly you can build it with:

```
cargo build
```

Usage
-----

Publishing
----------

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
      -p <port>      Network port to connect to. Defaults to 1883.
      -q <qos>       Quality of Service value (0, 1 or -1). Defaults to 0.
      -r             Message should be retained.
      -s             Read one whole message from STDIN.
      -t <topic>     MQTT-SN topic name to publish to.
      -T <topicid>   Pre-defined MQTT-SN topic ID to publish to.
      --fe           Enables Forwarder Encapsulation. Mqtt-sn packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.
      --wlnid        If Forwarder Encapsulation is enabled, wireless node ID for this client. Defaults to process id.
      --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to 0.


Subscribing
-----------

    Usage: mqtt-sn-sub [opts] -t <topic>

      -1             exit after receiving a single message.
      -c             disable 'clean session' (store subscription and pending messages when client disconnects).
      -d             Increase debug level by one. -d can occur multiple times.
      -h <host>      MQTT-SN host to connect to. Defaults to '127.0.0.1'.
      -i <clientid>  ID to use for this client. Defaults to 'mqtt-sn-tools-' with process id.
      -k <keepalive> keep alive in seconds for this client. Defaults to 10.
      -e <sleep>     sleep duration in seconds when disconnecting. Defaults to 0.
      -p <port>      Network port to connect to. Defaults to 1883.
      -q <qos>       QoS level to subscribe with (0 or 1). Defaults to 0.
      -t <topic>     MQTT-SN topic name to subscribe to. It may repeat multiple times.
      -T <topicid>   Pre-defined MQTT-SN topic ID to subscribe to. It may repeat multiple times.
      --fe           Enables Forwarder Encapsulation. Mqtt-sn packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.
      --wlnid        If Forwarder Encapsulation is enabled, wireless node ID for this client. Defaults to process id.
      -v             Print messages verbosely, showing the topic name.
      -V             Print messages verbosely, showing current time and the topic name.
      --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to 0.


Dumping
-------

Displays MQTT-SN packets sent to specified port.
Most useful for listening out for QoS -1 messages being published by a client.

    Usage: mqtt-sn-dump [opts] -p <port>

      -a             Dump all packet types.
      -d             Increase debug level by one. -d can occur multiple times.
      -p <port>      Network port to listen on. Defaults to 1883.
      -v             Print messages verbosely, showing the topic name.


Serial Port Bridge
------------------

The Serial Port bridge can be used to relay packets from a remote device on the end of a
serial port and convert them into UDP packets, which are sent and received from a broker
or MQTT-SN gateway.

    Usage: mqtt-sn-serial-bridge [opts] <device>

      -b <baud>      Set the baud rate. Defaults to 9600.
      -d             Increase debug level by one. -d can occur multiple times.
      -dd            Enable extended debugging - display packets in hex.
      -h <host>      MQTT-SN host to connect to. Defaults to '127.0.0.1'.
      -p <port>      Network port to connect to. Defaults to 1883.
      --fe           Enables Forwarder Encapsulation. Mqtt-sn packets are encapsulated according to MQTT-SN Protocol Specification v1.2, chapter 5.5 Forwarder Encapsulation.
      --cport <port> Source port for outgoing packets. Uses port in ephemeral range if not specified or set to 0.


License
-------

MQTT-SN  is licensed under the [MIT License].



[GPLv3 License]: https://opensource.org/license/gpl-3-0