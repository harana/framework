# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Zip(BaseModel):
    """
    zip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Unzip(BaseModel):
    """
    unzip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tar(BaseModel):
    """
    tar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Untar(BaseModel):
    """
    untar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Gzip(BaseModel):
    """
    gzip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Gunzip(BaseModel):
    """
    gunzip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class List(BaseModel):
    """
    list
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddToArchive(BaseModel):
    """
    add_to_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveFromArchive(BaseModel):
    """
    remove_from_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Bzip2(BaseModel):
    """
    bzip2
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Bunzip2(BaseModel):
    """
    bunzip2
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Xz(BaseModel):
    """
    xz
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Unxz(BaseModel):
    """
    unxz
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyArchive(BaseModel):
    """
    verify_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetArchiveInfo(BaseModel):
    """
    get_archive_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Archive(BaseModel):
    """
    archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ArchiveEntry(BaseModel):
    """
    archive_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


