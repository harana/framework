# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SearchIndex(BaseModel):
    """
    Search Index
    
    Class: search_index
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_active: bool = Field(default=True)
    settings: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchField(BaseModel):
    """
    Search Field
    
    Class: search_field
    ID fields: index_id, name
    """

    boost: float = Field(default="1.0")
    index_id: str
    is_facet: bool = Field(default=False)
    is_filterable: bool = Field(default=True)
    is_searchable: bool = Field(default=True)
    is_sortable: bool = Field(default=False)
    type: str = Field(default="text")
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchSynonym(BaseModel):
    """
    Search Synonym
    
    Class: search_synonym
    ID fields: index_id, term
    """

    index_id: str
    synonyms: str
    term: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchQueryLog(BaseModel):
    """
    Search Query Log
    
    Class: search_query_log
    ID fields: index_id, query, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    index_id: str
    query: str
    response_time_ms: Optional[int] = None
    results_count: int = Field(default=0)
    user_id: Optional[str] = Field(default=None, description="Reference: User.id")
    class Config:
        from_attributes = True
        populate_by_name = True


