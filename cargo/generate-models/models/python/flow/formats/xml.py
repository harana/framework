# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Parse(BaseModel):
    """
    parse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Generate(BaseModel):
    """
    generate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Validate(BaseModel):
    """
    validate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XpathQuery(BaseModel):
    """
    xpath_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlDocument(BaseModel):
    """
    xml_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlObject(BaseModel):
    """
    xml_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlValidationError(BaseModel):
    """
    xml_validation_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlNamespaces(BaseModel):
    """
    xml_namespaces
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlNamespace(BaseModel):
    """
    xml_namespace
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


