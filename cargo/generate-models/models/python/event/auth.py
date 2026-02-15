# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AuthRegistrationStarted(BaseModel):
    """
    auth_registration_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthRegistrationCompleted(BaseModel):
    """
    auth_registration_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthRegistrationFailed(BaseModel):
    """
    auth_registration_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthLoginStarted(BaseModel):
    """
    auth_login_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthLoginCompleted(BaseModel):
    """
    auth_login_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthLoginFailed(BaseModel):
    """
    auth_login_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthLogout(BaseModel):
    """
    auth_logout
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthPasskeyCreated(BaseModel):
    """
    auth_passkey_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthPasskeyRemoved(BaseModel):
    """
    auth_passkey_removed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthPasskeyUsed(BaseModel):
    """
    auth_passkey_used
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthSessionCreated(BaseModel):
    """
    auth_session_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthSessionExpired(BaseModel):
    """
    auth_session_expired
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthSessionRevoked(BaseModel):
    """
    auth_session_revoked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthChallengeCreated(BaseModel):
    """
    auth_challenge_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthChallengeCompleted(BaseModel):
    """
    auth_challenge_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthAccountLocked(BaseModel):
    """
    auth_account_locked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthAccountUnlocked(BaseModel):
    """
    auth_account_unlocked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthReauthenticationRequired(BaseModel):
    """
    auth_reauthentication_required
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthReauthenticationCompleted(BaseModel):
    """
    auth_reauthentication_completed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


