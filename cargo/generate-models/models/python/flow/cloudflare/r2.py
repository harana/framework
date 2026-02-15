# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Head(BaseModel):
    """
    head
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Put(BaseModel):
    """
    put
    
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


class DeleteMany(BaseModel):
    """
    delete_many
    
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


class CreateMultipartUpload(BaseModel):
    """
    create_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UploadPart(BaseModel):
    """
    upload_part
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CompleteMultipartUpload(BaseModel):
    """
    complete_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AbortMultipartUpload(BaseModel):
    """
    abort_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


