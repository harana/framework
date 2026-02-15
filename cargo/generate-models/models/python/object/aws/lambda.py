# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsLambdaFunction(BaseModel):
    """
    aws_lambda_function
    
    ID fields: account_id, function_name
    """

    account_id: str
    architectures: Optional[str] = None
    code_sha256: Optional[str] = None
    code_size: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dead_letter_arn: Optional[str] = None
    description: Optional[str] = None
    function_arn: Optional[str] = None
    function_name: str
    handler: Optional[str] = None
    kms_key_arn: Optional[str] = None
    last_modified: Optional[datetime] = None
    memory_size: int = Field(default=128)
    package_type: str = Field(default="zip")
    region: Optional[str] = None
    role: str
    runtime: Optional[str] = None
    state: str = Field(default="pending")
    tags: Optional[str] = None
    timeout: int = Field(default=3)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsLambdaFunctionVersion(BaseModel):
    """
    aws_lambda_function_version
    
    ID fields: function_id, version
    """

    code_sha256: Optional[str] = None
    code_size: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    function_id: str = Field(description="Reference: aws_lambda_function.id")
    handler: Optional[str] = None
    memory_size: Optional[int] = None
    runtime: Optional[str] = None
    timeout: Optional[int] = None
    version: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsLambdaAlias(BaseModel):
    """
    aws_lambda_alias
    
    ID fields: function_id, alias_name
    """

    alias_arn: Optional[str] = None
    alias_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    function_id: str = Field(description="Reference: aws_lambda_function.id")
    function_version: str
    routing_config: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsLambdaEventSourceMapping(BaseModel):
    """
    aws_lambda_event_source_mapping
    
    ID fields: function_id, event_source_arn
    """

    batch_size: Optional[int] = None
    bisect_batch_on_error: bool = Field(default=False)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    enabled: bool = Field(default=True)
    event_source_arn: str
    function_id: str = Field(description="Reference: aws_lambda_function.id")
    last_processing_result: Optional[str] = None
    maximum_batching_window: Optional[int] = None
    maximum_retry_attempts: Optional[int] = None
    state: str = Field(default="creating")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    uuid: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsLambdaLayer(BaseModel):
    """
    aws_lambda_layer
    
    ID fields: account_id, layer_name
    """

    account_id: str
    compatible_architectures: Optional[str] = None
    compatible_runtimes: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    layer_arn: Optional[str] = None
    layer_name: str
    latest_version: int = Field(default=1)
    license_info: Optional[str] = None
    region: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsLambdaInvocationLog(BaseModel):
    """
    aws_lambda_invocation_log
    
    ID fields: function_id, invoked_at
    """

    billed_duration_ms: Optional[int] = None
    duration_ms: Optional[int] = None
    error: Optional[str] = None
    function_id: str = Field(description="Reference: aws_lambda_function.id")
    invocation_type: str = Field(default="request_response")
    invoked_at: datetime = Field(default_factory=datetime.utcnow)
    max_memory_used_mb: Optional[int] = None
    request_id: Optional[str] = None
    status: str = Field(default="success")
    class Config:
        from_attributes = True
        populate_by_name = True


