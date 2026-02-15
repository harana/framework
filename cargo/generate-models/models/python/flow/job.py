# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Schedule(BaseModel):
    """
    schedule
    
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


class Retry(BaseModel):
    """
    retry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Lists(BaseModel):
    """
    lists
    
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


class GetResult(BaseModel):
    """
    get_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateProgress(BaseModel):
    """
    update_progress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Job(BaseModel):
    """
    job
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JobInfo(BaseModel):
    """
    job_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


