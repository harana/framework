# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareVectorsInserted(BaseModel):
    """
    cloudflare_vectors_inserted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorsUpserted(BaseModel):
    """
    cloudflare_vectors_upserted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorsQueried(BaseModel):
    """
    cloudflare_vectors_queried
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorsDeleted(BaseModel):
    """
    cloudflare_vectors_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorizeMetadataIndexCreated(BaseModel):
    """
    cloudflare_vectorize_metadata_index_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareVectorizeMetadataIndexDeleted(BaseModel):
    """
    cloudflare_vectorize_metadata_index_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


