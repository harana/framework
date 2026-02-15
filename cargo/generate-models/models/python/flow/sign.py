# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SignArtifact(BaseModel):
    """
    sign_artifact
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyArtifact(BaseModel):
    """
    verify_artifact
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignCode(BaseModel):
    """
    sign_code
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDetached(BaseModel):
    """
    create_detached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyDetached(BaseModel):
    """
    verify_detached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignContainer(BaseModel):
    """
    sign_container
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyContainer(BaseModel):
    """
    verify_container
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignJar(BaseModel):
    """
    sign_jar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyJar(BaseModel):
    """
    verify_jar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Timestamp(BaseModel):
    """
    timestamp
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignatureMetadata(BaseModel):
    """
    signature_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SignedArtifact(BaseModel):
    """
    signed_artifact
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


