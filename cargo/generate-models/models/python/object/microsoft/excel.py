# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ExcelWorkbook(BaseModel):
    """
    excel_workbook
    
    ID fields: id
    """

    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    file_path: Optional[str] = None
    sheet_count: int = Field(default=0)
    size: int = Field(default=0)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ExcelSheet(BaseModel):
    """
    excel_sheet
    
    ID fields: workbook_id, name
    """

    column_count: int = Field(default=0)
    name: str
    row_count: int = Field(default=0)
    workbook_id: str = Field(description="Reference: excel_workbook.id")
    class Config:
        from_attributes = True
        populate_by_name = True


