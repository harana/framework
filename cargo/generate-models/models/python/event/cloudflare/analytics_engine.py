# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareAnalyticsDataPointWritten(BaseModel):
    """
    cloudflare_analytics_data_point_written
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareAnalyticsQueried(BaseModel):
    """
    cloudflare_analytics_queried
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


