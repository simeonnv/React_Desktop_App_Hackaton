from typing import List, Optional

from pydantic import BaseModel, Field


class NetworkStats(BaseModel):
    rx_dropped: int
    rx_bytes: int
    rx_errors: int
    tx_packets: int
    tx_dropped: int
    rx_packets: int
    tx_errors: int
    tx_bytes: int

class CPUUsage(BaseModel):
    percpu_usage: Optional[List[int]] = None
    total_usage: int

class ThrottlingData(BaseModel):
    periods: int
    throttled_periods: int
    throttled_time: int

class CPUStats(BaseModel):
    cpu_usage: CPUUsage
    system_cpu_usage: Optional[int] = None
    online_cpus: Optional[int] = None
    throttling_data: ThrottlingData

class PidsStats(BaseModel):
    current: Optional[int] = None

class MemoryStats(BaseModel):
    usage: Optional[int] = None
    max_usage: Optional[int] = None
    limit: Optional[int] = None

class BlkioStats(BaseModel):
    io_service_bytes_recursive: Optional[list[dict[str, int]]] = None
    io_serviced_recursive: Optional[list[dict[str, int]]] = None

class StorageStats(BaseModel):
    read_count: Optional[int] = None
    write_count: Optional[int] = None
    read_size_bytes: Optional[int] = None
    write_size_bytes: Optional[int] = None

class ContainerStats(BaseModel):
    read: str
    preread: str
    num_procs: int
    pids_stats: PidsStats
    network: Optional[NetworkStats] = None
    # networks: Optional[dict[str, NetworkStats]] = None
    memory_stats: MemoryStats
    # blkio_stats: BlkioStats
    cpu_stats: CPUStats
    precpu_stats: CPUStats
    storage_stats: StorageStats
    name: str = Field(default="")
    id: str = Field(default="", alias="Id")



# async def login(session: aiohttp.ClientSession) -> str | None:
#     """
#     Logs in to the auth endpoint and returns the bearer token.
#     In this example, credentials are sent in the JSON payload.
#     Adjust the payload as needed for your authentication requirements.
#     """
#     # Dummy credentials; update these as needed
#     url = "http://localhost:6004/auth/login"
#     payload = {"username": "admin", "password": "admin"}
#
#     async with session.post(url, json=payload) as response:
#         response_data = await response.json()
#
#         try:
#             login_response = LoginResponse.model_validate(response_data)
#             if login_response.status == "success":
#                 print("Login successful, token:", login_response.data)
#                 return login_response.data
#             else:
#                 print("Login failed:", response_data)
#                 return None
#         except ValidationError as ve:
#             print("Failed to validate login response:", ve)
#             return None
#
