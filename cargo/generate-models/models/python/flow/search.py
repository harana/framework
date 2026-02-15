# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SearchQuery(BaseModel):
    """
    search_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IndexDocument(BaseModel):
    """
    index_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkIndex(BaseModel):
    """
    bulk_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDocument(BaseModel):
    """
    get_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateDocument(BaseModel):
    """
    update_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDocument(BaseModel):
    """
    delete_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteByQuery(BaseModel):
    """
    delete_by_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateIndex(BaseModel):
    """
    create_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteIndex(BaseModel):
    """
    delete_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListIndexes(BaseModel):
    """
    list_indexes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetIndexStats(BaseModel):
    """
    get_index_stats
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Suggest(BaseModel):
    """
    suggest
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchDocument(BaseModel):
    """
    search_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchDocumentData(BaseModel):
    """
    search_document_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchFilters(BaseModel):
    """
    search_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchSortField(BaseModel):
    """
    search_sort_field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchHit(BaseModel):
    """
    search_hit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexError(BaseModel):
    """
    search_index_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchQuery(BaseModel):
    """
    search_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchMappings(BaseModel):
    """
    search_mappings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchFieldMapping(BaseModel):
    """
    search_field_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexSettings(BaseModel):
    """
    search_index_settings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchIndexInfo(BaseModel):
    """
    search_index_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


