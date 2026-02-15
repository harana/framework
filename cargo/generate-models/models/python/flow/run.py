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


class Stop(BaseModel):
    """
    stop
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Kill(BaseModel):
    """
    kill
    
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


class List(BaseModel):
    """
    list
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Wait(BaseModel):
    """
    wait
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Output(BaseModel):
    """
    output
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ProcessStatus(BaseModel):
    """
    process_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ProcessInfo(BaseModel):
    """
    process_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


