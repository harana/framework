# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class EncryptionKey(BaseModel):
    """
    encryption_key
    
    ID fields: key_id
    """

    algorithm: str = Field(default="aes256_gcm")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    expires_at: Optional[datetime] = None
    is_active: bool = Field(default=True)
    key_id: str
    key_size: int
    key_type: str = Field(default="symmetric")
    last_rotated_at: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    usage: str = Field(default="encrypt")
    class Config:
        from_attributes = True
        populate_by_name = True


class KeyPair(BaseModel):
    """
    key_pair
    
    ID fields: key_id
    """

    algorithm: str = Field(default="rsa")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    expires_at: Optional[datetime] = None
    is_active: bool = Field(default=True)
    key_id: str
    key_size: int
    public_key_fingerprint: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CryptoOperationLog(BaseModel):
    """
    crypto_operation_log
    
    ID fields: key_id, created_at
    """

    algorithm: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    key_id: str
    operation: str = Field(default="encrypt")
    status: str = Field(default="success")
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


