# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareQueueMessageSent(BaseModel):
    """
    cloudflare_queue_message_sent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageBatchSent(BaseModel):
    """
    cloudflare_queue_message_batch_sent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageArrived(BaseModel):
    """
    cloudflare_queue_message_arrived
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageBatchArrived(BaseModel):
    """
    cloudflare_queue_message_batch_arrived
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageAcknowledged(BaseModel):
    """
    cloudflare_queue_message_acknowledged
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueAllMessagesAcknowledged(BaseModel):
    """
    cloudflare_queue_all_messages_acknowledged
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageRetried(BaseModel):
    """
    cloudflare_queue_message_retried
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueAllMessagesRetried(BaseModel):
    """
    cloudflare_queue_all_messages_retried
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareQueueMessageProcessingFailed(BaseModel):
    """
    cloudflare_queue_message_processing_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


