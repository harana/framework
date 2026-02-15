# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AppStarted(BaseModel):
    """
    app_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppStopped(BaseModel):
    """
    app_stopped
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppConfigUpdated(BaseModel):
    """
    app_config_updated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppConfigLoaded(BaseModel):
    """
    app_config_loaded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppHealthCheck(BaseModel):
    """
    app_health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppReady(BaseModel):
    """
    app_ready
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppShuttingDown(BaseModel):
    """
    app_shutting_down
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppError(BaseModel):
    """
    app_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppConnectionPoolCreated(BaseModel):
    """
    app_connection_pool_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AppConnectionPoolExhausted(BaseModel):
    """
    app_connection_pool_exhausted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


