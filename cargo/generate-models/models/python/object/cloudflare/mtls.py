# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CfMtlsCertificate(BaseModel):
    """
    cf_mtls_certificate
    
    ID fields: account_id, certificate_name
    """

    account_id: str
    binding: str
    certificate_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None
    issuer: Optional[str] = None
    serial_number: Optional[str] = None
    status: str = Field(default="active")
    subject: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


