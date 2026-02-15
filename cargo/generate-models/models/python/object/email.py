# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class EmailTemplate(BaseModel):
    """
    email_template
    
    ID fields: id
    """

    body_html: str
    body_text: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_active: bool = Field(default=True)
    subject: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class Email(BaseModel):
    """
    email
    
    ID fields: from_address, to_address, created_at
    """

    bcc: Optional[str] = None
    body_html: Optional[str] = None
    body_text: Optional[str] = None
    cc: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    from_address: str
    reply_to: Optional[str] = None
    sent_at: Optional[datetime] = None
    status: str = Field(default="pending")
    subject: str
    template_id: Optional[str] = Field(default=None, description="Reference: email_template.id")
    to_address: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class EmailAttachment(BaseModel):
    """
    email_attachment
    
    ID fields: email_id, file_name
    """

    content_type: str
    email_id: str = Field(description="Reference: email.id")
    class Config:
        from_attributes = True
        populate_by_name = True


