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


class MongoDocument(BaseModel):
    """
    mongo_document
    
    ID fields: id
    """

    _id: str
    collection: str
    database: str
    document: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoDocumentData(BaseModel):
    """
    mongo_document_data
    
    ID fields: id
    """

    data: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoFilter(BaseModel):
    """
    mongo_filter
    
    ID fields: id
    """

    conditions: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoProjection(BaseModel):
    """
    mongo_projection
    
    ID fields: id
    """

    fields: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoSort(BaseModel):
    """
    mongo_sort
    
    ID fields: id
    """

    fields: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoUpdate(BaseModel):
    """
    mongo_update
    
    ID fields: id
    """

    operators: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoPipelineStage(BaseModel):
    """
    mongo_pipeline_stage
    
    ID fields: id
    """

    stage: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoIndexKeys(BaseModel):
    """
    mongo_index_keys
    
    ID fields: id
    """

    fields: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MongoBulkOperation(BaseModel):
    """
    mongo_bulk_operation
    
    ID fields: id
    """

    operation_type: str
    filter: str
    document: str
    update: str
    class Config:
        from_attributes = True
        populate_by_name = True


