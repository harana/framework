# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareD1QueryExecuted(BaseModel):
    """
    cloudflare_d1_query_executed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareD1QueryFailed(BaseModel):
    """
    cloudflare_d1_query_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareD1BatchExecuted(BaseModel):
    """
    cloudflare_d1_batch_executed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareD1DatabaseDumped(BaseModel):
    """
    cloudflare_d1_database_dumped
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


