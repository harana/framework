# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployHeroku(BaseModel):
    """
    deploy_heroku
    
    ID fields: id
    """

    addons: List[str]
    app_name: str
    buildpacks: List[str]
    config_vars: str
    enabled: bool = Field(default=True)
    post_deploy: List[str]
    pre_deploy: List[str]
    region: str = Field(default="us")
    rollback_on_failure: bool = Field(default=True)
    run_migrations: bool = Field(default=True)
    scale: str
    stack: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


