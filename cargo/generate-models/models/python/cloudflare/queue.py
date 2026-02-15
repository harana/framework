# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareQueue(BaseModel):
    """
    cloudflare_queue
    
    ID fields: account_id, queue_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    queue_id: Optional[str] = None
    queue_name: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessage(BaseModel):
    """
    cloudflare_queue_message
    
    ID fields: queue_id, message_id
    """

    body: str
    content_type: str = Field(default="json")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    delay_seconds: Optional[int] = None
    message_id: str
    queue_id: str = Field(description="Reference: cf_queue.id")
    receive_count: int = Field(default=0)
    status: str = Field(default="pending")
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueConsumer(BaseModel):
    """
    cloudflare_queue_consumer
    
    ID fields: queue_id, consumer_name
    """

    consumer_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    max_batch_size: int = Field(default=10)
    max_batch_timeout: int = Field(default=5)
    max_retries: int = Field(default=3)
    queue_id: str = Field(description="Reference: cf_queue.id")
    script_name: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


