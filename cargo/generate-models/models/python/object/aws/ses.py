# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsSesIdentity(BaseModel):
    """
    aws_ses_identity
    
    ID fields: account_id, identity
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dkim_enabled: bool = Field(default=False)
    feedback_forwarding_enabled: bool = Field(default=True)
    identity: str
    identity_type: str
    region: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    verification_status: str = Field(default="not_started")
    verification_token: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSesTemplate(BaseModel):
    """
    aws_ses_template
    
    ID fields: account_id, template_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    html_part: Optional[str] = None
    region: Optional[str] = None
    subject_part: str
    template_name: str
    text_part: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSesConfigurationSet(BaseModel):
    """
    aws_ses_configuration_set
    
    ID fields: account_id, configuration_set_name
    """

    account_id: str
    configuration_set_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    region: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSesSendLog(BaseModel):
    """
    aws_ses_send_log
    
    ID fields: identity_id, message_id
    """

    bcc_addresses: Optional[str] = None
    cc_addresses: Optional[str] = None
    configuration_set_name: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    from_address: str
    identity_id: Optional[str] = Field(default=None, description="Reference: aws_ses_identity.id")
    message_id: str
    status: str = Field(default="queued")
    subject: Optional[str] = None
    template_name: Optional[str] = None
    to_addresses: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


