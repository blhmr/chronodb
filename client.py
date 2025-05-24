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
