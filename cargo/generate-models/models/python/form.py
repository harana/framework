# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Form(BaseModel):
    """
    form
    
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
    form_field
    
    ID fields: form_id, name
    """

    default_value: Optional[str] = None
    form_id: str = Field(description="Reference: form.id")
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
    form_submission
    
    ID fields: form_id, created_at, submitted_by
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    data: str
    ip_address: Optional[str] = None
    status: str = Field(default="pending")
    submitted_by: Optional[str] = Field(default=None, description="Reference: user.id")
    user_agent: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class FormData(BaseModel):
    """
    form_data
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FormValidationError(BaseModel):
    """
    form_validation_error
    
    ID fields: id
    """

    field: str
    message: str
    code: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FormMetadata(BaseModel):
    """
    form_metadata
    
    ID fields: id
    """

    title: str
    description: str
    version: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FormValidationRules(BaseModel):
    """
    form_validation_rules
    
    ID fields: id
    """

    rules: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FormSubmissionInfo(BaseModel):
    """
    form_submission_info
    
    ID fields: id
    """

    submission_id: str
    form_id: str
    status: str
    submitted_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


