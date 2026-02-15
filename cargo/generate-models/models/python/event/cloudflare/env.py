# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareEnvVariableRead(BaseModel):
    """
    cloudflare_env_variable_read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareSecretRead(BaseModel):
    """
    cloudflare_secret_read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareSecretsStoreSecretRead(BaseModel):
    """
    cloudflare_secrets_store_secret_read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareEnvVariablesListed(BaseModel):
    """
    cloudflare_env_variables_listed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


