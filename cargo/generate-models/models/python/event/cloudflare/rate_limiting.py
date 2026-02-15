# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareRateLimitChecked(BaseModel):
    """
    cloudflare_rate_limit_checked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareRateLimitExceeded(BaseModel):
    """
    cloudflare_rate_limit_exceeded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


