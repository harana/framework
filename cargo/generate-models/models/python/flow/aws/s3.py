# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateBucket(BaseModel):
    """
    create_bucket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteBucket(BaseModel):
    """
    delete_bucket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListBuckets(BaseModel):
    """
    list_buckets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutObject(BaseModel):
    """
    put_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetObject(BaseModel):
    """
    get_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteObject(BaseModel):
    """
    delete_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteObjects(BaseModel):
    """
    delete_objects
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListObjects(BaseModel):
    """
    list_objects
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopyObject(BaseModel):
    """
    copy_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HeadObject(BaseModel):
    """
    head_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPresignedUrl(BaseModel):
    """
    get_presigned_url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutBucketPolicy(BaseModel):
    """
    put_bucket_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBucketPolicy(BaseModel):
    """
    get_bucket_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteBucketPolicy(BaseModel):
    """
    delete_bucket_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutBucketVersioning(BaseModel):
    """
    put_bucket_versioning
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBucketVersioning(BaseModel):
    """
    get_bucket_versioning
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutBucketEncryption(BaseModel):
    """
    put_bucket_encryption
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBucketEncryption(BaseModel):
    """
    get_bucket_encryption
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutBucketCors(BaseModel):
    """
    put_bucket_cors
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBucketCors(BaseModel):
    """
    get_bucket_cors
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteBucketCors(BaseModel):
    """
    delete_bucket_cors
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutBucketLifecycle(BaseModel):
    """
    put_bucket_lifecycle
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetBucketLifecycle(BaseModel):
    """
    get_bucket_lifecycle
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutObjectTagging(BaseModel):
    """
    put_object_tagging
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetObjectTagging(BaseModel):
    """
    get_object_tagging
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateMultipartUpload(BaseModel):
    """
    create_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UploadPart(BaseModel):
    """
    upload_part
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CompleteMultipartUpload(BaseModel):
    """
    complete_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AbortMultipartUpload(BaseModel):
    """
    abort_multipart_upload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


