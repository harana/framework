# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PublishDockerhub(BaseModel):
    """
    publish_dockerhub
    
    ID fields: id
    """

    description: Optional[str] = None
    enabled: bool = Field(default=True)
    image: str
    password_env: str
    readme: Optional[str] = None
    registry: str = Field(default="docker.io")
    tags: List[str]
    username_env: str
    class Config:
        from_attributes = True
        populate_by_name = True


