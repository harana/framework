# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GetSecret(BaseModel):
    """
    get_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetSecret(BaseModel):
    """
    set_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSecret(BaseModel):
    """
    delete_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListSecrets(BaseModel):
    """
    list_secrets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RotateSecret(BaseModel):
    """
    rotate_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSecretMetadata(BaseModel):
    """
    get_secret_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretExists(BaseModel):
    """
    secret_exists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListSecretVersions(BaseModel):
    """
    list_secret_versions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreSecretVersion(BaseModel):
    """
    restore_secret_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateSecret(BaseModel):
    """
    generate_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Secret(BaseModel):
    """
    secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretInfo(BaseModel):
    """
    secret_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretVersion(BaseModel):
    """
    secret_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


