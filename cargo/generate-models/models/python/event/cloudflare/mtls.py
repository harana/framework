# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareMtlsFetchInvoked(BaseModel):
    """
    cloudflare_mtls_fetch_invoked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareMtlsFetchFailed(BaseModel):
    """
    cloudflare_mtls_fetch_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


