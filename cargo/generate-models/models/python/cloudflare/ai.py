# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareWorkersAiModel(BaseModel):
    """
    cloudflare_workers_ai_model
    
    ID fields: account_id, model_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    model_name: str
    task_type: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkersAiInference(BaseModel):
    """
    cloudflare_workers_ai_inference
    
    ID fields: model_id, invoked_at
    """

    binding: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    input_tokens: Optional[int] = None
    invoked_at: datetime = Field(default_factory=datetime.utcnow)
    model_id: str = Field(description="Reference: cf_workers_ai_model.id")
    output_tokens: Optional[int] = None
    status: str = Field(default="success")
    stream: bool = Field(default=False)
    class Config:
        from_attributes = True
        populate_by_name = True


