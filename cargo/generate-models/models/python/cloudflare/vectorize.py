# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareVectorizeIndex(BaseModel):
    """
    cloudflare_vectorize_index
    
    ID fields: account_id, index_name
    """

    account_id: str
    binding: str
    configured_dimensions: int
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    index_name: str
    metric: str = Field(default="cosine")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vectors_count: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorizeVector(BaseModel):
    """
    cloudflare_vectorize_vector
    
    ID fields: index_id, vector_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    index_id: str = Field(description="Reference: cf_vectorize_index.id")
    metadata: Optional[str] = None
    namespace: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    values: Optional[str] = None
    vector_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorizeMetadataIndex(BaseModel):
    """
    cloudflare_vectorize_metadata_index
    
    ID fields: index_id, property_name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    index_id: str = Field(description="Reference: cf_vectorize_index.id")
    property_name: str
    type: str = Field(default="string")
    class Config:
        from_attributes = True
        populate_by_name = True


