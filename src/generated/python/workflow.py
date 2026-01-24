# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Workflow(BaseModel):
    """
    Workflow
    
    Class: workflow
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    trigger_type: str = Field(default="manual")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: int = Field(default=1)
    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowStep(BaseModel):
    """
    Workflow Step
    
    Class: workflow_step
    ID fields: workflow_id, name
    """

    action: str
    condition: Optional[str] = None
    on_failure: str = Field(default="stop")
    retry_count: int = Field(default=0)
    sort_order: int = Field(default=0)
    timeout_seconds: int = Field(default=300)
    workflow_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowExecution(BaseModel):
    """
    Workflow Execution
    
    Class: workflow_execution
    ID fields: workflow_id, created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    input: Optional[str] = None
    output: Optional[str] = None
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    triggered_by: Optional[str] = Field(default=None, description="Reference: User.id")
    workflow_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowStepExecution(BaseModel):
    """
    Workflow Step Execution
    
    Class: workflow_step_execution
    ID fields: execution_id, step_id
    """

    completed_at: Optional[datetime] = None
    error_message: Optional[str] = None
    input: Optional[str] = None
    output: Optional[str] = None
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    step_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


