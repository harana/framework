# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareHyperdriveConnected(BaseModel):
    """
    cloudflare_hyperdrive_connected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareHyperdriveConnectionFailed(BaseModel):
    """
    cloudflare_hyperdrive_connection_failed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


