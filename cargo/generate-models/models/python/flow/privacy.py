# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RecordConsent(BaseModel):
    """
    record_consent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RevokeConsent(BaseModel):
    """
    revoke_consent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetConsentStatus(BaseModel):
    """
    get_consent_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateConsent(BaseModel):
    """
    update_consent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExportData(BaseModel):
    """
    export_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetExportStatus(BaseModel):
    """
    get_export_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteData(BaseModel):
    """
    delete_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDeletionStatus(BaseModel):
    """
    get_deletion_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelDeletion(BaseModel):
    """
    cancel_deletion
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AnonymizeData(BaseModel):
    """
    anonymize_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListConsentHistory(BaseModel):
    """
    list_consent_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RecordDataAccess(BaseModel):
    """
    record_data_access
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAccessLog(BaseModel):
    """
    get_access_log
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePolicyVersion(BaseModel):
    """
    create_policy_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetActivePolicy(BaseModel):
    """
    get_active_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RecordPolicyAcceptance(BaseModel):
    """
    record_policy_acceptance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyRtbfEligibility(BaseModel):
    """
    verify_rtbf_eligibility
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Consent(BaseModel):
    """
    consent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyConsent(BaseModel):
    """
    privacy_consent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyConsentHistory(BaseModel):
    """
    privacy_consent_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyDataAccess(BaseModel):
    """
    privacy_data_access
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


