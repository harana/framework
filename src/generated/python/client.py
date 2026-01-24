# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ClientConnection(BaseModel):
    """
    Client Connection
    
    Class: client_connection
    ID fields: name
    """

    client_type: str = Field(default="http")
    connection_retry_attempts: int = Field(default=3)
    connection_retry_delay_ms: int = Field(default=1000)
    connection_string: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    database: Optional[str] = None
    host: str
    is_active: bool = Field(default=True)
    max_connections: int = Field(default=10)
    metadata: Optional[str] = None
    min_connections: int = Field(default=1)
    password: Optional[str] = None
    port: int
    ssl_cert_path: Optional[str] = None
    timeout_seconds: int = Field(default=30)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    use_ssl: bool = Field(default=False)
    username: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ClientPool(BaseModel):
    """
    Client Pool
    
    Class: client_pool
    ID fields: name
    """

    active_connections: int = Field(default=0)
    connection_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    idle_connections: int = Field(default=0)
    idle_timeout_seconds: int = Field(default=300)
    is_healthy: bool = Field(default=True)
    max_wait_time_ms: int = Field(default=5000)
    pool_size: int
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    wait_queue_size: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class ClientQuery(BaseModel):
    """
    Client Query
    
    Class: client_query
    ID fields: connection_id, executed_at, query_hash
    """

    bytes_returned: Optional[int] = None
    connection_id: str
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    executed_by: Optional[str] = Field(default=None, description="Reference: User.id")
    parameters: Optional[str] = None
    query_hash: Optional[str] = None
    query_text: Optional[str] = None
    query_type: str = Field(default="select")
    rows_affected: Optional[int] = None
    status: str = Field(default="success")
    class Config:
        from_attributes = True
        populate_by_name = True


class ClientHealthCheck(BaseModel):
    """
    Client Health Check
    
    Class: client_health_check
    ID fields: connection_id, checked_at
    """

    checked_at: datetime = Field(default_factory=datetime.utcnow)
    connection_id: str
    error_message: Optional[str] = None
    is_healthy: bool = Field(default=True)
    metadata: Optional[str] = None
    response_time_ms: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ClientConfiguration(BaseModel):
    """
    Client Configuration
    
    Class: client_config
    ID fields: connection_id, key
    """

    connection_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_encrypted: bool = Field(default=False)
    is_required: bool = Field(default=False)
    key: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


