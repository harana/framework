# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateDistribution(BaseModel):
    """
    create_distribution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDistribution(BaseModel):
    """
    get_distribution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateDistribution(BaseModel):
    """
    update_distribution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDistribution(BaseModel):
    """
    delete_distribution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListDistributions(BaseModel):
    """
    list_distributions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateInvalidation(BaseModel):
    """
    create_invalidation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetInvalidation(BaseModel):
    """
    get_invalidation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListInvalidations(BaseModel):
    """
    list_invalidations
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateCachePolicy(BaseModel):
    """
    create_cache_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCachePolicy(BaseModel):
    """
    get_cache_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteCachePolicy(BaseModel):
    """
    delete_cache_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCachePolicies(BaseModel):
    """
    list_cache_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateOriginRequestPolicy(BaseModel):
    """
    create_origin_request_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateResponseHeadersPolicy(BaseModel):
    """
    create_response_headers_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateFunction(BaseModel):
    """
    create_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PublishFunction(BaseModel):
    """
    publish_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDistributionConfig(BaseModel):
    """
    get_distribution_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TagResource(BaseModel):
    """
    tag_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTagsForResource(BaseModel):
    """
    list_tags_for_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


