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


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Copy(BaseModel):
    """
    copy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Move(BaseModel):
    """
    move
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Exists(BaseModel):
    """
    exists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Info(BaseModel):
    """
    info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListDirectory(BaseModel):
    """
    list_directory
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDirectory(BaseModel):
    """
    create_directory
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDirectory(BaseModel):
    """
    delete_directory
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class File(BaseModel):
    """
    file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


