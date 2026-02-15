# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Enqueue(BaseModel):
    """
    enqueue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Dequeue(BaseModel):
    """
    dequeue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Peek(BaseModel):
    """
    peek
    
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


class Nack(BaseModel):
    """
    nack
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetQueueStats(BaseModel):
    """
    get_queue_stats
    
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


class ListQueues(BaseModel):
    """
    list_queues
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MoveToDlq(BaseModel):
    """
    move_to_dlq
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueueMessage(BaseModel):
    """
    queue_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueuePeekMessage(BaseModel):
    """
    queue_peek_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


