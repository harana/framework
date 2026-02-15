# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CfHyperdriveConfig(BaseModel):
    """
    cf_hyperdrive_config
    
    ID fields: account_id, config_name
    """

    account_id: str
    binding: str
    caching_disabled: bool = Field(default=False)
    config_id: Optional[str] = None
    config_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    database: str
    host: str
    port: int
    scheme: str = Field(default="postgres")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user: str
    class Config:
        from_attributes = True
        populate_by_name = True


