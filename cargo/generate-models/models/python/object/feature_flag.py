# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FeatureFlag(BaseModel):
    """
    feature_flag
    
    ID fields: key
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_variation: int = Field(default=0)
    description: Optional[str] = None
    enabled: bool = Field(default=False)
    key: str
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class FeatureFlagVariation(BaseModel):
    """
    feature_flag_variation
    
    ID fields: flag_id, sort_order
    """

    description: Optional[str] = None
    flag_id: str = Field(description="Reference: feature_flag.id")
    sort_order: int = Field(default=0)
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FeatureFlagTargetingRule(BaseModel):
    """
    feature_flag_targeting_rule
    
    ID fields: flag_id, name
    """

    conditions: str
    flag_id: str = Field(description="Reference: feature_flag.id")
    is_active: bool = Field(default=True)
    name: str
    priority: int = Field(default=0)
    variation_index: int
    class Config:
        from_attributes = True
        populate_by_name = True


class FeatureFlagEvaluationLog(BaseModel):
    """
    feature_flag_evaluation_log
    
    ID fields: flag_id, user_id, evaluated_at
    """

    evaluated_at: datetime = Field(default_factory=datetime.utcnow)
    flag_id: str = Field(description="Reference: feature_flag.id")
    reason: Optional[str] = None
    user_id: Optional[str] = None
    variation_index: int
    class Config:
        from_attributes = True
        populate_by_name = True


