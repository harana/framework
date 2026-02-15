# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PublishEcr(BaseModel):
    """
    publish_ecr
    
    ID fields: id
    """

    access_key_env: str
    create_repository: bool = Field(default=False)
    enabled: bool = Field(default=True)
    image: str
    image_scanning: bool = Field(default=True)
    image_tag_mutability: str = Field(default="mutable")
    lifecycle_policy: Optional[str] = None
    region: str
    registry_id: Optional[str] = None
    repository: str
    secret_key_env: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


