# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateTopic(BaseModel):
    """
    create_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTopic(BaseModel):
    """
    delete_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTopic(BaseModel):
    """
    get_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTopics(BaseModel):
    """
    list_topics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Publish(BaseModel):
    """
    publish
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PublishBatch(BaseModel):
    """
    publish_batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSubscription(BaseModel):
    """
    create_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSubscription(BaseModel):
    """
    delete_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSubscription(BaseModel):
    """
    get_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListSubscriptions(BaseModel):
    """
    list_subscriptions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Pull(BaseModel):
    """
    pull
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Acknowledge(BaseModel):
    """
    acknowledge
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyAckDeadline(BaseModel):
    """
    modify_ack_deadline
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Seek(BaseModel):
    """
    seek
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSnapshot(BaseModel):
    """
    create_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSnapshot(BaseModel):
    """
    delete_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTopicSubscriptions(BaseModel):
    """
    list_topic_subscriptions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateTopic(BaseModel):
    """
    update_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateSubscription(BaseModel):
    """
    update_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubMessage(BaseModel):
    """
    pub_sub_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubLabels(BaseModel):
    """
    pub_sub_labels
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubLabel(BaseModel):
    """
    pub_sub_label
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubSchemaSettings(BaseModel):
    """
    pub_sub_schema_settings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubTopic(BaseModel):
    """
    pub_sub_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubMessageAttributes(BaseModel):
    """
    pub_sub_message_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubMessageAttribute(BaseModel):
    """
    pub_sub_message_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubPublishMessage(BaseModel):
    """
    pub_sub_publish_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeadLetterPolicy(BaseModel):
    """
    dead_letter_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushConfig(BaseModel):
    """
    push_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OidcToken(BaseModel):
    """
    oidc_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RetryPolicy(BaseModel):
    """
    retry_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubSubscription(BaseModel):
    """
    pub_sub_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PubSubReceivedMessage(BaseModel):
    """
    pub_sub_received_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


