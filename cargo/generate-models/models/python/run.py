# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RunProcess(BaseModel):
    """
    run_process
    
    ID fields: process_id
    """

    args: Optional[str] = None
    command: str
    cpu_usage: Optional[float] = None
    environment: Optional[str] = None
    exit_code: Optional[int] = None
    is_detached: bool = Field(default=False)
    memory_usage: Optional[int] = None
    process_id: Optional[int] = None
    started_at: datetime = Field(default_factory=datetime.utcnow)
    status: str = Field(default="pending")
    stopped_at: Optional[datetime] = None
    working_directory: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RunProcessLog(BaseModel):
    """
    run_process_log
    
    ID fields: process_id, created_at
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    process_id: str = Field(description="Reference: run_process.id")
    stream: str = Field(default="stdout")
    class Config:
        from_attributes = True
        populate_by_name = True


class ProcessStatus(BaseModel):
    """
    process_status
    
    ID fields: id
    """

    process_id: int
    running: bool
    exit_code: int
    cpu_usage: float
    memory_usage: int
    uptime: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ProcessInfo(BaseModel):
    """
    process_info
    
    ID fields: id
    """

    process_id: int
    name: str
    command: str
    user: str
    cpu_usage: float
    memory_usage: int
    status: str
    uptime: int
    class Config:
        from_attributes = True
        populate_by_name = True


