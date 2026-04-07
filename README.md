# Overview

As a software engineer, I selected Rust to strengthen my ability to learn a language used in embedded and systems programming while applying object-oriented design techniques in a practical project. This project demonstrates that goal by implementing an aviation-focused fault detection system using Rust.

The software I wrote is a Rust Aircraft Fault Detection System for Predictive Maintenance. It is a console-based application that loads aircraft sensor data from a CSV file, processes each record, and evaluates whether conditions are normal, warning, or critical. Each aircraft component is analyzed using threshold-based logic for temperature, vibration, flight cycles, and oil pressure. The system provides maintenance recommendations and displays the results in a structured format.

The purpose of writing this software was to gain hands-on experience with the Rust programming language while applying core programming concepts such as variables, expressions, conditionals, loops, functions, structs, and vectors. Additionally, I wanted to explore how software can support predictive maintenance in aviation by identifying potential issues before failure occurs and maintaining persistent logs for traceability and analysis.

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

Programming language used in this project is Rust. My application uses the Rust standard library along with the `chrono` crate to generate timestamps for advanced fault logging. Cargo was used to manage dependencies, build the application, and run the program from the terminal.

## Rust Concepts Demonstrated
- Variables (mutable and immutable)
- Expressions
- Conditionals
- Loops
- Functions with references
- Data structures using Vec
- Object-oriented techniques using struct and impl

# Useful Websites

- Rust Official Website https://rust-lang.org/
- Rust Programming Language Book https://doc.rust-lang.org/book/
- Full Rust Course https://youtu.be/rQ_J9WH6CGk?si=Jso4iwFtXyJrhqhh

# Future Work

- Add unit tests for parsing, fault detection logic, and logging
- Add color-coded terminal output for severity levels (normal, warning, critical)
- Implement filtering options e.g., show only critical faults
- Add support for additional aircraft systems and sensor types
- Implement trend analysis for predictive maintenance over time
- Integrate a database (such as SQLite) for more advanced data storage
- Improve input validation and error handling for CSV data

# Project Structure

rust-aircraft-fault-detection/
│
├── data/
│   ├── fault_log.csv
│   └── sample_sensor_data.csv
│
├── src/
│   ├── logger.rs
│   ├── main.rs
│   ├── models.rs
│   ├── parser.rs
│   ├── reports.rs
│   └── ui.rs
│
├── target/              # build output (generated)
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md

# Run Software

- Install Software 
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
- Build Project
        cargo build
- Run Project
        cargo run
