# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class LogEvent(BaseModel):
    """
    log_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueryLogs(BaseModel):
    """
    query_logs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetEvent(BaseModel):
    """
    get_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetActorActivity(BaseModel):
    """
    get_actor_activity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetResourceHistory(BaseModel):
    """
    get_resource_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExportLogs(BaseModel):
    """
    export_logs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateAlert(BaseModel):
    """
    create_alert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteAlert(BaseModel):
    """
    delete_alert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListAlerts(BaseModel):
    """
    list_alerts
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetStatistics(BaseModel):
    """
    get_statistics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetRetentionPolicy(BaseModel):
    """
    set_retention_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRetentionPolicy(BaseModel):
    """
    get_retention_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyIntegrity(BaseModel):
    """
    verify_integrity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditEvent(BaseModel):
    """
    audit_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditActivity(BaseModel):
    """
    audit_activity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditFilter(BaseModel):
    """
    audit_filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditAlertCondition(BaseModel):
    """
    audit_alert_condition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditAlert(BaseModel):
    """
    audit_alert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuditStatistic(BaseModel):
    """
    audit_statistic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


