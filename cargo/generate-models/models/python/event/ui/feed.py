# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FeedItemLiked(BaseModel):
    """
    feed_item_liked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FeedItemCommented(BaseModel):
    """
    feed_item_commented
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FeedItemShared(BaseModel):
    """
    feed_item_shared
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FeedScrolled(BaseModel):
    """
    feed_scrolled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LoadMoreClicked(BaseModel):
    """
    load_more_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


