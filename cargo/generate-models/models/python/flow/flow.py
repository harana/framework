# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Flow(BaseModel):
    """
    flow
    
    ID fields: slug
    """

    name: str
    description: Optional[str] = None
    steps: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class FlowStep(BaseModel):
    """
    flow_step
    
    ID fields: feed_id, created_at
    """

    actor_id: Optional[str] = Field(default=None, description="Reference: user.id")
    content: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    feed_id: str = Field(description="Reference: feed.id")
    is_complete: bool = Field(default=False)
    target_id: Optional[str] = None
    target_name: Optional[str] = None
    target_type: Optional[str] = None
    type: str = Field(default="action")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


