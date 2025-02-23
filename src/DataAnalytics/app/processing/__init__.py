import asyncio
from datetime import time
from time import sleep

import aiohttp
from loguru import logger

from app import ContainerStats
from app.bridges.docker_usage import get_docker_usage


def calculate_cpu_percent(stats: ContainerStats) -> float:
    previous_cpu = stats.precpu_stats.cpu_usage.total_usage
    previous_system = stats.precpu_stats.system_cpu_usage or 0

    cpu_percent = 0.0

    # Calculate the change for the CPU usage of the container between readings
    cpu_delta = float(stats.cpu_stats.cpu_usage.total_usage) - float(previous_cpu)

    # Calculate the change for the entire system between readings
    system_delta = float(stats.cpu_stats.system_cpu_usage or 0) - float(previous_system)

    if system_delta > 0.0 and cpu_delta > 0.0:
        percpu_usage_count = len(stats.cpu_stats.cpu_usage.percpu_usage or [])
        cpu_percent = (cpu_delta / system_delta) * float(percpu_usage_count) * 100.0

    return cpu_percent


async def collect_usage():
    async with aiohttp.ClientSession() as session:
        async for data in get_docker_usage(session):
            pass
