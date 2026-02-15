# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateQueue(BaseModel):
    """
    create_queue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteQueue(BaseModel):
    """
    delete_queue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetQueueUrl(BaseModel):
    """
    get_queue_url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListQueues(BaseModel):
    """
    list_queues
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessage(BaseModel):
    """
    send_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessageBatch(BaseModel):
    """
    send_message_batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReceiveMessage(BaseModel):
    """
    receive_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessage(BaseModel):
    """
    delete_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessageBatch(BaseModel):
    """
    delete_message_batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibility(BaseModel):
    """
    change_message_visibility
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibilityBatch(BaseModel):
    """
    change_message_visibility_batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetQueueAttributes(BaseModel):
    """
    get_queue_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetQueueAttributes(BaseModel):
    """
    set_queue_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PurgeQueue(BaseModel):
    """
    purge_queue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddPermission(BaseModel):
    """
    add_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemovePermission(BaseModel):
    """
    remove_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TagQueue(BaseModel):
    """
    tag_queue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UntagQueue(BaseModel):
    """
    untag_queue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListQueueTags(BaseModel):
    """
    list_queue_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListDeadLetterSourceQueues(BaseModel):
    """
    list_dead_letter_source_queues
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


