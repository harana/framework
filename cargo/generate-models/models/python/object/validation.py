# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ValidationRule(BaseModel):
    """
    validation_rule
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    pattern: Optional[str] = None
    type: str = Field(default="required")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationRuleSet(BaseModel):
    """
    validation_rule_set
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationRuleSetAssignment(BaseModel):
    """
    validation_rule_set_assignment
    
    ID fields: rule_set_id, rule_id
    """

    rule_id: str = Field(description="Reference: validation_rule.id")
    rule_set_id: str = Field(description="Reference: validation_rule_set.id")
    sort_order: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationErrorLog(BaseModel):
    """
    validation_error_log
    
    ID fields: field_name, created_at, rule_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    field_name: str
    input_value: Optional[str] = None
    rule_id: Optional[str] = Field(default=None, description="Reference: validation_rule.id")
    rule_set_id: Optional[str] = Field(default=None, description="Reference: validation_rule_set.id")
    violation_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


