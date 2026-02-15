# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ParallelExecution(BaseModel):
    """
    parallel_execution
    
    ID fields: id
    """

    completed_at: Optional[datetime] = None
    completed_count: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    failed_count: int = Field(default=0)
    max_concurrency: Optional[int] = None
    status: str = Field(default="pending")
    strategy: str = Field(default="all")
    timeout_ms: Optional[int] = None
    total_count: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class ParallelTask(BaseModel):
    """
    parallel_task
    
    ID fields: execution_id, task_index
    """

    completed_at: Optional[datetime] = None
    error_message: Optional[str] = None
    execution_id: str = Field(description="Reference: parallel_execution.id")
    result: Optional[str] = None
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    task_index: int
    class Config:
        from_attributes = True
        populate_by_name = True


