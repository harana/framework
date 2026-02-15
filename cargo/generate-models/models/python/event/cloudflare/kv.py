# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareKvValueRead(BaseModel):
    """
    cloudflare_kv_value_read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareKvValueWritten(BaseModel):
    """
    cloudflare_kv_value_written
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareKvKeyDeleted(BaseModel):
    """
    cloudflare_kv_key_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareKvKeysListed(BaseModel):
    """
    cloudflare_kv_keys_listed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


