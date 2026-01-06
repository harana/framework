# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Form(BaseModel):
    """
    Form
    
    Class: form
    ID fields: slug
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    slug: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FormField(BaseModel):
    """
    Form Field
    
    Class: form_field
    ID fields: form_id, name
    """

    default_value: Optional[str] = None
    form_id: str
    is_required: bool = Field(default=False)
    label: str
    options: Optional[str] = None
    placeholder: Optional[str] = None
    sort_order: int = Field(default=0)
    type: str = Field(default="text")
    validation_rules: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class FormSubmission(BaseModel):
    """
    Form Submission
    
    Class: form_submission
    ID fields: form_id, created_at, submitted_by
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    data: str
    ip_address: Optional[str] = None
    status: str = Field(default="pending")
    submitted_by: Optional[str] = Field(default=None, description="Reference: User.id")
    user_agent: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


