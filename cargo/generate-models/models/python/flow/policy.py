# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Create(BaseModel):
    """
    create
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Update(BaseModel):
    """
    update
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListPolicies(BaseModel):
    """
    list_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachToUser(BaseModel):
    """
    attach_to_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachFromUser(BaseModel):
    """
    detach_from_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachToRole(BaseModel):
    """
    attach_to_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachFromRole(BaseModel):
    """
    detach_from_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Evaluate(BaseModel):
    """
    evaluate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Policy(BaseModel):
    """
    policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyConditions(BaseModel):
    """
    policy_conditions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyInfo(BaseModel):
    """
    policy_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyEvaluationContext(BaseModel):
    """
    policy_evaluation_context
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyEvaluatedConditions(BaseModel):
    """
    policy_evaluated_conditions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


