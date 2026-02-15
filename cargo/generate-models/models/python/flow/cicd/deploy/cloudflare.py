# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployCloudflare(BaseModel):
    """
    deploy_cloudflare
    
    ID fields: id
    """

    account_id_env: str
    api_token_env: str
    directory: Optional[str] = None
    enabled: bool = Field(default=True)
    environment: str = Field(default="production")
    project_name: str
    route: Optional[str] = None
    workers: List[str]
    zone_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployCloudflareWorker(BaseModel):
    """
    deploy_cloudflare_worker
    
    ID fields: id
    """

    compatibility_date: Optional[str] = None
    entry_point: str
    name: str
    routes: List[str]
    vars: str
    class Config:
        from_attributes = True
        populate_by_name = True


