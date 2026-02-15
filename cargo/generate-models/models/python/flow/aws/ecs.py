# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateRepository(BaseModel):
    """
    create_repository
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRepository(BaseModel):
    """
    delete_repository
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeRepositories(BaseModel):
    """
    describe_repositories
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListImages(BaseModel):
    """
    list_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeImages(BaseModel):
    """
    describe_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutImage(BaseModel):
    """
    put_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BatchDeleteImage(BaseModel):
    """
    batch_delete_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAuthToken(BaseModel):
    """
    get_auth_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDownloadUrl(BaseModel):
    """
    get_download_url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TagImage(BaseModel):
    """
    tag_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetRepositoryPolicy(BaseModel):
    """
    set_repository_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRepositoryPolicy(BaseModel):
    """
    get_repository_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRepositoryPolicy(BaseModel):
    """
    delete_repository_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartImageScan(BaseModel):
    """
    start_image_scan
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeScanFindings(BaseModel):
    """
    describe_scan_findings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutLifecyclePolicy(BaseModel):
    """
    put_lifecycle_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetLifecyclePolicy(BaseModel):
    """
    get_lifecycle_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


