# Overview

As a software engineer, I selected Rust to strengthen my ability to learn a language used in embedded and systems programming while applying structured and object-oriented design techniques in a practical project. Demonstrating my goal by implementing an aviation-focused fault detection system using Rust.

The software I developed is a Rust Aircraft Fault Detection System for Predictive Maintenance, a console-based application that loads aircraft sensor data from a CSV file, processes each record, and evaluates whether conditions are normal, warning, or critical. Each aircraft component is analyzed using threshold-based logic for temperature, vibration, flight cycles, and oil pressure.

The system supports:
- Viewing sensor data (all aircraft or individual aircraft)
- Running fault detection (all or one aircraft)
- Viewing summaries (all reports, one aircraft, or faults only)
- Persistent logging of results to CSV for traceability

The purpose of this project was to gain hands-on experience with Rust while applying core programming concepts such as variables, expressions, conditionals, loops, functions, structs, and vectors. Additionally, this project explores how software can support predictive maintenance in aviation by identifying potential issues before failure occurs.

## Software Demo Video's

### Video 1:
### Video 2:

# Development Environment

## Software was developed using:
- Visual Studio Code
- Rust
- Cargo (Rust build system and package manager)
- Rust Analyzer extension
- macOS Terminal

Application uses the Rust standard library along with external crates:
- chrono → for timestamps in logs and reports
- csv → for reading and writing CSV data
- serde → for deserializing CSV into structs
- colored → for enhanced terminal output and readability

Programming language used in this project is Rust. My application uses the Rust standard library along with the `chrono` crate to generate timestamps for advanced fault logging. Cargo was used to manage dependencies, build the application, and run the program from the terminal.

## Rust Concepts Demonstrated
- Variables (mutable and immutable)
- Expressions
- Conditionals
- Loops
- Functions with references
- Data structures using Vec
- Object-oriented techniques using struct and impl

## Key Features
- Aircraft sensor data loaded from CSV
- Fault detection using threshold-based logic
- Severity classification (Low, Moderate, High, Critical)
- Maintenance recommendations for each component
- Timestamped results for traceability
- CSV logging for persistent audit trail
- Sorting reports by severity
- Filtering to show only faulty aircraft
- Color-coded terminal output
- Menu-driven interface

## Project Structure

rust-aircraft-fault-detection/
│
├── data/
│   ├── fault_log.csv
│   └── sample_sensor_data.csv
│
├── src/
│   ├── logger.rs       # Handles CSV logging
│   ├── main.rs         # Entry point and menu control
│   ├── models.rs       # Data structures and methods
│   ├── parser.rs       # CSV loading logic
│   ├── reports.rs      # Fault detection and analysis
│   └── ui.rs           # User interface (CLI)
│
├── target/             # build output (generated)
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md

# Run Software

- Install Software:
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
- Build Project:
        cargo build
- Run Project:
        cargo run

## Aircraft ID's

| ID   | Aircraft Name |
|------|-------------- |
| A100 | Boeing 737    |
| A200 | Airbus A320   |
| A300 | Cessna 172    |
| A400 | Embraer E190  |
| A500 | Boeing 787    |

# Summary

This project demonstrates how Rust can be used to build reliable, structured, and efficient software systems. Combining a real-world application in aviation maintenance with core programming principles, while showcasing Rust’s strengths in safety, performance, and modular design.

# Useful Websites

- Rust Official Website: https://rust-lang.org/
- Rust Programming Language Book: https://doc.rust-lang.org/book/
- Full Rust Course: https://youtu.be/rQ_J9WH6CGk?si=Jso4iwFtXyJrhqhh

# Future Work

- Add unit tests for parsing, fault detection logic, and logging
- Implement automated testing using Rust test framework
- Add advanced predictive analytics (trend analysis over time)
- Support additional aircraft systems and sensors
- Integrate a database (e.g., SQLite)
- Improve input validation and error handling
- Add export options (JSON/API)
- Build a graphical or web-based interface