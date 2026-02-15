# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Create(BaseModel):
    """
    create
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateBatch(BaseModel):
    """
    create_batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Status(BaseModel):
    """
    status
    
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


class Restart(BaseModel):
    """
    restart
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Terminate(BaseModel):
    """
    terminate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendEvent(BaseModel):
    """
    send_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepDo(BaseModel):
    """
    step_do
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepSleep(BaseModel):
    """
    step_sleep
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepSleepUntil(BaseModel):
    """
    step_sleep_until
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepWaitForEvent(BaseModel):
    """
    step_wait_for_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


