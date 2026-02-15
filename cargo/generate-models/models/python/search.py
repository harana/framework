# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SearchIndex(BaseModel):
    """
    search_index
    
    ID fields: id
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
    search_field
    
    ID fields: index_id, name
    """

    boost: float = Field(default="1.0")
    index_id: str = Field(description="Reference: search_index.id")
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
    search_synonym
    
    ID fields: index_id, term
    """

    index_id: str = Field(description="Reference: search_index.id")
    synonyms: str
    term: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchQueryLog(BaseModel):
    """
    search_query_log
    
    ID fields: index_id, query, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    index_id: str = Field(description="Reference: search_index.id")
    query: str
    response_time_ms: Optional[int] = None
    results_count: int = Field(default=0)
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchDocument(BaseModel):
    """
    search_document
    
    ID fields: id
    """

    document_id: str
    index: str
    document: str
    score: float
    version: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchDocumentData(BaseModel):
    """
    search_document_data
    
    ID fields: id
    """

    fields: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchFilters(BaseModel):
    """
    search_filters
    
    ID fields: id
    """

    conditions: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchSortField(BaseModel):
    """
    search_sort_field
    
    ID fields: id
    """

    field: str
    order: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchHit(BaseModel):
    """
    search_hit
    
    ID fields: id
    """

    document_id: str
    score: float
    source: str
    highlight: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexError(BaseModel):
    """
    search_index_error
    
    ID fields: id
    """

    document_id: str
    error: str
    status: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchQuery(BaseModel):
    """
    search_query
    
    ID fields: id
    """

    query: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchMappings(BaseModel):
    """
    search_mappings
    
    ID fields: id
    """

    properties: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchFieldMapping(BaseModel):
    """
    search_field_mapping
    
    ID fields: id
    """

    type: str
    analyzer: str
    index: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexSettings(BaseModel):
    """
    search_index_settings
    
    ID fields: id
    """

    number_of_shards: int
    number_of_replicas: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexInfo(BaseModel):
    """
    search_index_info
    
    ID fields: id
    """

    name: str
    document_count: int
    size_bytes: int
    status: str
    class Config:
        from_attributes = True
        populate_by_name = True


