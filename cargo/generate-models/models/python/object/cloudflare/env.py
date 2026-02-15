# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CfEnvVariable(BaseModel):
    """
    cf_env_variable
    
    ID fields: worker_id, name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_secret: bool = Field(default=False)
    name: str
    type: str = Field(default="plain_text")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: Optional[str] = None
    worker_id: str = Field(description="Reference: cf_worker.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class CfSecretsStore(BaseModel):
    """
    cf_secrets_store
    
    ID fields: account_id, store_name
    """

    account_id: str
    binding: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    store_name: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CfSecretsStoreSecret(BaseModel):
    """
    cf_secrets_store_secret
    
    ID fields: store_id, name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    name: str
    store_id: str = Field(description="Reference: cf_secrets_store.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CfVersionMetadata(BaseModel):
    """
    cf_version_metadata
    
    ID fields: worker_id, version_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    message: Optional[str] = None
    tag: Optional[str] = None
    timestamp: Optional[datetime] = None
    version_id: str
    worker_id: str = Field(description="Reference: cf_worker.id")
    class Config:
        from_attributes = True
        populate_by_name = True


