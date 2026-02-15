# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class LoginAttempted(BaseModel):
    """
    login_attempted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LogoutClicked(BaseModel):
    """
    logout_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasswordResetRequested(BaseModel):
    """
    password_reset_requested
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignupInitiated(BaseModel):
    """
    signup_initiated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OauthProviderSelected(BaseModel):
    """
    oauth_provider_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


