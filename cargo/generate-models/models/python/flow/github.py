# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateRepo(BaseModel):
    """
    create_repo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRepo(BaseModel):
    """
    get_repo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateRepo(BaseModel):
    """
    update_repo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRepo(BaseModel):
    """
    delete_repo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRepos(BaseModel):
    """
    list_repos
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateIssue(BaseModel):
    """
    create_issue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateIssue(BaseModel):
    """
    update_issue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloseIssue(BaseModel):
    """
    close_issue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListIssues(BaseModel):
    """
    list_issues
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePullRequest(BaseModel):
    """
    create_pull_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdatePullRequest(BaseModel):
    """
    update_pull_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MergePullRequest(BaseModel):
    """
    merge_pull_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListPullRequests(BaseModel):
    """
    list_pull_requests
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateBranch(BaseModel):
    """
    create_branch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteBranch(BaseModel):
    """
    delete_branch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListBranches(BaseModel):
    """
    list_branches
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCommit(BaseModel):
    """
    get_commit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCommits(BaseModel):
    """
    list_commits
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRelease(BaseModel):
    """
    create_release
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRelease(BaseModel):
    """
    get_release
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListReleases(BaseModel):
    """
    list_releases
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateWebhook(BaseModel):
    """
    create_webhook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteWebhook(BaseModel):
    """
    delete_webhook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFileContents(BaseModel):
    """
    get_file_contents
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateOrUpdateFile(BaseModel):
    """
    create_or_update_file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFile(BaseModel):
    """
    delete_file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubRepository(BaseModel):
    """
    github_repository
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Author(BaseModel):
    """
    author
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Committer(BaseModel):
    """
    committer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Commit(BaseModel):
    """
    commit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FileContent(BaseModel):
    """
    file_content
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WebhookConfig(BaseModel):
    """
    webhook_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubIssue(BaseModel):
    """
    github_issue
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubPullRequest(BaseModel):
    """
    github_pull_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubBranch(BaseModel):
    """
    github_branch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubCommit(BaseModel):
    """
    github_commit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GithubRelease(BaseModel):
    """
    github_release
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


