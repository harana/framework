# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CheckService(BaseModel):
    """
    check_service
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckDatabase(BaseModel):
    """
    check_database
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckCache(BaseModel):
    """
    check_cache
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckExternalApi(BaseModel):
    """
    check_external_api
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckDiskSpace(BaseModel):
    """
    check_disk_space
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckMemory(BaseModel):
    """
    check_memory
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckAll(BaseModel):
    """
    check_all
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSystemStatus(BaseModel):
    """
    get_system_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HealthStatus(BaseModel):
    """
    health_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HealthDetails(BaseModel):
    """
    health_details
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HealthCheck(BaseModel):
    """
    health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SystemMetrics(BaseModel):
    """
    system_metrics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


