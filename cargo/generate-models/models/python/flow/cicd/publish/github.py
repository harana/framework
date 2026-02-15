# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class BuildGithubRelease(BaseModel):
    """
    build_github_release
    
    ID fields: id
    """

    assets: List[str]
    body: Optional[str] = None
    discussion_category: Optional[str] = None
    draft: bool = Field(default=False)
    enabled: bool = Field(default=True)
    generate_release_notes: bool = Field(default=True)
    overwrite: bool = Field(default=False)
    prerelease: bool = Field(default=False)
    release_name: str
    repository: str
    tag: str
    token_env: str
    class Config:
        from_attributes = True
        populate_by_name = True


class BuildGithubReleaseAsset(BaseModel):
    """
    build_github_release_asset
    
    ID fields: id
    """

    content_type: str
    label: Optional[str] = None
    path: str
    class Config:
        from_attributes = True
        populate_by_name = True


