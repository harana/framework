# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Insert(BaseModel):
    """
    insert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InsertMany(BaseModel):
    """
    insert_many
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FindOne(BaseModel):
    """
    find_one
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Find(BaseModel):
    """
    find
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateOne(BaseModel):
    """
    update_one
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateMany(BaseModel):
    """
    update_many
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplaceOne(BaseModel):
    """
    replace_one
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteOne(BaseModel):
    """
    delete_one
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMany(BaseModel):
    """
    delete_many
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Count(BaseModel):
    """
    count
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Aggregate(BaseModel):
    """
    aggregate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateIndex(BaseModel):
    """
    create_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DropIndex(BaseModel):
    """
    drop_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCollections(BaseModel):
    """
    list_collections
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DropCollection(BaseModel):
    """
    drop_collection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkWrite(BaseModel):
    """
    bulk_write
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoDocument(BaseModel):
    """
    mongo_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoDocumentData(BaseModel):
    """
    mongo_document_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoFilter(BaseModel):
    """
    mongo_filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoProjection(BaseModel):
    """
    mongo_projection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoSort(BaseModel):
    """
    mongo_sort
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoUpdate(BaseModel):
    """
    mongo_update
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoPipelineStage(BaseModel):
    """
    mongo_pipeline_stage
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoIndexKeys(BaseModel):
    """
    mongo_index_keys
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MongoBulkOperation(BaseModel):
    """
    mongo_bulk_operation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


