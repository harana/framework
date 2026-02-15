# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployS3(BaseModel):
    """
    deploy_s3
    
    ID fields: id
    """

    acl: Optional[str] = None
    bucket: str
    cache_control: Optional[str] = None
    cloudfront: Optional[str] = None
    content_encoding: Optional[str] = None
    credentials_env: Optional[str] = None
    destination_prefix: Optional[str] = None
    enabled: bool = Field(default=True)
    metadata: str
    region: str
    source_dir: str
    sync: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployS3Cloudfront(BaseModel):
    """
    deploy_s3_cloudfront
    
    ID fields: id
    """

    distribution_id: str
    enabled: bool = Field(default=True)
    invalidate_paths: List[str]
    wait_for_invalidation: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployS3Sync(BaseModel):
    """
    deploy_s3_sync
    
    ID fields: id
    """

    delete: bool = Field(default=True)
    exclude: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployAwsCredentials(BaseModel):
    """
    deploy_aws_credentials
    
    ID fields: id
    """

    aws_access_key_id: str
    aws_secret_access_key: str
    class Config:
        from_attributes = True
        populate_by_name = True


