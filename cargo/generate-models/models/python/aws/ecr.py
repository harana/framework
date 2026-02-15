# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsEcrRepository(BaseModel):
    """
    aws_ecr_repository
    
    ID fields: account_id, repository_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    encryption_type: str = Field(default="aes256")
    image_tag_mutability: str = Field(default="mutable")
    kms_key: Optional[str] = None
    region: Optional[str] = None
    registry_id: Optional[str] = None
    repository_arn: Optional[str] = None
    repository_name: str
    repository_uri: Optional[str] = None
    scan_on_push: bool = Field(default=False)
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcrImage(BaseModel):
    """
    aws_ecr_image
    
    ID fields: repository_id, image_digest
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    image_digest: str
    image_manifest_media_type: Optional[str] = None
    image_pushed_at: Optional[datetime] = None
    image_size_bytes: Optional[int] = None
    image_tags: Optional[str] = None
    last_scan_at: Optional[datetime] = None
    repository_id: str = Field(description="Reference: aws_ecr_repository.id")
    scan_status: str = Field(default="in_progress")
    vulnerability_count: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcrRepositoryPolicy(BaseModel):
    """
    aws_ecr_repository_policy
    
    ID fields: repository_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    policy_text: str
    repository_id: str = Field(description="Reference: aws_ecr_repository.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcrLifecyclePolicy(BaseModel):
    """
    aws_ecr_lifecycle_policy
    
    ID fields: repository_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    last_evaluated_at: Optional[datetime] = None
    lifecycle_policy_text: str
    repository_id: str = Field(description="Reference: aws_ecr_repository.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcrImageScanFinding(BaseModel):
    """
    aws_ecr_image_scan_finding
    
    ID fields: image_id, name, uri
    """

    description: Optional[str] = None
    image_id: str = Field(description="Reference: aws_ecr_image.id")
    name: str
    severity: str = Field(default="undefined")
    uri: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


