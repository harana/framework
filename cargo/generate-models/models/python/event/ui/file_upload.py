# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FileSelected(BaseModel):
    """
    file_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileRemoved(BaseModel):
    """
    file_removed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileUploadStarted(BaseModel):
    """
    file_upload_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileUploadCompleted(BaseModel):
    """
    file_upload_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileUploadFailed(BaseModel):
    """
    file_upload_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DragEntered(BaseModel):
    """
    drag_entered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DragLeft(BaseModel):
    """
    drag_left
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileDropped(BaseModel):
    """
    file_dropped
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


