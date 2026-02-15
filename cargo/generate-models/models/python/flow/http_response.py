# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RenderTemplate(BaseModel):
    """
    Render Template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RenderTemplateFromFile(BaseModel):
    """
    Render Template From File
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnJSONResponse(BaseModel):
    """
    Return JSON Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnPlainTextResponse(BaseModel):
    """
    Return Plain Text Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnHTMLResponse(BaseModel):
    """
    Return HTML Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnXMLResponse(BaseModel):
    """
    Return XML Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnBinaryResponse(BaseModel):
    """
    Return Binary Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StreamSSEEvent(BaseModel):
    """
    Stream SSE Event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StreamSSEEvents(BaseModel):
    """
    Stream SSE Events
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return404NotFound(BaseModel):
    """
    Return 404 Not Found
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return400BadRequest(BaseModel):
    """
    Return 400 Bad Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return401Unauthorized(BaseModel):
    """
    Return 401 Unauthorized
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return403Forbidden(BaseModel):
    """
    Return 403 Forbidden
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return500InternalServerError(BaseModel):
    """
    Return 500 Internal Server Error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return502BadGateway(BaseModel):
    """
    Return 502 Bad Gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Return503ServiceUnavailable(BaseModel):
    """
    Return 503 Service Unavailable
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnCustomErrorResponse(BaseModel):
    """
    Return Custom Error Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetResponseCookie(BaseModel):
    """
    Set Response Cookie
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteResponseCookie(BaseModel):
    """
    Delete Response Cookie
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnFileDownload(BaseModel):
    """
    Return File Download
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnStreamingResponse(BaseModel):
    """
    Return Streaming Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnEmptyResponse(BaseModel):
    """
    Return Empty Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnRedirectResponse(BaseModel):
    """
    Return Redirect Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnCORSPreflightResponse(BaseModel):
    """
    Return CORS Preflight Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnGraphQLResponse(BaseModel):
    """
    Return GraphQL Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnPaginatedResponse(BaseModel):
    """
    Return Paginated Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReturnCSVResponse(BaseModel):
    """
    Return CSV Response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HttpResponse(BaseModel):
    """
    HttpResponse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


