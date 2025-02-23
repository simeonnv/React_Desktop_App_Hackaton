from pydantic import BaseModel
from typing import List
from datetime import datetime

class CPUStats(BaseModel):
    online_cpus: int
    total_usage: int

class MemoryStats(BaseModel):
    limit: int
    usage: int

class PidsStats(BaseModel):
    current: int
    limit: int

class ContainerData(BaseModel):
    cpu_stats: CPUStats
    id: str
    memory_stats: MemoryStats
    names: List[str]
    pids_stats: PidsStats
    read: datetime

# class DockerUsageResponse(BaseModel):
#     data: List[ContainerData]
#     status: str

class LoginResponse(BaseModel):
    status: str
    data: str
