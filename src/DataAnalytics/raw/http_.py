import json
from datetime import datetime
from enum import Enum
from typing import List, Optional, Annotated

import aiohttp
import uvicorn
from fastapi import FastAPI
from pydantic import BaseModel, Field, ValidationError

from schemas import LoginResponse


class ContainerStatus(str, Enum):
    RUNNING = "running"
    PAUSED = "paused"
    RESTARTING = "restarting"
    EXITED = "exited"
    DEAD = "dead"

class HealthStatus(str, Enum):
    STARTING = "starting"
    HEALTHY = "healthy"
    UNHEALTHY = "unhealthy"
    NONE = "none"

class ContainerResourceMetrics(BaseModel):
    timestamp: Annotated[datetime, Field(alias="timestamp")]
    cpu_percent: Annotated[float, Field(alias="cpuPercent")]
    memory_usage: Annotated[int, Field(alias="memoryUsage")]
    memory_limit: Annotated[int, Field(alias="memoryLimit")]
    network_rx_bytes: Annotated[int, Field(alias="networkRxBytes")]
    network_tx_bytes: Annotated[int, Field(alias="networkTxBytes")]
    block_io_read: Annotated[int, Field(alias="blockIoRead")]
    block_io_write: Annotated[int, Field(alias="blockIoWrite")]

class ContainerHealth(BaseModel):
    status: Annotated[HealthStatus, Field(alias="status")]
    log: Annotated[Optional[List[str]], Field(alias="log")] = None
    last_output: Annotated[Optional[str], Field(alias="lastOutput")] = None

class ContainerDetails(BaseModel):
    container_id: Annotated[str, Field(alias="containerId")]
    name: Annotated[str, Field(alias="name")]
    image: Annotated[str, Field(alias="image")]
    status: Annotated[ContainerStatus, Field(alias="status")]
    created_at: Annotated[datetime, Field(alias="createdAt")]
    started_at: Annotated[Optional[datetime], Field(alias="startedAt")] = None
    finished_at: Annotated[Optional[datetime], Field(alias="finishedAt")] = None
    restart_count: Annotated[int, Field(alias="restartCount")]
    exit_code: Annotated[Optional[int], Field(alias="exitCode")] = None
    ports: Annotated[List[str], Field(alias="ports")]
    command: Annotated[str, Field(alias="command")]
    labels: Annotated[dict, Field(alias="labels")]
    resource_metrics: Annotated[List[ContainerResourceMetrics], Field(alias="resourceMetrics")]
    health: Annotated[Optional[ContainerHealth], Field(alias="health")] = None

class ServiceStatus(BaseModel):
    service_name: Annotated[str, Field(alias="serviceName")]
    desired_replicas: Annotated[int, Field(alias="desiredReplicas")]
    running_replicas: Annotated[int, Field(alias="runningReplicas")]
    failed_replicas: Annotated[int, Field(alias="failedReplicas")]
    containers: Annotated[List[ContainerDetails], Field(alias="containers")]

class DockerComposeProject(BaseModel):
    project_name: Annotated[str, Field(alias="projectName")]
    compose_file: Annotated[str, Field(alias="composeFile")]
    version: Annotated[str, Field(alias="version")]
    status: Annotated[str, Field(alias="status")]
    services: Annotated[List[str], Field(alias="services")]
    dependencies: Annotated[dict, Field(alias="dependencies")]
    created_at: Annotated[datetime, Field(alias="createdAt")]
    updated_at: Annotated[datetime, Field(alias="updatedAt")]

class ErrorLog(BaseModel):
    timestamp: Annotated[datetime, Field(alias="timestamp")]
    service_name: Annotated[str, Field(alias="serviceName")]
    container_id: Annotated[str, Field(alias="containerId")]
    error_message: Annotated[str, Field(alias="errorMessage")]
    error_code: Annotated[Optional[int], Field(alias="errorCode")] = None
    stack_trace: Annotated[Optional[str], Field(alias="stackTrace")] = None

class MonitoringData(BaseModel):
    project: Annotated[DockerComposeProject, Field(alias="project")]
    services: Annotated[List[ServiceStatus], Field(alias="services")]
    errors: Annotated[List[ErrorLog], Field(alias="errors")]
    timestamp: Annotated[datetime, Field(alias="timestamp")]


async def login(session: aiohttp.ClientSession) -> str | None:
    """
    Logs in to the auth endpoint and returns the bearer token.
    In this example, credentials are sent in the JSON payload.
    Adjust the payload as needed for your authentication requirements.
    """
    # Dummy credentials; update these as needed
    url = "http://localhost:6004/auth/login"
    payload = {"username": "admin", "password": "admin"}

    async with session.post(url, json=payload) as response:
        response_data = await response.json()

        try:
            login_response = LoginResponse.model_validate(response_data)
            if login_response.status == "success":
                print("Login successful, token:", login_response.data)
                return login_response.data
            else:
                print("Login failed:", response_data)
                return None
        except ValidationError as ve:
            print("Failed to validate login response:", ve)
            return None
