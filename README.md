# ChronoDB ⏳⚡

<p align="center">
  <img src="assets/logo.png" alt="ChronoDB Logo" width="300"/>
</p>

**A time-aware, JSON document store with TTL support written in Rust**  
*"Data that expires when you want it to"*

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- Time-To-Live (TTL) support for automatic data expiration  
- Simple JSON-over-TCP protocol  
- Support for insert, query, delete operations by bucket and pattern  
- Bucket-based data organization  

## Installation
```sh
git clone https://github.com/blhmr/chronodb
cd chronodb && cargo build --release
```

## Example JSON Requests

Insert data with TTL (in seconds):
```json
{
  "method": "insert",
  "bucket": "users",
  "data": {"username": "hatim", "age": 18},
  "ttl": 20
}
```

Get documents matching a pattern:
```json
{
  "method": "get",
  "bucket": "users",
  "pattern": {"age": 18}
}
```

Get all bucket names:
```json
{
  "method": "get_all_buckets"
}
```

Get all documents from a bucket:
```json
{
  "method": "get_bucket",
  "bucket": "users"
}
```

Get all documents from all buckets:
```json
{
  "method": "get_all"
}
```

Delete documents by pattern:
```json
{
  "method": "delete",
  "bucket": "users",
  "pattern": {"username": "hatim"}
}
```

Delete an entire bucket:
```json
{
  "method": "delete_bucket",
  "bucket": "users"
}
```

## Connection examples
Using netcat:
```sh
echo '{"method":"get_bucket","bucket":"users"}' | nc localhost 8080
```

Using telnet:
```sh
telnet localhost 8080
```

## Example with client in Python
```py
import asyncio
import json

TEST_CASES = [
    {"method":"insert","bucket":"users","data":{"username":"hatim","age":18},"ttl":20},
    {"method":"get","bucket":"users","pattern":{"age":18}},
    {"method":"get_all_buckets"},
    {"method":"get_bucket","bucket":"users"},
    {"method":"get_all"},
    {"method":"delete","bucket":"users","pattern":{"username":"hatim"}},
    {"method":"delete_bucket","bucket":"users"},
]

async def send_request(request):
    reader, writer = await asyncio.open_connection('127.0.0.1', 8080)
    message = json.dumps(request) + "\n"
    writer.write(message.encode())
    await writer.drain()
    data = await reader.read(4096)
    writer.close()
    await writer.wait_closed()
    return data.decode()

async def main():
    for i, req in enumerate(TEST_CASES, 1):
        print(f"Test case {i}: sending {req}")
        response = await send_request(req)
        print(f"Response: {response.strip()}\n")

if __name__ == "__main__":
    asyncio.run(main())
```

Expected output:
```
Test case 1: sending {'method': 'insert', 'bucket': 'users', 'data': {'username': 'hatim', 'age': 18}, 'ttl': 20}
Response: {"status":"OK"}

Test case 2: sending {'method': 'get', 'bucket': 'users', 'pattern': {'age': 18}}
Response: {"data":[{"age":18,"username":"hatim"}],"status":"OK"}

Test case 3: sending {'method': 'get_all_buckets'}
Response: {"data":["users"],"status":"OK"}

Test case 4: sending {'method': 'get_bucket', 'bucket': 'users'}
Response: {"data":[{"age":18,"username":"hatim"}],"status":"OK"}

Test case 5: sending {'method': 'get_all'}
Response: {"data":{"users":[{"age":18,"username":"hatim"}]},"status":"OK"}

Test case 6: sending {'method': 'delete', 'bucket': 'users', 'pattern': {'username': 'hatim'}}
Response: {"status":"OK"}

Test case 7: sending {'method': 'delete_bucket', 'bucket': 'users'}
Response: {"status":"OK"}
```

## Notes
- All JSON requests must be newline-delimited when using raw TCP (telnet, netcat)
- TTL values are in seconds; omitted TTL means data persists indefinitely
- Pattern matching is done by exact match of JSON fields

## License
MIT © 2025 Hatim