# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RecordMetric(BaseModel):
    """
    record_metric
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IncrementCounter(BaseModel):
    """
    increment_counter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetGauge(BaseModel):
    """
    set_gauge
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RecordHistogram(BaseModel):
    """
    record_histogram
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartTimer(BaseModel):
    """
    start_timer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StopTimer(BaseModel):
    """
    stop_timer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueryMetrics(BaseModel):
    """
    query_metrics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetMetricSummary(BaseModel):
    """
    get_metric_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListMetrics(BaseModel):
    """
    list_metrics
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMetric(BaseModel):
    """
    delete_metric
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Metric(BaseModel):
    """
    metric
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MetricDatapoint(BaseModel):
    """
    metric_datapoint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MetricInfo(BaseModel):
    """
    metric_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


