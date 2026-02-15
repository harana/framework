# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AssumeRole(BaseModel):
    """
    assume_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssumeRoleWithSaml(BaseModel):
    """
    assume_role_with_saml
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssumeRoleWithWebIdentity(BaseModel):
    """
    assume_role_with_web_identity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DecodeAuthorizationMessage(BaseModel):
    """
    decode_authorization_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCallerIdentity(BaseModel):
    """
    get_caller_identity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSessionToken(BaseModel):
    """
    get_session_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFederationToken(BaseModel):
    """
    get_federation_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAccessKeyInfo(BaseModel):
    """
    get_access_key_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


