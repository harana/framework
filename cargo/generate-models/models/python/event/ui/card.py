# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CardClicked(BaseModel):
    """
    card_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CardExpanded(BaseModel):
    """
    card_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CardCollapsed(BaseModel):
    """
    card_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CardActionClicked(BaseModel):
    """
    card_action_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CardDismissed(BaseModel):
    """
    card_dismissed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


