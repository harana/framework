# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateLoadBalancer(BaseModel):
    """
    create_load_balancer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLoadBalancer(BaseModel):
    """
    delete_load_balancer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLoadBalancers(BaseModel):
    """
    describe_load_balancers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyLoadBalancerAttributes(BaseModel):
    """
    modify_load_balancer_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLoadBalancerAttributes(BaseModel):
    """
    describe_load_balancer_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetSecurityGroups(BaseModel):
    """
    set_security_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetSubnets(BaseModel):
    """
    set_subnets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetIpAddressType(BaseModel):
    """
    set_ip_address_type
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTargetGroup(BaseModel):
    """
    create_target_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTargetGroup(BaseModel):
    """
    delete_target_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTargetGroups(BaseModel):
    """
    describe_target_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyTargetGroup(BaseModel):
    """
    modify_target_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyTargetGroupAttributes(BaseModel):
    """
    modify_target_group_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTargetGroupAttributes(BaseModel):
    """
    describe_target_group_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RegisterTargets(BaseModel):
    """
    register_targets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeregisterTargets(BaseModel):
    """
    deregister_targets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTargetHealth(BaseModel):
    """
    describe_target_health
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateListener(BaseModel):
    """
    create_listener
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteListener(BaseModel):
    """
    delete_listener
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeListeners(BaseModel):
    """
    describe_listeners
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyListener(BaseModel):
    """
    modify_listener
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRule(BaseModel):
    """
    create_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRule(BaseModel):
    """
    delete_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeRules(BaseModel):
    """
    describe_rules
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyRule(BaseModel):
    """
    modify_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetRulePriorities(BaseModel):
    """
    set_rule_priorities
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddListenerCertificates(BaseModel):
    """
    add_listener_certificates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveListenerCertificates(BaseModel):
    """
    remove_listener_certificates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeListenerCertificates(BaseModel):
    """
    describe_listener_certificates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSslPolicies(BaseModel):
    """
    describe_ssl_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAccountLimits(BaseModel):
    """
    describe_account_limits
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddTags(BaseModel):
    """
    add_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveTags(BaseModel):
    """
    remove_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTags(BaseModel):
    """
    describe_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


