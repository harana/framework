# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareWorkflow(BaseModel):
    """
    cloudflare_workflow
    
    ID fields: account_id, workflow_name
    """

    account_id: str
    binding: str
    class_name: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    script_name: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    workflow_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstance(BaseModel):
    """
    cloudflare_workflow_instance
    
    ID fields: workflow_id, instance_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    error: Optional[str] = None
    instance_id: str
    output: Optional[str] = None
    params: Optional[str] = None
    status: str = Field(default="queued")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    workflow_id: str = Field(description="Reference: cf_workflow.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowStep(BaseModel):
    """
    cloudflare_workflow_step
    
    ID fields: instance_id, step_name
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    instance_id: str = Field(description="Reference: cf_workflow_instance.id")
    result: Optional[str] = None
    retries_backoff: str = Field(default="constant")
    retries_delay: Optional[str] = None
    retries_limit: Optional[int] = None
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    step_name: str
    step_type: str = Field(default="do")
    timeout: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowEvent(BaseModel):
    """
    cloudflare_workflow_event
    
    ID fields: instance_id, event_type, sent_at
    """

    event_type: str
    instance_id: str = Field(description="Reference: cf_workflow_instance.id")
    payload: Optional[str] = None
    sent_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


