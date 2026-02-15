# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Validate(BaseModel):
    """
    validate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Submit(BaseModel):
    """
    submit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSubmission(BaseModel):
    """
    get_submission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListSubmissions(BaseModel):
    """
    list_submissions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateSubmission(BaseModel):
    """
    update_submission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSubmission(BaseModel):
    """
    delete_submission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExportSubmissions(BaseModel):
    """
    export_submissions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormSubmission(BaseModel):
    """
    form_submission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormData(BaseModel):
    """
    form_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormValidationError(BaseModel):
    """
    form_validation_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormField(BaseModel):
    """
    form_field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormMetadata(BaseModel):
    """
    form_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormValidationRules(BaseModel):
    """
    form_validation_rules
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FormSubmissionInfo(BaseModel):
    """
    form_submission_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


