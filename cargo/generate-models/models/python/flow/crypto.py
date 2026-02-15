# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Hash(BaseModel):
    """
    hash
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Hmac(BaseModel):
    """
    hmac
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Encrypt(BaseModel):
    """
    encrypt
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Decrypt(BaseModel):
    """
    decrypt
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateKey(BaseModel):
    """
    generate_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Sign(BaseModel):
    """
    sign
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Verify(BaseModel):
    """
    verify
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateKeypair(BaseModel):
    """
    generate_keypair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class KeyPair(BaseModel):
    """
    key_pair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


