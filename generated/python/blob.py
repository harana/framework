# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class BlobStorage(BaseModel):
    """
    Blob Storage
    
    Class: blob_storage
    ID fields: name
    """

    access_key_id: Optional[str] = None
    allowed_mime_types: Optional[str] = None
    base_path: Optional[str] = None
    bucket: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    endpoint: Optional[str] = None
    is_active: bool = Field(default=True)
    max_file_size: int = Field(default=104857600)
    provider: str = Field(default="local")
    region: Optional[str] = None
    secret_access_key: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class BlobObject(BaseModel):
    """
    Blob Object
    
    Class: blob_object
    ID fields: storage_id, key
    """

    cdn_url: Optional[str] = None
    content_hash: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None
    file_name: str
    file_size: int
    is_encrypted: bool = Field(default=False)
    is_public: bool = Field(default=False)
    key: str
    metadata: Optional[str] = None
    mime_type: str
    storage_id: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    uploaded_by: Optional[str] = Field(default=None, description="Reference: User.id")
    url: Optional[str] = None
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class BlobAccessLog(BaseModel):
    """
    Blob Access Log
    
    Class: blob_access_log
    ID fields: object_id, accessed_at, accessed_by
    """

    accessed_at: datetime = Field(default_factory=datetime.utcnow)
    accessed_by: Optional[str] = Field(default=None, description="Reference: User.id")
    action: str = Field(default="view")
    bytes_transferred: Optional[int] = None
    ip_address: Optional[str] = None
    object_id: str
    status: str = Field(default="success")
    user_agent: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class BlobMultipartUpload(BaseModel):
    """
    Blob Multipart Upload
    
    Class: blob_multipart_upload
    ID fields: upload_id
    """

    completed_at: Optional[datetime] = None
    completed_parts: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None
    file_name: str
    key: str
    status: str = Field(default="initiated")
    storage_id: str
    total_parts: int
    total_size: Optional[int] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    upload_id: str
    uploaded_by: Optional[str] = Field(default=None, description="Reference: User.id")
    class Config:
        from_attributes = True
        populate_by_name = True


