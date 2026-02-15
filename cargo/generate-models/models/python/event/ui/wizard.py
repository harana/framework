# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class StepCompleted(BaseModel):
    """
    step_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepSkipped(BaseModel):
    """
    step_skipped
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StepNavigated(BaseModel):
    """
    step_navigated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WizardCompleted(BaseModel):
    """
    wizard_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WizardCancelled(BaseModel):
    """
    wizard_cancelled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


