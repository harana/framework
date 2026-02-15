# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SigningKey(BaseModel):
    """
    signing_key
    
    ID fields: key_id
    """

    algorithm: str = Field(default="rsa_sha256")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    expires_at: Optional[datetime] = None
    is_active: bool = Field(default=True)
    key_id: str
    public_key_fingerprint: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    usage: str = Field(default="artifact")
    class Config:
        from_attributes = True
        populate_by_name = True


class Signature(BaseModel):
    """
    signature
    
    ID fields: artifact_hash, key_id
    """

    algorithm: str
    artifact_hash: str
    artifact_path: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    format: str = Field(default="pkcs7")
    is_valid: bool = Field(default=True)
    key_id: str
    signer: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class SignatureVerificationLog(BaseModel):
    """
    signature_verification_log
    
    ID fields: signature_id, verified_at
    """

    error_message: Optional[str] = None
    ip_address: Optional[str] = None
    is_valid: bool = Field(default=False)
    signature_id: str = Field(description="Reference: signature.id")
    verified_at: datetime = Field(default_factory=datetime.utcnow)
    verified_by: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SignatureMetadata(BaseModel):
    """
    signature_metadata
    
    ID fields: id
    """

    signer: str
    organization: str
    email: str
    description: str
    url: str
    custom_attributes: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SignedArtifact(BaseModel):
    """
    signed_artifact
    
    ID fields: id
    """

    artifact: str
    signature: str
    algorithm: str
    format: str
    timestamp: datetime
    signer: str
    certificate_chain: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


