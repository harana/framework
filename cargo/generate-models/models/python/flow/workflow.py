# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Start(BaseModel):
    """
    start
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Pause(BaseModel):
    """
    pause
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Resume(BaseModel):
    """
    resume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Cancel(BaseModel):
    """
    cancel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetStatus(BaseModel):
    """
    get_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetResult(BaseModel):
    """
    get_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListExecutions(BaseModel):
    """
    list_executions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Signal(BaseModel):
    """
    signal
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WaitForEvent(BaseModel):
    """
    wait_for_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class History(BaseModel):
    """
    history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RetryStep(BaseModel):
    """
    retry_step
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SkipStep(BaseModel):
    """
    skip_step
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TerminateAll(BaseModel):
    """
    terminate_all
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowExecution(BaseModel):
    """
    workflow_execution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowContext(BaseModel):
    """
    workflow_context
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowExecutionInfo(BaseModel):
    """
    workflow_execution_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WorkflowHistoryEvent(BaseModel):
    """
    workflow_history_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


