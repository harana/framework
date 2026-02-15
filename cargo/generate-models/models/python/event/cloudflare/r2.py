# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareR2ObjectUploaded(BaseModel):
    """
    cloudflare_r2_object_uploaded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2ObjectDownloaded(BaseModel):
    """
    cloudflare_r2_object_downloaded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2ObjectDeleted(BaseModel):
    """
    cloudflare_r2_object_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2ObjectsDeleted(BaseModel):
    """
    cloudflare_r2_objects_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2ObjectsListed(BaseModel):
    """
    cloudflare_r2_objects_listed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2MultipartUploadStarted(BaseModel):
    """
    cloudflare_r2_multipart_upload_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2MultipartUploadCompleted(BaseModel):
    """
    cloudflare_r2_multipart_upload_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareR2MultipartUploadAborted(BaseModel):
    """
    cloudflare_r2_multipart_upload_aborted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


