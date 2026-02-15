# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareWorker(BaseModel):
    """
    cloudflare_worker
    
    ID fields: account_id, id
    """

    account_id: str
    compatibility_date: Optional[str] = None
    compatibility_flags: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    etag: Optional[str] = None
    id: str
    size: Optional[int] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerServiceBinding(BaseModel):
    """
    cloudflare_worker_service_binding
    
    ID fields: worker_id, binding_name
    """

    binding_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    environment: Optional[str] = None
    target_service: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    worker_id: str = Field(description="Reference: cf_worker.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerCronTrigger(BaseModel):
    """
    cloudflare_worker_cron_trigger
    
    ID fields: worker_id, cron
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    cron: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    worker_id: str = Field(description="Reference: cf_worker.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerVersion(BaseModel):
    """
    cloudflare_worker_version
    
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


