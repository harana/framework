# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FormSubmitted(BaseModel):
    """
    form_submitted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormValidationFailed(BaseModel):
    """
    form_validation_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormReset(BaseModel):
    """
    form_reset
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


