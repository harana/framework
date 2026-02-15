# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SearchInitiated(BaseModel):
    """
    search_initiated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchCleared(BaseModel):
    """
    search_cleared
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchResultsShown(BaseModel):
    """
    search_results_shown
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchResultSelected(BaseModel):
    """
    search_result_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchEmptyState(BaseModel):
    """
    search_empty_state
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


