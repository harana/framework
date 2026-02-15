# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Queue(BaseModel):
    """
    queue
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    dead_letter_queue_id: Optional[str] = Field(default=None, description="Reference: queue.id")
    is_fifo: bool = Field(default=False)
    max_message_size: int = Field(default=262144)
    message_retention_seconds: int = Field(default=345600)
    receive_wait_time_seconds: int = Field(default=0)
    type: str = Field(default="standard")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    visibility_timeout_seconds: int = Field(default=30)
    class Config:
        from_attributes = True
        populate_by_name = True


class QueueMessage(BaseModel):
    """
    queue_message
    
    ID fields: queue_id, sequence_number, created_at
    """

    attributes: Optional[str] = None
    body: str
    message_group_id: Optional[str] = None
    queue_id: str = Field(description="Reference: queue.id")
    receipt_handle: Optional[str] = None
    receive_count: int = Field(default=0)
    sequence_number: Optional[int] = None
    status: str = Field(default="pending")
    visible_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class QueueSubscription(BaseModel):
    """
    queue_subscription
    
    ID fields: queue_id, endpoint
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    endpoint: str
    is_active: bool = Field(default=True)
    protocol: str = Field(default="https")
    queue_id: str = Field(description="Reference: queue.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class QueueMetric(BaseModel):
    """
    queue_metric
    
    ID fields: queue_id, timestamp
    """

    approximate_age_oldest_message: Optional[int] = None
    approximate_messages_delayed: int = Field(default=0)
    queue_id: str = Field(description="Reference: queue.id")
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class QueuePeekMessage(BaseModel):
    """
    queue_peek_message
    
    ID fields: id
    """

    message_id: str
    message: str
    enqueued_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


