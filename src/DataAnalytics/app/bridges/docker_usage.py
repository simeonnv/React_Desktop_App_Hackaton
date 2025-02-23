from collections.abc import AsyncIterator

import aiohttp
from pydantic import ValidationError, TypeAdapter
from loguru import logger

from app.schemas import ContainerStats


async def get_docker_usage(session: aiohttp.ClientSession, interval_secs: int = 5) -> AsyncIterator[list[ContainerStats]]:
    url = f"ws://hackaton_backend:6004/docker/usage?interval={interval_secs}"

    try:
        async with session.ws_connect(url) as ws:
            print(f"Connected to {url}")

            async for msg in ws:
                if msg.type == aiohttp.WSMsgType.TEXT:
                    raw_message = msg.data
                    logger.info("Received ws: " + raw_message[:30] + ("...(truncated)" if len(raw_message) > 30 else ""))
                    try:
                        # Validate and parse the JSON message
                        docker_response = TypeAdapter(list[ContainerStats]).validate_json(raw_message)
                        yield docker_response  # Yield parsed data instead of printing
                    except ValidationError as ve:
                        logger.error("Validation error while parsing JSON:", ve)
                        raise ve
                elif msg.type in {aiohttp.WSMsgType.CLOSED, aiohttp.WSMsgType.ERROR}:
                    logger.error("WebSocket closed or encountered an error")
                    break
    except Exception as e:
        logger.error("Failed to connect to WebSocket:", e)
        raise e
