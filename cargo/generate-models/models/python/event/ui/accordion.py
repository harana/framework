# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AccordionExpanded(BaseModel):
    """
    accordion_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AccordionCollapsed(BaseModel):
    """
    accordion_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AccordionSectionClicked(BaseModel):
    """
    accordion_section_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


