# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsSqsQueue(BaseModel):
    """
    aws_sqs_queue
    
    ID fields: account_id, queue_name
    """

    account_id: str
    content_based_deduplication: bool = Field(default=False)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dead_letter_target_arn: Optional[str] = None
    delay_seconds: int = Field(default=0)
    is_fifo: bool = Field(default=False)
    max_message_size: int = Field(default=262144)
    message_retention_seconds: int = Field(default=345600)
    queue_arn: Optional[str] = None
    queue_name: str
    queue_url: Optional[str] = None
    receive_wait_time_seconds: int = Field(default=0)
    redrive_max_receive_count: Optional[int] = None
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    visibility_timeout: int = Field(default=30)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSqsMessage(BaseModel):
    """
    aws_sqs_message
    
    ID fields: queue_id, message_id
    """

    body: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    delay_seconds: Optional[int] = None
    message_attributes: Optional[str] = None
    message_deduplication_id: Optional[str] = None
    message_group_id: Optional[str] = None
    message_id: str
    queue_id: str = Field(description="Reference: aws_sqs_queue.id")
    receipt_handle: Optional[str] = None
    receive_count: int = Field(default=0)
    sequence_number: Optional[str] = None
    status: str = Field(default="pending")
    visible_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSqsQueuePermission(BaseModel):
    """
    aws_sqs_queue_permission
    
    ID fields: queue_id, label
    """

    actions: str
    aws_account_ids: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    label: str
    queue_id: str = Field(description="Reference: aws_sqs_queue.id")
    class Config:
        from_attributes = True
        populate_by_name = True


