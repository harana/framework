# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GitRepository(BaseModel):
    """
    git_repository
    
    ID fields: full_name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_branch: str = Field(default="main")
    description: Optional[str] = None
    full_name: str
    is_private: bool = Field(default=False)
    provider: str = Field(default="github")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GitBranch(BaseModel):
    """
    git_branch
    
    ID fields: repository_id, name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_protected: bool = Field(default=False)
    name: str
    repository_id: str = Field(description="Reference: git_repository.id")
    sha: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GitCommit(BaseModel):
    """
    git_commit
    
    ID fields: repository_id, sha
    """

    author_email: Optional[str] = None
    author_name: Optional[str] = None
    committed_at: datetime = Field(default_factory=datetime.utcnow)
    message: Optional[str] = None
    repository_id: str = Field(description="Reference: git_repository.id")
    sha: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GitTag(BaseModel):
    """
    git_tag
    
    ID fields: repository_id, name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    message: Optional[str] = None
    name: str
    repository_id: str = Field(description="Reference: git_repository.id")
    sha: str
    class Config:
        from_attributes = True
        populate_by_name = True


