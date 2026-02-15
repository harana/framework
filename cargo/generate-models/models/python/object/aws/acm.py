# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsAcmCertificate(BaseModel):
    """
    aws_acm_certificate
    
    ID fields: account_id, certificate_arn
    """

    account_id: str
    certificate_arn: str
    certificate_authority_arn: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    domain_name: str
    failure_reason: Optional[str] = None
    imported_at: Optional[datetime] = None
    in_use_by: Optional[str] = None
    issued_at: Optional[datetime] = None
    issuer: Optional[str] = None
    key_algorithm: str = Field(default="RSA_2048")
    not_after: Optional[datetime] = None
    not_before: Optional[datetime] = None
    region: Optional[str] = None
    renewal_eligibility: str = Field(default="ineligible")
    revocation_reason: Optional[str] = None
    revoked_at: Optional[datetime] = None
    serial: Optional[str] = None
    status: str = Field(default="pending_validation")
    subject_alternative_names: Optional[str] = None
    tags: Optional[str] = None
    type: str = Field(default="amazon_issued")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    validation_method: str = Field(default="dns")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsAcmDomainValidation(BaseModel):
    """
    aws_acm_domain_validation
    
    ID fields: certificate_id, domain_name
    """

    certificate_id: str = Field(description="Reference: aws_acm_certificate.id")
    domain_name: str
    resource_record_name: Optional[str] = None
    resource_record_type: Optional[str] = None
    resource_record_value: Optional[str] = None
    validation_domain: Optional[str] = None
    validation_method: str = Field(default="dns")
    validation_status: str = Field(default="pending_validation")
    class Config:
        from_attributes = True
        populate_by_name = True


