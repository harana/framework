# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SfEmployee(BaseModel):
    """
    sf_employee
    
    ID fields: user_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    department: Optional[str] = None
    email: str
    first_name: str
    hire_date: Optional[datetime] = None
    job_title: Optional[str] = None
    last_name: str
    manager_id: Optional[str] = None
    status: str = Field(default="active")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SfJobInfo(BaseModel):
    """
    sf_job_info
    
    ID fields: employee_id, effective_start_date
    """

    cost_center: Optional[str] = None
    department: Optional[str] = None
    division: Optional[str] = None
    effective_start_date: datetime
    employee_id: str = Field(description="Reference: sf_employee.id")
    employment_type: Optional[str] = None
    job_code: Optional[str] = None
    job_title: Optional[str] = None
    location: Optional[str] = None
    manager_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class SfCompensation(BaseModel):
    """
    sf_compensation
    
    ID fields: employee_id, effective_start_date
    """

    bonus_target: Optional[float] = None
    currency: Optional[str] = None
    effective_start_date: datetime
    employee_id: str = Field(description="Reference: sf_employee.id")
    frequency: Optional[str] = None
    pay_grade: Optional[str] = None
    pay_type: Optional[str] = None
    salary: Optional[float] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class SfTimeOffRequest(BaseModel):
    """
    sf_time_off_request
    
    ID fields: request_id
    """

    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_hours: Optional[float] = None
    duration_type: str = Field(default="full_day")
    employee_id: str = Field(description="Reference: sf_employee.id")
    end_date: datetime
    request_id: str
    start_date: datetime
    status: str = Field(default="pending")
    time_type: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class SfTimeOffBalance(BaseModel):
    """
    sf_time_off_balance
    
    ID fields: employee_id, time_type
    """

    as_of_date: datetime = Field(default_factory=datetime.utcnow)
    balance: float = Field(default=0)
    employee_id: str = Field(description="Reference: sf_employee.id")
    time_type: str
    unit: str = Field(default="days")
    class Config:
        from_attributes = True
        populate_by_name = True


class SfDepartment(BaseModel):
    """
    sf_department
    
    ID fields: department_id
    """

    cost_center: Optional[str] = None
    department_id: str
    description: Optional[str] = None
    head_id: Optional[str] = None
    name: str
    parent_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


