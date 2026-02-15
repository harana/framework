# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareAnalyticsEngineDataset(BaseModel):
    """
    cloudflare_analytics_engine_dataset
    
    ID fields: account_id, dataset_name
    """

    account_id: str
    binding: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dataset_name: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareAnalyticsEngineDataPoint(BaseModel):
    """
    cloudflare_analytics_engine_data_point
    
    ID fields: dataset_id, timestamp
    """

    blobs: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dataset_id: str = Field(description="Reference: cf_analytics_engine_dataset.id")
    doubles: Optional[str] = None
    indexes: Optional[str] = None
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


