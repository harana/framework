# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class All(BaseModel):
    """
    all
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Race(BaseModel):
    """
    race
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Any(BaseModel):
    """
    any
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AllSettled(BaseModel):
    """
    all_settled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Map(BaseModel):
    """
    map
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Filter(BaseModel):
    """
    filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Reduce(BaseModel):
    """
    reduce
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Retry(BaseModel):
    """
    retry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Timeout(BaseModel):
    """
    timeout
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Batch(BaseModel):
    """
    batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Semaphore(BaseModel):
    """
    semaphore
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ParallelTask(BaseModel):
    """
    parallel_task
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ParallelResult(BaseModel):
    """
    parallel_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ParallelSettledResult(BaseModel):
    """
    parallel_settled_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ParallelError(BaseModel):
    """
    parallel_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


