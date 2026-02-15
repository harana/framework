# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployNetlify(BaseModel):
    """
    deploy_netlify
    
    ID fields: id
    """

    deploy_message: Optional[str] = None
    enabled: bool = Field(default=True)
    functions_dir: Optional[str] = None
    production: bool = Field(default=True)
    site_id: str
    source_dir: str
    token_env: str
    class Config:
        from_attributes = True
        populate_by_name = True


