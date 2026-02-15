# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareWorkerFetchInvoked(BaseModel):
    """
    cloudflare_worker_fetch_invoked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerRouteMatched(BaseModel):
    """
    cloudflare_worker_route_matched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerCronTriggered(BaseModel):
    """
    cloudflare_worker_cron_triggered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerErrorOccurred(BaseModel):
    """
    cloudflare_worker_error_occurred
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerVersionDeployed(BaseModel):
    """
    cloudflare_worker_version_deployed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareServiceBindingFetchInvoked(BaseModel):
    """
    cloudflare_service_binding_fetch_invoked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


