# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HttpResponseSent(BaseModel):
    """
    http response sent
    
    Class: event_route_response_sent
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteMatched(BaseModel):
    """
    route matched
    
    Class: event_route_matched
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteNotFound(BaseModel):
    """
    route not found
    
    Class: event_route_not_found
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RequestAuthenticated(BaseModel):
    """
    request authenticated
    
    Class: event_route_request_authenticated
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RequestRateLimited(BaseModel):
    """
    request rate limited
    
    Class: event_route_request_rate_limited
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


