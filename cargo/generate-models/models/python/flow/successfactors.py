# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GetEmployee(BaseModel):
    """
    get_employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListEmployees(BaseModel):
    """
    list_employees
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateEmployee(BaseModel):
    """
    create_employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateEmployee(BaseModel):
    """
    update_employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TerminateEmployee(BaseModel):
    """
    terminate_employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetJobInfo(BaseModel):
    """
    get_job_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListJobHistory(BaseModel):
    """
    list_job_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCompensation(BaseModel):
    """
    get_compensation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCompensationHistory(BaseModel):
    """
    list_compensation_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTimeOffBalance(BaseModel):
    """
    get_time_off_balance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RequestTimeOff(BaseModel):
    """
    request_time_off
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTimeOffRequests(BaseModel):
    """
    list_time_off_requests
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ApproveTimeOff(BaseModel):
    """
    approve_time_off
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RejectTimeOff(BaseModel):
    """
    reject_time_off
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetOrgChart(BaseModel):
    """
    get_org_chart
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDirectReports(BaseModel):
    """
    get_direct_reports
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListDepartments(BaseModel):
    """
    list_departments
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDepartment(BaseModel):
    """
    get_department
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListPositions(BaseModel):
    """
    list_positions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPosition(BaseModel):
    """
    get_position
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Employee(BaseModel):
    """
    employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfEmployee(BaseModel):
    """
    sf_employee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfJobHistoryEntry(BaseModel):
    """
    sf_job_history_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfCompensationEntry(BaseModel):
    """
    sf_compensation_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfTimeOffBalance(BaseModel):
    """
    sf_time_off_balance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfTimeOffRequest(BaseModel):
    """
    sf_time_off_request
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfOrgChartNode(BaseModel):
    """
    sf_org_chart_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfDepartment(BaseModel):
    """
    sf_department
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SfPosition(BaseModel):
    """
    sf_position
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


