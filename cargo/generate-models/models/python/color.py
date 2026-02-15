# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Color(BaseModel):
    """
    color
    
    ID fields: hex
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    format: str = Field(default="hex")
    hex: str
    label: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ColorPalette(BaseModel):
    """
    color_palette
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    palette_type: str = Field(default="custom")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ColorPaletteEntry(BaseModel):
    """
    color_palette_entry
    
    ID fields: palette_id, color_id
    """

    color_id: str = Field(description="Reference: color.id")
    palette_id: str = Field(description="Reference: color_palette.id")
    sort_order: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class RgbColor(BaseModel):
    """
    rgb_color
    
    ID fields: id
    """

    r: int
    g: int
    b: int
    class Config:
        from_attributes = True
        populate_by_name = True


class HslColor(BaseModel):
    """
    hsl_color
    
    ID fields: id
    """

    h: int
    s: float
    l: float
    class Config:
        from_attributes = True
        populate_by_name = True


class HsvColor(BaseModel):
    """
    hsv_color
    
    ID fields: id
    """

    h: int
    s: float
    v: float
    class Config:
        from_attributes = True
        populate_by_name = True


class CmykColor(BaseModel):
    """
    cmyk_color
    
    ID fields: id
    """

    c: float
    m: float
    y: float
    k: float
    class Config:
        from_attributes = True
        populate_by_name = True


class ColorComponents(BaseModel):
    """
    color_components
    
    ID fields: id
    """

    red: int
    green: int
    blue: int
    alpha: float
    class Config:
        from_attributes = True
        populate_by_name = True


