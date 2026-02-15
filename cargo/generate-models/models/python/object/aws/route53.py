# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsRoute53HostedZone(BaseModel):
    """
    aws_route53_hosted_zone
    
    ID fields: account_id, hosted_zone_id
    """

    account_id: str
    caller_reference: Optional[str] = None
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    hosted_zone_id: str
    is_private: bool = Field(default=False)
    name: str
    name_servers: Optional[str] = None
    record_set_count: int = Field(default=0)
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRoute53RecordSet(BaseModel):
    """
    aws_route53_record_set
    
    ID fields: hosted_zone_id, name, type
    """

    alias_dns_name: Optional[str] = None
    alias_evaluate_target_health: bool = Field(default=False)
    alias_hosted_zone_id: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    failover: str
    health_check_id: Optional[str] = None
    hosted_zone_id: str = Field(description="Reference: aws_route53_hosted_zone.id")
    multi_value_answer: bool = Field(default=False)
    name: str
    region: Optional[str] = None
    resource_records: Optional[str] = None
    set_identifier: Optional[str] = None
    ttl: Optional[int] = None
    type: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    weight: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRoute53HealthCheck(BaseModel):
    """
    aws_route53_health_check
    
    ID fields: account_id, health_check_id
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    disabled: bool = Field(default=False)
    enable_sni: bool = Field(default=True)
    failure_threshold: int = Field(default=3)
    fqdn: Optional[str] = None
    health_check_id: str
    health_threshold: Optional[int] = None
    insufficient_data_health_status: str = Field(default="healthy")
    inverted: bool = Field(default=False)
    ip_address: Optional[str] = None
    port: Optional[int] = None
    region: Optional[str] = None
    request_interval: int = Field(default=30)
    resource_path: Optional[str] = None
    search_string: Optional[str] = None
    status: str = Field(default="healthy")
    tags: Optional[str] = None
    type: str = Field(default="http")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRoute53TrafficPolicy(BaseModel):
    """
    aws_route53_traffic_policy
    
    ID fields: account_id, traffic_policy_id, version
    """

    account_id: str
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    document: str
    name: str
    region: Optional[str] = None
    traffic_policy_id: str
    type: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: int
    class Config:
        from_attributes = True
        populate_by_name = True


