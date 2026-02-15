# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Redirect(BaseModel):
    """
    redirect
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Forward(BaseModel):
    """
    forward
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RewritePath(BaseModel):
    """
    rewrite_path
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddHeaders(BaseModel):
    """
    add_headers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveHeaders(BaseModel):
    """
    remove_headers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetStatus(BaseModel):
    """
    set_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonResponse(BaseModel):
    """
    json_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlResponse(BaseModel):
    """
    html_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RateLimit(BaseModel):
    """
    rate_limit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MatchRoute(BaseModel):
    """
    match_route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ProxyWebsocket(BaseModel):
    """
    proxy_websocket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


