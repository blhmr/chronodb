# ChronoDB ⏳⚡

![ChronoDB Logo](assets/logo.png)

**A time-aware key-value store written in Rust**  
*"Data with an expiration date"*

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features
- 🕒 **TTL Support**: Automatic key expiration
- 🚀 **JSON-over-TCP**: Simple protocol

## Installation
```sh
git clone https://github.com/blhmr/chronodb
cd chronodb && cargo build --release
```

## Usage
Start the server
```sh
chronodb 8080
```

Basic operations:
Set a persistent key
```json
{"method": "SET", "key": "name", "value": "ChronoDB"}
```

Set with 30s TTL
```json
{"method": "SET", "key": "temp", "value": "data", "ttl": 30}
```

Get single key
```json
{"method": "GET", "key": "name"}
```

Get all keys
```json
{"method": "GET", "key": "*"}
```

## Connection examples
Using netcat:
```sh
echo '{"method":"GET","key":"username"}' | nc localhost 8080
```

Using HTTP (if enabled)
```sh
curl -X POST -d '{"method":"SET","key":"status","value":"online"}' http://localhost:8080
```

## Notes
- All JSON commands must be newline-delimited when using raw TCP
- TTL values are in seconds (none = persistent)
- GET * returns all key-value pairs as a single JSON object

## Architecture
```mermaid
graph TD
    Client -->|JSON| Server
    Server -->|TTL Check| Scheduler
    Server -->|Read/Write| Storage
```

## License
MIT © 2025 Hatim