# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SendEmail(BaseModel):
    """
    send_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendRawEmail(BaseModel):
    """
    send_raw_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendTemplatedEmail(BaseModel):
    """
    send_templated_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendBulkTemplatedEmail(BaseModel):
    """
    send_bulk_templated_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyEmailIdentity(BaseModel):
    """
    verify_email_identity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyDomainIdentity(BaseModel):
    """
    verify_domain_identity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteIdentity(BaseModel):
    """
    delete_identity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListIdentities(BaseModel):
    """
    list_identities
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetIdentityVerificationAttributes(BaseModel):
    """
    get_identity_verification_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetIdentityNotificationTopic(BaseModel):
    """
    set_identity_notification_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTemplate(BaseModel):
    """
    create_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateTemplate(BaseModel):
    """
    update_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTemplate(BaseModel):
    """
    delete_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTemplate(BaseModel):
    """
    get_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTemplates(BaseModel):
    """
    list_templates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSendQuota(BaseModel):
    """
    get_send_quota
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSendStatistics(BaseModel):
    """
    get_send_statistics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutConfigurationSet(BaseModel):
    """
    put_configuration_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteConfigurationSet(BaseModel):
    """
    delete_configuration_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListConfigurationSets(BaseModel):
    """
    list_configuration_sets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutIdentityPolicy(BaseModel):
    """
    put_identity_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetIdentityPolicies(BaseModel):
    """
    get_identity_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteIdentityPolicy(BaseModel):
    """
    delete_identity_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetIdentityFeedbackForwarding(BaseModel):
    """
    set_identity_feedback_forwarding
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetIdentityDkimEnabled(BaseModel):
    """
    set_identity_dkim_enabled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetIdentityDkimAttributes(BaseModel):
    """
    get_identity_dkim_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


