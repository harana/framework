# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CfR2Bucket(BaseModel):
    """
    cf_r2_bucket
    
    ID fields: account_id, bucket_name
    """

    account_id: str
    bucket_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    location: Optional[str] = None
    storage_class: str = Field(default="standard")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CfR2Object(BaseModel):
    """
    cf_r2_object
    
    ID fields: bucket_id, key
    """

    bucket_id: str = Field(description="Reference: cf_r2_bucket.id")
    custom_metadata: Optional[str] = None
    etag: Optional[str] = None
    http_etag: Optional[str] = None
    key: str
    size: Optional[int] = None
    storage_class: str = Field(default="standard")
    uploaded: Optional[datetime] = None
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CfR2MultipartUpload(BaseModel):
    """
    cf_r2_multipart_upload
    
    ID fields: bucket_id, key, upload_id
    """

    bucket_id: str = Field(description="Reference: cf_r2_bucket.id")
    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    custom_metadata: Optional[str] = None
    key: str
    status: str = Field(default="initiated")
    storage_class: str = Field(default="standard")
    upload_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CfR2MultipartPart(BaseModel):
    """
    cf_r2_multipart_part
    
    ID fields: upload_id, part_number
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    etag: Optional[str] = None
    part_number: int
    size: Optional[int] = None
    upload_id: str = Field(description="Reference: cf_r2_multipart_upload.id")
    class Config:
        from_attributes = True
        populate_by_name = True


