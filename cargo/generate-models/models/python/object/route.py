# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    method: str = Field(default="get")
    middleware: Optional[str] = None
    path: str
    rate_limit: Optional[int] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroup(BaseModel):
    """
    route_group
    
    ID fields: id
    """

    middleware: Optional[str] = None
    prefix: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroupAssignment(BaseModel):
    """
    route_group_assignment
    
    ID fields: route_id, group_id
    """

    group_id: str = Field(description="Reference: route_group.id")
    route_id: str = Field(description="Reference: route.id")
    class Config:
        from_attributes = True
        populate_by_name = True


