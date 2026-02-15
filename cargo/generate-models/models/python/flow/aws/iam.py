# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateUser(BaseModel):
    """
    create_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteUser(BaseModel):
    """
    delete_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetUser(BaseModel):
    """
    get_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListUsers(BaseModel):
    """
    list_users
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateUser(BaseModel):
    """
    update_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateGroup(BaseModel):
    """
    create_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteGroup(BaseModel):
    """
    delete_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListGroups(BaseModel):
    """
    list_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddUserToGroup(BaseModel):
    """
    add_user_to_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveUserFromGroup(BaseModel):
    """
    remove_user_from_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListGroupsForUser(BaseModel):
    """
    list_groups_for_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRole(BaseModel):
    """
    create_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRole(BaseModel):
    """
    delete_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRole(BaseModel):
    """
    get_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRoles(BaseModel):
    """
    list_roles
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePolicy(BaseModel):
    """
    create_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeletePolicy(BaseModel):
    """
    delete_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPolicy(BaseModel):
    """
    get_policy
    
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


class AttachUserPolicy(BaseModel):
    """
    attach_user_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachUserPolicy(BaseModel):
    """
    detach_user_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachGroupPolicy(BaseModel):
    """
    attach_group_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachGroupPolicy(BaseModel):
    """
    detach_group_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachRolePolicy(BaseModel):
    """
    attach_role_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachRolePolicy(BaseModel):
    """
    detach_role_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutUserPolicy(BaseModel):
    """
    put_user_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutGroupPolicy(BaseModel):
    """
    put_group_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutRolePolicy(BaseModel):
    """
    put_role_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateAccessKey(BaseModel):
    """
    create_access_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteAccessKey(BaseModel):
    """
    delete_access_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListAccessKeys(BaseModel):
    """
    list_access_keys
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateAccessKey(BaseModel):
    """
    update_access_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAccountSummary(BaseModel):
    """
    get_account_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateCredentialReport(BaseModel):
    """
    generate_credential_report
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCredentialReport(BaseModel):
    """
    get_credential_report
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


