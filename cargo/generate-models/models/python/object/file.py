# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class File(BaseModel):
    """
    file
    
    ID fields: full_path
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    path: str
    full_path: str
    parent_id: Optional[str] = Field(default=None, description="Reference: file.id")
    type: str = Field(default="file")
    extension: Optional[str] = None
    mime_type: Optional[str] = None
    size: int = Field(default=0)
    content_hash: Optional[str] = None
    is_hidden: bool = Field(default=False)
    is_system: bool = Field(default=False)
    is_readonly: bool = Field(default=False)
    is_archived: bool = Field(default=False)
    permissions: Optional[str] = None
    owner: Optional[str] = None
    group: Optional[str] = None
    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    modified_at: Optional[datetime] = None
    modified_by: Optional[str] = Field(default=None, description="Reference: user.id")
    accessed_at: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FileVersion(BaseModel):
    """
    file_version
    
    ID fields: file_id, version_number
    """

    file_id: str = Field(description="Reference: file.id")
    version_number: int
    size: int
    content_hash: str
    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_current: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class FileMetadata(BaseModel):
    """
    file_metadata
    
    ID fields: file_id, key
    """

    file_id: str = Field(description="Reference: file.id")
    key: str
    value: str
    value_type: str = Field(default="string")
    source: str = Field(default="user")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FileTag(BaseModel):
    """
    file_tag
    
    ID fields: id
    """

    color: Optional[str] = None
    description: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class FileTagAssociation(BaseModel):
    """
    file_tag_association
    
    ID fields: file_id, tag_id
    """

    file_id: str = Field(description="Reference: file.id")
    tag_id: str = Field(description="Reference: file_tag.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class FileShare(BaseModel):
    """
    file_share
    
    ID fields: share_token
    """

    file_id: str = Field(description="Reference: file.id")
    shared_by: str = Field(description="Reference: user.id")
    shared_with: Optional[str] = Field(default=None, description="Reference: user.id")
    share_token: str
    permissions: str = Field(default="read")
    password: Optional[str] = None
    download_limit: Optional[int] = None
    download_count: int = Field(default=0)
    expires_at: Optional[datetime] = None
    is_active: bool = Field(default=True)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    accessed_at: Optional[datetime] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class FileOperation(BaseModel):
    """
    file_operation
    
    ID fields: file_id, operation, performed_at
    """

    file_id: str = Field(description="Reference: file.id")
    operation: str = Field(default="read")
    status: str = Field(default="pending")
    source_path: Optional[str] = None
    destination_path: Optional[str] = None
    bytes_processed: int = Field(default=0)
    total_bytes: Optional[int] = None
    progress_percent: int = Field(default=0)
    error_message: Optional[str] = None
    performed_at: datetime = Field(default_factory=datetime.utcnow)
    performed_by: Optional[str] = Field(default=None, description="Reference: user.id")
    completed_at: Optional[datetime] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class FileFormat(BaseModel):
    """
    file_format
    
    ID fields: format_type
    """

    format_type: str = Field(default="txt")
    encoding: Optional[str] = None
    page_count: Optional[int] = None
    word_count: Optional[int] = None
    line_count: Optional[int] = None
    column_count: Optional[int] = None
    row_count: Optional[int] = None
    has_macros: bool = Field(default=False)
    is_encrypted: bool = Field(default=False)
    language: Optional[str] = None
    author: Optional[str] = None
    title: Optional[str] = None
    subject: Optional[str] = None
    keywords: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FileIndex(BaseModel):
    """
    file_index
    
    ID fields: file_id, index_type
    """

    file_id: str = Field(description="Reference: file.id")
    index_type: str = Field(default="fulltext")
    content: Optional[str] = None
    content_vector: Optional[str] = None
    language: Optional[str] = None
    indexed_at: datetime = Field(default_factory=datetime.utcnow)
    index_version: Optional[str] = None
    is_current: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class FileWatch(BaseModel):
    """
    file_watch
    
    ID fields: path, pattern, watch_type
    """

    path: str
    pattern: Optional[str] = None
    watch_type: str = Field(default="all")
    is_recursive: bool = Field(default=True)
    is_active: bool = Field(default=True)
    callback_url: Optional[str] = None
    callback_event: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    last_triggered_at: Optional[datetime] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class FileCrawl(BaseModel):
    """
    file_crawl
    
    ID fields: root_path, started_at
    """

    root_path: str
    pattern: Optional[str] = None
    exclude_pattern: Optional[str] = None
    max_depth: Optional[int] = None
    max_files: Optional[int] = None
    follow_symlinks: bool = Field(default=False)
    status: str = Field(default="pending")
    files_found: int = Field(default=0)
    files_processed: int = Field(default=0)
    total_size: int = Field(default=0)
    error_message: Optional[str] = None
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


