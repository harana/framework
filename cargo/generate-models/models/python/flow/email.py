# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Send(BaseModel):
    """
    send
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendTemplate(BaseModel):
    """
    send_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidateEmail(BaseModel):
    """
    validate_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Status(BaseModel):
    """
    status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendBulk(BaseModel):
    """
    send_bulk
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTemplate(BaseModel):
    """
    create_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTemplate(BaseModel):
    """
    delete_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTemplates(BaseModel):
    """
    list_templates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmailMessage(BaseModel):
    """
    email_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmailAttachment(BaseModel):
    """
    email_attachment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmailRecipient(BaseModel):
    """
    email_recipient
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmailTemplate(BaseModel):
    """
    email_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


