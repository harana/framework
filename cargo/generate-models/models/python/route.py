# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    auth: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    download: Optional[str] = None
    handler: str = Field(default="handler")
    is_active: bool = Field(default=True)
    method: str = Field(default="get")
    middleware: Optional[str] = None
    path: str
    rate_limit: Optional[int] = None
    sse: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    upload: Optional[str] = None
    websocket: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroup(BaseModel):
    """
    route_group
    
    ID fields: id
    """

    middleware: Optional[str] = None
    prefix: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteGroupAssignment(BaseModel):
    """
    route_group_assignment
    
    ID fields: route_id, group_id
    """

    group_id: str = Field(description="Reference: route_group.id")
    route_id: str = Field(description="Reference: route.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteAuthOptions(BaseModel):
    """
    route_auth_options
    
    ID fields: id
    """

    allow_anonymous_fallback: bool = Field(default=False)
    auth_required: bool = Field(default=True)
    auth_type: str = Field(default="any")
    max_token_age: Optional[int] = None
    required_permissions: Optional[str] = None
    required_roles: Optional[str] = None
    required_scopes: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteUploadOptions(BaseModel):
    """
    route_upload_options
    
    ID fields: id
    """

    allowed_extensions: Optional[str] = None
    allowed_mime_types: Optional[str] = None
    is_public: bool = Field(default=False)
    max_file_size: int = Field(default=104857600)
    max_files: int = Field(default=10)
    overwrite_existing: bool = Field(default=False)
    storage_id: str = Field(description="Reference: blob_storage.id")
    storage_path: Optional[str] = None
    strip_exif: bool = Field(default=False)
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteDownloadOptions(BaseModel):
    """
    route_download_options
    
    ID fields: id
    """

    cache_control: Optional[str] = None
    content_disposition: str = Field(default="attachment")
    require_signed_url: bool = Field(default=False)
    signed_url_ttl: int = Field(default=3600)
    storage_id: str = Field(description="Reference: blob_storage.id")
    storage_path: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteWebsocketOptions(BaseModel):
    """
    route_websocket_options
    
    ID fields: id
    """

    allowed_origins: Optional[str] = None
    heartbeat_interval: int = Field(default=30)
    idle_timeout: int = Field(default=300)
    max_connections: Optional[int] = None
    max_frame_size: int = Field(default=65536)
    max_message_size: int = Field(default=1048576)
    protocol: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteSseOptions(BaseModel):
    """
    route_sse_options
    
    ID fields: id
    """

    channel: Optional[str] = None
    heartbeat_interval: int = Field(default=15)
    idle_timeout: int = Field(default=300)
    max_connections: Optional[int] = None
    retry_timeout: int = Field(default=3000)
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTransferLog(BaseModel):
    """
    route_transfer_log
    
    ID fields: route_id, created_at, user_id
    """

    blob_key: str
    bytes_transferred: int
    created_at: datetime = Field(default_factory=datetime.utcnow)
    direction: str = Field(default="upload")
    duration_ms: Optional[int] = None
    file_name: Optional[str] = None
    ip_address: Optional[str] = None
    mime_type: Optional[str] = None
    route_id: str = Field(description="Reference: route.id")
    status: str = Field(default="success")
    user_agent: Optional[str] = None
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


