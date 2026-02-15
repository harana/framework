# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployGithubPages(BaseModel):
    """
    deploy_github_pages
    
    ID fields: id
    """

    branch: str = Field(default="gh-pages")
    cname: Optional[str] = None
    commit_message: Optional[str] = None
    enabled: bool = Field(default=True)
    force: bool = Field(default=False)
    keep_history: bool = Field(default=True)
    repository: str
    source_dir: str
    token_env: str
    user: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployGithubPagesUser(BaseModel):
    """
    deploy_github_pages_user
    
    ID fields: id
    """

    email: str
    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


