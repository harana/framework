# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Insert(BaseModel):
    """
    insert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Upsert(BaseModel):
    """
    upsert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Query(BaseModel):
    """
    query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueryById(BaseModel):
    """
    query_by_id
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetByIds(BaseModel):
    """
    get_by_ids
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteByIds(BaseModel):
    """
    delete_by_ids
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Describe(BaseModel):
    """
    describe
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Info(BaseModel):
    """
    info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateMetadataIndex(BaseModel):
    """
    create_metadata_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMetadataIndex(BaseModel):
    """
    delete_metadata_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListMetadataIndexes(BaseModel):
    """
    list_metadata_indexes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


