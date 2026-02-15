# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Feed(BaseModel):
    """
    feed
    
    ID fields: slug
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    slug: str
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FeedItem(BaseModel):
    """
    feed_item
    
    ID fields: feed_id, created_at
    """

    actor_id: Optional[str] = Field(default=None, description="Reference: user.id")
    content: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    feed_id: str = Field(description="Reference: feed.id")
    is_complete: bool = Field(default=False)
    target_id: Optional[str] = None
    target_name: Optional[str] = None
    target_type: Optional[str] = None
    type: str = Field(default="action")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FeedItemTag(BaseModel):
    """
    feed_item_tag
    
    ID fields: feed_item_id, slug
    """

    color: str = Field(default="gray")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    feed_item_id: str = Field(description="Reference: feed_item.id")
    href: Optional[str] = None
    label: str
    order: int = Field(default=0)
    slug: str
    class Config:
        from_attributes = True
        populate_by_name = True


