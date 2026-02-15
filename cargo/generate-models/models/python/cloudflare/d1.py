# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareD1Database(BaseModel):
    """
    cloudflare_d1_database
    
    ID fields: account_id, database_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    database_id: Optional[str] = None
    database_name: str
    file_size: int = Field(default=0)
    num_tables: int = Field(default=0)
    region: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareD1QueryLog(BaseModel):
    """
    cloudflare_d1_query_log
    
    ID fields: database_id, executed_at
    """

    changes: int = Field(default=0)
    database_id: str = Field(description="Reference: cf_d1_database.id")
    duration_ms: Optional[float] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    last_row_id: Optional[int] = None
    rows_read: int = Field(default=0)
    rows_written: int = Field(default=0)
    sql: str
    status: str = Field(default="success")
    class Config:
        from_attributes = True
        populate_by_name = True


