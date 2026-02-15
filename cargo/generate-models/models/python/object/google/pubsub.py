# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GooglePubsubTopic(BaseModel):
    """
    google_pubsub_topic
    
    ID fields: project_id, topic_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    labels: Optional[str] = None
    message_retention_duration: Optional[str] = None
    project_id: str
    schema_encoding: Optional[str] = None
    schema_name: Optional[str] = None
    topic_id: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class GooglePubsubSubscription(BaseModel):
    """
    google_pubsub_subscription
    
    ID fields: project_id, subscription_id
    """

    ack_deadline_seconds: int = Field(default=10)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dead_letter_max_delivery_attempts: Optional[int] = None
    dead_letter_topic: Optional[str] = None
    enable_message_ordering: bool = Field(default=False)
    filter: Optional[str] = None
    labels: Optional[str] = None
    message_retention_duration: Optional[str] = None
    project_id: str
    push_endpoint: Optional[str] = None
    retain_acked_messages: bool = Field(default=False)
    retry_max_backoff: Optional[str] = None
    retry_min_backoff: Optional[str] = None
    subscription_id: str
    topic_id: str = Field(description="Reference: google_pubsub_topic.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class GooglePubsubMessage(BaseModel):
    """
    google_pubsub_message
    
    ID fields: topic_id, message_id
    """

    attributes: Optional[str] = None
    data: str
    message_id: str
    ordering_key: Optional[str] = None
    publish_time: Optional[datetime] = None
    topic_id: str = Field(description="Reference: google_pubsub_topic.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class GooglePubsubReceivedMessage(BaseModel):
    """
    google_pubsub_received_message
    
    ID fields: subscription_id, ack_id
    """

    ack_id: str
    delivery_attempt: Optional[int] = None
    message_id: str = Field(description="Reference: google_pubsub_message.id")
    received_at: datetime = Field(default_factory=datetime.utcnow)
    subscription_id: str = Field(description="Reference: google_pubsub_subscription.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class GooglePubsubSnapshot(BaseModel):
    """
    google_pubsub_snapshot
    
    ID fields: project_id, snapshot_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    expire_time: Optional[datetime] = None
    labels: Optional[str] = None
    project_id: str
    snapshot_id: str
    subscription_id: str = Field(description="Reference: google_pubsub_subscription.id")
    topic: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GooglePubsubLabel(BaseModel):
    """
    google_pubsub_label
    
    ID fields: resource_id, key
    """

    key: str
    resource_id: str
    resource_type: str
    value: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


