# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Resize(BaseModel):
    """
    resize
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Crop(BaseModel):
    """
    crop
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Compress(BaseModel):
    """
    compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Rotate(BaseModel):
    """
    rotate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Thumbnail(BaseModel):
    """
    thumbnail
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Blur(BaseModel):
    """
    blur
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Greyscale(BaseModel):
    """
    greyscale
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Flip(BaseModel):
    """
    flip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Sharpen(BaseModel):
    """
    sharpen
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Brightness(BaseModel):
    """
    brightness
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Contrast(BaseModel):
    """
    contrast
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tint(BaseModel):
    """
    tint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetMetadata(BaseModel):
    """
    get_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ConvertFormat(BaseModel):
    """
    convert_format
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Watermark(BaseModel):
    """
    watermark
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Image(BaseModel):
    """
    image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


