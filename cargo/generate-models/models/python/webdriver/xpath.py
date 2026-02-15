# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class XPathEntry(BaseModel):
    """
    x_path_entry
    
    ID fields: id
    """

    description: str
    label: str
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathCollection(BaseModel):
    """
    x_path_collection
    
    ID fields: id
    """

    entry_count: int
    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathTextResult(BaseModel):
    """
    x_path_text_result
    
    ID fields: id
    """

    error: str
    found: bool
    texts: List[str]
    total: int
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathAttributeResult(BaseModel):
    """
    x_path_attribute_result
    
    ID fields: id
    """

    attribute: str
    error: str
    found: bool
    total: int
    values: List[str]
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathTestResult(BaseModel):
    """
    x_path_test_result
    
    ID fields: id
    """

    count: int
    found: bool
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathCountResult(BaseModel):
    """
    x_path_count_result
    
    ID fields: id
    """

    count: int
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathActionResult(BaseModel):
    """
    x_path_action_result
    
    ID fields: id
    """

    affected: int
    error: str
    success: bool
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathKeysInput(BaseModel):
    """
    x_path_keys_input
    
    ID fields: id
    """

    keys: str
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


