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


class GithubRepository(BaseModel):
    """
    github_repository
    
    ID fields: id
    """

    repo_id: int
    name: str
    full_name: str
    description: str
    url: str
    private: bool
    default_branch: str
    fork: bool
    created_at: datetime
    updated_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class Author(BaseModel):
    """
    author
    
    ID fields: id
    """

    name: str
    email: str
    date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class Committer(BaseModel):
    """
    committer
    
    ID fields: id
    """

    name: str
    email: str
    date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class Commit(BaseModel):
    """
    commit
    
    ID fields: id
    """

    sha: str
    message: str
    url: str
    author: str
    committer: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FileContent(BaseModel):
    """
    file_content
    
    ID fields: id
    """

    name: str
    path: str
    sha: str
    size: int
    url: str
    content: str
    encoding: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebhookConfig(BaseModel):
    """
    webhook_config
    
    ID fields: id
    """

    url: str
    content_type: str
    secret: str
    insecure_ssl: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubIssue(BaseModel):
    """
    github_issue
    
    ID fields: id
    """

    issue_number: int
    title: str
    body: str
    state: str
    labels: List[str]
    assignees: List[str]
    milestone: int
    url: str
    created_at: datetime
    updated_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubPullRequest(BaseModel):
    """
    github_pull_request
    
    ID fields: id
    """

    pr_number: int
    title: str
    body: str
    state: str
    head: str
    base: str
    draft: bool
    merged: bool
    url: str
    created_at: datetime
    updated_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubBranch(BaseModel):
    """
    github_branch
    
    ID fields: id
    """

    name: str
    sha: str
    protected: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubCommit(BaseModel):
    """
    github_commit
    
    ID fields: id
    """

    sha: str
    message: str
    url: str
    author: str
    committer: str
    created_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class GithubRelease(BaseModel):
    """
    github_release
    
    ID fields: id
    """

    release_id: int
    tag_name: str
    name: str
    body: str
    draft: bool
    prerelease: bool
    created_at: datetime
    published_at: datetime
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


