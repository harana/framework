# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Post(BaseModel):
    """
    post
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Put(BaseModel):
    """
    put
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Patch(BaseModel):
    """
    patch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Download(BaseModel):
    """
    download
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Upload(BaseModel):
    """
    upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphqlQuery(BaseModel):
    """
    graphql_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HttpRequest(BaseModel):
    """
    http_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Headers(BaseModel):
    """
    headers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueryParams(BaseModel):
    """
    query_params
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Variables(BaseModel):
    """
    variables
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphQlError(BaseModel):
    """
    graph_ql_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphQlErrorLocation(BaseModel):
    """
    graph_ql_error_location
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


