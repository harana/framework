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


class Transform(BaseModel):
    """
    transform
    
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


class CsvDocument(BaseModel):
    """
    csv_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvRow(BaseModel):
    """
    csv_row
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvTransformOperation(BaseModel):
    """
    csv_transform_operation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvSchema(BaseModel):
    """
    csv_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvColumnSchema(BaseModel):
    """
    csv_column_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvValidationError(BaseModel):
    """
    csv_validation_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


