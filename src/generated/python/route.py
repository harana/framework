# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Route(BaseModel):
    """
    Route
    
    Class: route
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    method: str = Field(default="GET")
    middleware: Optional[str] = None
    path: str
    rate_limit: Optional[int] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroup(BaseModel):
    """
    Route Group
    
    Class: route_group
    ID fields: name
    """

    middleware: Optional[str] = None
    prefix: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroupAssignment(BaseModel):
    """
    Route Group Assignment
    
    Class: route_group_assignment
    ID fields: route_id, group_id
    """

    group_id: str
    route_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


