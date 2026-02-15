# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateFlag(BaseModel):
    """
    create_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateFlag(BaseModel):
    """
    update_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ToggleFlag(BaseModel):
    """
    toggle_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFlag(BaseModel):
    """
    delete_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFlag(BaseModel):
    """
    get_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListFlags(BaseModel):
    """
    list_flags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EvaluateFlag(BaseModel):
    """
    evaluate_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTargetingRule(BaseModel):
    """
    create_targeting_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateTargetingRule(BaseModel):
    """
    update_targeting_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTargetingRule(BaseModel):
    """
    delete_targeting_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRollout(BaseModel):
    """
    create_rollout
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateRollout(BaseModel):
    """
    update_rollout
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetEvaluationCount(BaseModel):
    """
    get_evaluation_count
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateEnvironment(BaseModel):
    """
    create_environment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloneFlag(BaseModel):
    """
    clone_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ArchiveFlag(BaseModel):
    """
    archive_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreFlag(BaseModel):
    """
    restore_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FeatureFlag(BaseModel):
    """
    feature_flag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FlagVariation(BaseModel):
    """
    flag_variation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FlagEvaluationContext(BaseModel):
    """
    flag_evaluation_context
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FlagRuleCondition(BaseModel):
    """
    flag_rule_condition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


