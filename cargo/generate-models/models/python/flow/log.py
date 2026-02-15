# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Debug(BaseModel):
    """
    debug
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Info(BaseModel):
    """
    info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Warn(BaseModel):
    """
    warn
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Error(BaseModel):
    """
    error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Structured(BaseModel):
    """
    structured
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LogEntry(BaseModel):
    """
    log_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LogContext(BaseModel):
    """
    log_context
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LogError(BaseModel):
    """
    log_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LogData(BaseModel):
    """
    log_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


