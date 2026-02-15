# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MongodbConnection(BaseModel):
    """
    mongodb_connection
    
    ID fields: id
    """

    connection_string: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    database: str
    host: str
    is_active: bool = Field(default=True)
    port: int = Field(default=27017)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    use_ssl: bool = Field(default=False)
    class Config:
        from_attributes = True
        populate_by_name = True


class MongodbCollection(BaseModel):
    """
    mongodb_collection
    
    ID fields: connection_id, name
    """

    connection_id: str = Field(description="Reference: mongodb_connection.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    document_count: int = Field(default=0)
    name: str
    size: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class MongodbIndex(BaseModel):
    """
    mongodb_index
    
    ID fields: collection_id, name
    """

    collection_id: str = Field(description="Reference: mongodb_collection.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_unique: bool = Field(default=False)
    keys: str
    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongodbQueryLog(BaseModel):
    """
    mongodb_query_log
    
    ID fields: collection_id, executed_at
    """

    collection_id: str = Field(description="Reference: mongodb_collection.id")
    documents_affected: int = Field(default=0)
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    operation: str = Field(default="find")
    status: str = Field(default="success")
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


