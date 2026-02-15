# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Authenticate(BaseModel):
    """
    authenticate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyToken(BaseModel):
    """
    verify_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RefreshToken(BaseModel):
    """
    refresh_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RevokeToken(BaseModel):
    """
    revoke_token
    
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


class CreateUser(BaseModel):
    """
    create_user
    
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


class DeleteUser(BaseModel):
    """
    delete_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangePassword(BaseModel):
    """
    change_password
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RequestPasswordReset(BaseModel):
    """
    request_password_reset
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResetPassword(BaseModel):
    """
    reset_password
    
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


class User(BaseModel):
    """
    user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TokenClaims(BaseModel):
    """
    token_claims
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserMetadata(BaseModel):
    """
    user_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserInfo(BaseModel):
    """
    user_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


