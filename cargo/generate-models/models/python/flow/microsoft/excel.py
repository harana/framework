# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Read(BaseModel):
    """
    read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Write(BaseModel):
    """
    write
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSheets(BaseModel):
    """
    get_sheets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReadSheet(BaseModel):
    """
    read_sheet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExcelWorkbook(BaseModel):
    """
    excel_workbook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExcelRow(BaseModel):
    """
    excel_row
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


