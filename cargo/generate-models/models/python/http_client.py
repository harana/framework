# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HttpClient(BaseModel):
    """
    http_client
    
    ID fields: id
    """

    base_url: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_headers: Optional[str] = None
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    max_retries: int = Field(default=3)
    timeout_seconds: int = Field(default=30)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class HttpRequestLog(BaseModel):
    """
    http_request_log
    
    ID fields: client_id, created_at
    """

    client_id: Optional[str] = Field(default=None, description="Reference: http_client.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    method: str = Field(default="get")
    request_body: Optional[str] = None
    request_headers: Optional[str] = None
    response_body: Optional[str] = None
    response_headers: Optional[str] = None
    status_code: Optional[int] = None
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HttpRequest(BaseModel):
    """
    http_request
    
    ID fields: id
    """

    url: str
    method: str
    headers: str
    query_params: str
    body: str
    timeout: int
    class Config:
        from_attributes = True
        populate_by_name = True


class Headers(BaseModel):
    """
    headers
    
    ID fields: id
    """

    content_type: str
    authorization: str
    accept: str
    user_agent: str
    class Config:
        from_attributes = True
        populate_by_name = True


class QueryParams(BaseModel):
    """
    query_params
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Variables(BaseModel):
    """
    variables
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphQlError(BaseModel):
    """
    graph_ql_error
    
    ID fields: id
    """

    message: str
    path: List[str]
    locations: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphQlErrorLocation(BaseModel):
    """
    graph_ql_error_location
    
    ID fields: id
    """

    line: int
    column: int
    class Config:
        from_attributes = True
        populate_by_name = True


