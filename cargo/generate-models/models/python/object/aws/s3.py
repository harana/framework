# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsS3Bucket(BaseModel):
    """
    aws_s3_bucket
    
    ID fields: account_id, bucket_name
    """

    account_id: str
    acl: str = Field(default="private")
    bucket_name: str
    cors_enabled: bool = Field(default=False)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    encryption_algorithm: str = Field(default="none")
    is_versioned: bool = Field(default=False)
    kms_key_id: Optional[str] = None
    lifecycle_rules_count: int = Field(default=0)
    location: Optional[str] = None
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsS3Object(BaseModel):
    """
    aws_s3_object
    
    ID fields: bucket_id, key
    """

    bucket_id: str = Field(description="Reference: aws_s3_bucket.id")
    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    etag: Optional[str] = None
    is_delete_marker: bool = Field(default=False)
    key: str
    last_modified: Optional[datetime] = None
    metadata: Optional[str] = None
    size: Optional[int] = None
    storage_class: str = Field(default="standard")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsS3BucketPolicy(BaseModel):
    """
    aws_s3_bucket_policy
    
    ID fields: bucket_id
    """

    bucket_id: str = Field(description="Reference: aws_s3_bucket.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    policy_document: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsS3MultipartUpload(BaseModel):
    """
    aws_s3_multipart_upload
    
    ID fields: bucket_id, key, upload_id
    """

    bucket_id: str = Field(description="Reference: aws_s3_bucket.id")
    completed_at: Optional[datetime] = None
    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    key: str
    metadata: Optional[str] = None
    status: str = Field(default="initiated")
    storage_class: Optional[str] = None
    upload_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


