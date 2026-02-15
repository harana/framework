# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateSchedule(BaseModel):
    """
    create_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateSchedule(BaseModel):
    """
    update_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSchedule(BaseModel):
    """
    delete_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EnableSchedule(BaseModel):
    """
    enable_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisableSchedule(BaseModel):
    """
    disable_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSchedule(BaseModel):
    """
    get_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListSchedules(BaseModel):
    """
    list_schedules
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TriggerSchedule(BaseModel):
    """
    trigger_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetNextRun(BaseModel):
    """
    get_next_run
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidateCron(BaseModel):
    """
    validate_cron
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateOneTime(BaseModel):
    """
    create_one_time
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PauseSchedule(BaseModel):
    """
    pause_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResumeSchedule(BaseModel):
    """
    resume_schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetScheduleHistory(BaseModel):
    """
    get_schedule_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetScheduleStats(BaseModel):
    """
    get_schedule_stats
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkEnable(BaseModel):
    """
    bulk_enable
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkDisable(BaseModel):
    """
    bulk_disable
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateInterval(BaseModel):
    """
    create_interval
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateAction(BaseModel):
    """
    update_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Schedule(BaseModel):
    """
    schedule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleMetadata(BaseModel):
    """
    schedule_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleInfo(BaseModel):
    """
    schedule_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleExecution(BaseModel):
    """
    schedule_execution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleBulkResult(BaseModel):
    """
    schedule_bulk_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleActionConfig(BaseModel):
    """
    schedule_action_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


