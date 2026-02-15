# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Address(BaseModel):
    """
    address
    
    ID fields: entity_id, entity_type, type
    """

    city: Optional[str] = None
    country: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    entity_id: str
    entity_type: str
    is_primary: bool = Field(default=False)
    is_verified: bool = Field(default=False)
    label: Optional[str] = None
    postal_code: Optional[str] = None
    state: Optional[str] = None
    street: Optional[str] = None
    type: str = Field(default="home")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    verified_at: Optional[datetime] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AddressValidationLog(BaseModel):
    """
    address_validation_log
    
    ID fields: address_id, validated_at
    """

    address_id: str = Field(description="Reference: address.id")
    errors: Optional[str] = None
    is_valid: bool = Field(default=False)
    provider: Optional[str] = None
    validated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


