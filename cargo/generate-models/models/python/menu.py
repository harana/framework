# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MenuItem(BaseModel):
    """
    menu_item
    
    ID fields: menu_id, slug
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    href: Optional[str] = None
    icon: Optional[str] = None
    is_active: bool = Field(default=True)
    label: str
    menu_id: str
    order: int = Field(default=0)
    parent_id: Optional[str] = Field(default=None, description="Reference: menu_item.id")
    slug: str
    target: str = Field(default="self")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class MenuItemAction(BaseModel):
    """
    menu_item_action
    
    ID fields: menu_item_id, slug
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    href: Optional[str] = None
    icon: Optional[str] = None
    is_active: bool = Field(default=True)
    label: str
    menu_item_id: str = Field(description="Reference: menu_item.id")
    order: int = Field(default=0)
    slug: str
    target: str = Field(default="self")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


