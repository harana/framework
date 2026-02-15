# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RowSelected(BaseModel):
    """
    row_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RowDeselected(BaseModel):
    """
    row_deselected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RowExpanded(BaseModel):
    """
    row_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RowCollapsed(BaseModel):
    """
    row_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ColumnSorted(BaseModel):
    """
    column_sorted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ColumnResized(BaseModel):
    """
    column_resized
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkSelectionToggled(BaseModel):
    """
    bulk_selection_toggled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectAllClicked(BaseModel):
    """
    select_all_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PaginationChanged(BaseModel):
    """
    pagination_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PageSizeChanged(BaseModel):
    """
    page_size_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RowClicked(BaseModel):
    """
    row_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CellEdited(BaseModel):
    """
    cell_edited
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


