# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateApprovalRequest(BaseModel):
    """
    Create Approval Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetApprovalRequest(BaseModel):
    """
    Get Approval Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListApprovalRequests(BaseModel):
    """
    List Approval Requests
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApproveRequest(BaseModel):
    """
    Approve Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RejectRequest(BaseModel):
    """
    Reject Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelApprovalRequest(BaseModel):
    """
    Cancel Approval Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddApproverToRequest(BaseModel):
    """
    Add Approver To Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveApproverFromRequest(BaseModel):
    """
    Remove Approver From Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetApprovalHistory(BaseModel):
    """
    Get Approval History
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EscalateApprovalRequest(BaseModel):
    """
    Escalate Approval Request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DelegateApproval(BaseModel):
    """
    Delegate Approval
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApprovalRequest(BaseModel):
    """
    ApprovalRequest
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApproverInfo(BaseModel):
    """
    ApproverInfo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApprovalRequestInfo(BaseModel):
    """
    ApprovalRequestInfo
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApprovalEvent(BaseModel):
    """
    ApprovalEvent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


