# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareWorkflowInstanceCreated(BaseModel):
    """
    cloudflare_workflow_instance_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceBatchCreated(BaseModel):
    """
    cloudflare_workflow_instance_batch_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceStatusChanged(BaseModel):
    """
    cloudflare_workflow_instance_status_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstancePaused(BaseModel):
    """
    cloudflare_workflow_instance_paused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceResumed(BaseModel):
    """
    cloudflare_workflow_instance_resumed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceRestarted(BaseModel):
    """
    cloudflare_workflow_instance_restarted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceTerminated(BaseModel):
    """
    cloudflare_workflow_instance_terminated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceCompleted(BaseModel):
    """
    cloudflare_workflow_instance_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowInstanceErrored(BaseModel):
    """
    cloudflare_workflow_instance_errored
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowEventSent(BaseModel):
    """
    cloudflare_workflow_event_sent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowStepExecuted(BaseModel):
    """
    cloudflare_workflow_step_executed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkflowStepFailed(BaseModel):
    """
    cloudflare_workflow_step_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


