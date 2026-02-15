# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployAzureBlob(BaseModel):
    """
    deploy_azure_blob
    
    ID fields: id
    """

    cdn: Optional[str] = None
    container: str
    content_type_mapping: str
    credentials_env: Optional[str] = None
    destination_prefix: Optional[str] = None
    enabled: bool = Field(default=True)
    public_access: Optional[str] = None
    source_dir: str
    storage_account: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployAzureBlobCdn(BaseModel):
    """
    deploy_azure_blob_cdn
    
    ID fields: id
    """

    enabled: bool = Field(default=True)
    endpoint: str
    profile: str
    purge_content: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployAzureBlobCredentials(BaseModel):
    """
    deploy_azure_blob_credentials
    
    ID fields: id
    """

    azure_storage_connection_string: str
    class Config:
        from_attributes = True
        populate_by_name = True


