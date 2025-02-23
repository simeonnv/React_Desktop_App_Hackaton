import asyncio
import aiohttp
from pydantic import ValidationError, TypeAdapter

from http_ import login
from schemas import ContainerData


async def websocket_handler(session: aiohttp.ClientSession, container_id: str):
    url = "ws://localhost:6004/docker/usage"
    headers = {"Authorization": f"Bearer {await login(session)}"}

    try:
        async with session.ws_connect(url, headers=headers) as ws:
            print(f"Connected to {url}")
            # Send the container_id we're interested in
            # await ws.send_str(container_id)

            async for msg in ws:
                if msg.type == aiohttp.WSMsgType.TEXT:
                    raw_message = msg.data
                    print("Raw message received:", raw_message)
                    try:
                        # Validate and parse the JSON message using DockerUsageResponse model
                        docker_response = TypeAdapter(list[ContainerData]).validate_json(raw_message)
                        print("Parsed response:", docker_response)
                    except ValidationError as ve:
                        print("Validation error while parsing JSON:", ve)
                elif msg.type == aiohttp.WSMsgType.CLOSED:
                    print("WebSocket closed")
                    break
                elif msg.type == aiohttp.WSMsgType.ERROR:
                    print("WebSocket error:", msg)
                    break
    except Exception as e:
        print("Failed to connect to WebSocket:", e)

async def main():
    # container_id = input("id: ")

    async with aiohttp.ClientSession() as session:
        await websocket_handler(session, "")

if __name__ == "__main__":
    asyncio.run(main())
