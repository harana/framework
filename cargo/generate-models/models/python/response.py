# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ResponseTemplate(BaseModel):
    """
    response_template
    
    ID fields: id
    """

    body_template: Optional[str] = None
    content_type: str = Field(default="application/json")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    headers: Optional[str] = None
    is_active: bool = Field(default=True)
    name: str
    status_code: int = Field(default=200)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ResponseLog(BaseModel):
    """
    response_log
    
    ID fields: request_path, created_at
    """

    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    request_method: str
    request_path: str
    response_size: Optional[int] = None
    status_code: int
    template_id: Optional[str] = Field(default=None, description="Reference: response_template.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class BadGatewayOutput(BaseModel):
    """
    bad_gateway_output
    
    ID fields: id
    """

    status_code: int
    message: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class BadRequestOutput(BaseModel):
    """
    bad_request_output
    
    ID fields: id
    """

    status_code: int
    message: str
    errors: List[str]
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class BinaryOutput(BaseModel):
    """
    binary_output
    
    ID fields: id
    """

    data: str
    content_type: str
    filename: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class CorsPreflightOutput(BaseModel):
    """
    cors_preflight_output
    
    ID fields: id
    """

    status_code: int
    headers: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CsvOutput(BaseModel):
    """
    csv_output
    
    ID fields: id
    """

    data: str
    content_type: str
    filename: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteCookieOutput(BaseModel):
    """
    delete_cookie_output
    
    ID fields: id
    """

    set_cookie_header: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EmptyOutput(BaseModel):
    """
    empty_output
    
    ID fields: id
    """

    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ErrorOutput(BaseModel):
    """
    error_output
    
    ID fields: id
    """

    status_code: int
    message: str
    error_code: str
    details: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FileDownloadOutput(BaseModel):
    """
    file_download_output
    
    ID fields: id
    """

    data: str
    content_type: str
    filename: str
    content_disposition: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ForbiddenOutput(BaseModel):
    """
    forbidden_output
    
    ID fields: id
    """

    status_code: int
    message: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphqlOutput(BaseModel):
    """
    graphql_output
    
    ID fields: id
    """

    data: str
    errors: str
    extensions: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlOutput(BaseModel):
    """
    html_output
    
    ID fields: id
    """

    html: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class InternalServerErrorOutput(BaseModel):
    """
    internal_server_error_output
    
    ID fields: id
    """

    status_code: int
    message: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class JsonOutput(BaseModel):
    """
    json_output
    
    ID fields: id
    """

    data: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class NotFoundOutput(BaseModel):
    """
    not_found_output
    
    ID fields: id
    """

    status_code: int
    message: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PaginatedOutput(BaseModel):
    """
    paginated_output
    
    ID fields: id
    """

    data: List[str]
    total: int
    page: int
    page_size: int
    total_pages: int
    has_next: bool
    has_previous: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class RedirectOutput(BaseModel):
    """
    redirect_output
    
    ID fields: id
    """

    url: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class RenderTemplateOutput(BaseModel):
    """
    render_template_output
    
    ID fields: id
    """

    content: str
    content_type: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class RenderTemplateFileOutput(BaseModel):
    """
    render_template_file_output
    
    ID fields: id
    """

    content: str
    content_type: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ServiceUnavailableOutput(BaseModel):
    """
    service_unavailable_output
    
    ID fields: id
    """

    status_code: int
    message: str
    retry_after: int
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SetCookieOutput(BaseModel):
    """
    set_cookie_output
    
    ID fields: id
    """

    set_cookie_header: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SseEventOutput(BaseModel):
    """
    sse_event_output
    
    ID fields: id
    """

    event_string: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SseStreamOutput(BaseModel):
    """
    sse_stream_output
    
    ID fields: id
    """

    content_type: str
    stream: str
    class Config:
        from_attributes = True
        populate_by_name = True


class StreamingOutput(BaseModel):
    """
    streaming_output
    
    ID fields: id
    """

    content_type: str
    chunk_size: int
    class Config:
        from_attributes = True
        populate_by_name = True


class TextOutput(BaseModel):
    """
    text_output
    
    ID fields: id
    """

    text: str
    content_type: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


class UnauthorizedOutput(BaseModel):
    """
    unauthorized_output
    
    ID fields: id
    """

    status_code: int
    message: str
    www_authenticate: str
    body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlOutput(BaseModel):
    """
    xml_output
    
    ID fields: id
    """

    xml: str
    status_code: int
    class Config:
        from_attributes = True
        populate_by_name = True


