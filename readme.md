# IRacing SDK Rust Library

## Overview

irsdk is a Rust implementation of the iRacing SDK, designed to interact with the iRacing simulation software by accessing shared memory data and sending broadcast messages. This library is a direct translation of the Python irsdk library, maintaining the same functionality while leveraging Rust's safety and performance features. It provides an interface to read telemetry data, control camera views, manage replays, and interact with various simulation features.

## Features

* Shared Memory Access: Connect to iRacing's shared memory on Windows using Windows API.
* Telemetry Data: Read real-time telemetry data such as speed, RPM, and other variables.
* Broadcast Messages: Send commands to control camera, replay, chat, pit commands, and more.
* Session Information: Parse YAML-formatted session data from the simulation.
* Windows API Integration: Utilizes windows-rs for native Windows API calls to handle events and memory mapping.

## Requirements

* Rust: Version 1.56 or higher with cargo.
* Windows: This library is designed to work on Windows due to its reliance on Windows API for shared memory and event handling.
* iRacing: Must be installed and running to access shared memory data.

## Usage
Below is a basic example of how to initialize the IRSDK, connect to iRacing, and read a telemetry value (e.g., Speed).
Example: Reading Speed from iRacing:

```
use irsdk::IRSDK;

fn main() {
    let mut ir = IRSDK::new(false); // Initialize IRSDK
    match ir.startup(None, None) { // Start connection to iRacing (None for no test file or dump)
        Ok(initialized) => {
            if initialized {
                println!("IRSDK initialized successfully");
                if ir.is_connected() {
                    println!("Connected to iRacing");
                    if let Some(speed) = ir.get("Speed") {
                        println!("Speed: {}", speed);
                    } else {
                        println!("Failed to read Speed");
                    }
                } else {
                    println!("Not connected to iRacing");
                }
            } else {
                println!("IRSDK initialization failed");
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

```

## License
This project is licensed under the MIT License.