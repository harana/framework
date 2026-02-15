# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class UploadBlob(BaseModel):
    """
    upload_blob
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DownloadBlob(BaseModel):
    """
    download_blob
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteBlob(BaseModel):
    """
    delete_blob
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListBlobs(BaseModel):
    """
    list_blobs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Exists(BaseModel):
    """
    exists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBlobMetadata(BaseModel):
    """
    get_blob_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopyBlob(BaseModel):
    """
    copy_blob
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeneratePresignedUrl(BaseModel):
    """
    generate_presigned_url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Blob(BaseModel):
    """
    blob
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BlobMetadata(BaseModel):
    """
    blob_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BlobInfo(BaseModel):
    """
    blob_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


