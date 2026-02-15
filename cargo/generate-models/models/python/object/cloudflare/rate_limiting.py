# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CfRateLimiter(BaseModel):
    """
    cf_rate_limiter
    
    ID fields: account_id, limiter_name
    """

    account_id: str
    binding: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    limiter_name: str
    period_seconds: int = Field(default=60)
    requests_per_period: int = Field(default=100)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


