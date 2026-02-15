# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsCloudfrontDistribution(BaseModel):
    """
    aws_cloudfront_distribution
    
    ID fields: account_id, distribution_id
    """

    account_id: str
    aliases: Optional[str] = None
    arn: Optional[str] = None
    certificate_arn: Optional[str] = None
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_root_object: Optional[str] = None
    distribution_id: str
    domain_name: Optional[str] = None
    enabled: bool = Field(default=True)
    http_version: str = Field(default="http2")
    last_modified_time: Optional[datetime] = None
    origin_domain_name: str
    origin_id: str
    price_class: str = Field(default="price_class_all")
    region: Optional[str] = None
    ssl_support_method: str = Field(default="sni_only")
    status: str = Field(default="in_progress")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsCloudfrontInvalidation(BaseModel):
    """
    aws_cloudfront_invalidation
    
    ID fields: distribution_id, invalidation_id
    """

    create_time: datetime = Field(default_factory=datetime.utcnow)
    distribution_id: str = Field(description="Reference: aws_cloudfront_distribution.id")
    invalidation_id: str
    paths: str
    status: str = Field(default="in_progress")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsCloudfrontCachePolicy(BaseModel):
    """
    aws_cloudfront_cache_policy
    
    ID fields: account_id, cache_policy_id
    """

    account_id: str
    cache_policy_id: str
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_ttl: int = Field(default=86400)
    etag: Optional[str] = None
    max_ttl: int = Field(default=31536000)
    min_ttl: int = Field(default=0)
    name: str
    parameters_in_cache_key: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsCloudfrontOriginRequestPolicy(BaseModel):
    """
    aws_cloudfront_origin_request_policy
    
    ID fields: account_id, origin_request_policy_id
    """

    account_id: str
    comment: Optional[str] = None
    cookies_config: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    etag: Optional[str] = None
    headers_config: Optional[str] = None
    name: str
    origin_request_policy_id: str
    query_strings_config: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsCloudfrontFunction(BaseModel):
    """
    aws_cloudfront_function
    
    ID fields: account_id, function_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    etag: Optional[str] = None
    function_arn: Optional[str] = None
    function_name: str
    runtime: str = Field(default="cloudfront_js20")
    status: str = Field(default="unpublished")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


