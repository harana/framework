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


class PubSubMessage(BaseModel):
    """
    pub_sub_message
    
    ID fields: id
    """

    message_id: str
    data: str
    attributes: str
    publish_time: datetime
    ordering_key: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubLabels(BaseModel):
    """
    pub_sub_labels
    
    ID fields: id
    """

    labels: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubLabel(BaseModel):
    """
    pub_sub_label
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubSchemaSettings(BaseModel):
    """
    pub_sub_schema_settings
    
    ID fields: id
    """

    schema: str
    encoding: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubTopic(BaseModel):
    """
    pub_sub_topic
    
    ID fields: id
    """

    name: str
    labels: str
    message_retention_duration: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubMessageAttributes(BaseModel):
    """
    pub_sub_message_attributes
    
    ID fields: id
    """

    attributes: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubMessageAttribute(BaseModel):
    """
    pub_sub_message_attribute
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubPublishMessage(BaseModel):
    """
    pub_sub_publish_message
    
    ID fields: id
    """

    data: str
    attributes: str
    ordering_key: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeadLetterPolicy(BaseModel):
    """
    dead_letter_policy
    
    ID fields: id
    """

    dead_letter_topic: str
    max_delivery_attempts: int
    class Config:
        from_attributes = True
        populate_by_name = True


class PushConfig(BaseModel):
    """
    push_config
    
    ID fields: id
    """

    push_endpoint: str
    attributes: str
    oidc_token: str
    class Config:
        from_attributes = True
        populate_by_name = True


class OidcToken(BaseModel):
    """
    oidc_token
    
    ID fields: id
    """

    service_account_email: str
    audience: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RetryPolicy(BaseModel):
    """
    retry_policy
    
    ID fields: id
    """

    minimum_backoff: str
    maximum_backoff: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubSubscription(BaseModel):
    """
    pub_sub_subscription
    
    ID fields: id
    """

    name: str
    topic: str
    ack_deadline_seconds: int
    message_retention_duration: str
    labels: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubReceivedMessage(BaseModel):
    """
    pub_sub_received_message
    
    ID fields: id
    """

    ack_id: str
    message: str
    delivery_attempt: int
    class Config:
        from_attributes = True
        populate_by_name = True


