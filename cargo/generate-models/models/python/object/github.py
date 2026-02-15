# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GithubRepository(BaseModel):
    """
    github_repository
    
    ID fields: full_name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_branch: str = Field(default="main")
    description: Optional[str] = None
    fork: bool = Field(default=False)
    full_name: str
    has_issues: bool = Field(default=True)
    has_wiki: bool = Field(default=True)
    homepage: Optional[str] = None
    is_private: bool = Field(default=False)
    repo_id: Optional[int] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubIssue(BaseModel):
    """
    github_issue
    
    ID fields: repository_id, issue_number
    """

    assignees: Optional[str] = None
    body: Optional[str] = None
    closed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    issue_number: int
    labels: Optional[str] = None
    repository_id: str = Field(description="Reference: git_hub_repository.id")
    state: str = Field(default="open")
    title: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubPullRequest(BaseModel):
    """
    github_pull_request
    
    ID fields: repository_id, pr_number
    """

    base_branch: str
    body: Optional[str] = None
    closed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    head_branch: str
    is_draft: bool = Field(default=False)
    merged_at: Optional[datetime] = None
    pr_number: int
    repository_id: str = Field(description="Reference: git_hub_repository.id")
    state: str = Field(default="open")
    title: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubWebhook(BaseModel):
    """
    github_webhook
    
    ID fields: repository_id, hook_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    events: str
    hook_id: int
    is_active: bool = Field(default=True)
    repository_id: str = Field(description="Reference: git_hub_repository.id")
    secret: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


