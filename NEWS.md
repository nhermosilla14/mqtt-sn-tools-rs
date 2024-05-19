Release History
===============

Version 0.0.2 (2024-05-19)
--------------------------

- Add SerialNetwork trait to support multiple communication methods
- Fix issue when receiving network timeout error
- Add first steps to support raw serial communication
- Add extra options to the CLI, allowing to publish in a loop


Version 0.0.1 (2024-05-13)
--------------------------

- First version
- Support for publishing and subscribing with QoS -1, 0 and 1
- Support for reading from a file using -f
- Support for reading from STDIN using -s
- Support for publishing one message per line using -l
- Support for timeout when waiting for packets
- Support for logging at three different levels