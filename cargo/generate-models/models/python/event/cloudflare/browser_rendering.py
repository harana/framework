# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareBrowserPageRendered(BaseModel):
    """
    cloudflare_browser_page_rendered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareBrowserScreenshotTaken(BaseModel):
    """
    cloudflare_browser_screenshot_taken
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareBrowserContentExtracted(BaseModel):
    """
    cloudflare_browser_content_extracted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareBrowserPdfGenerated(BaseModel):
    """
    cloudflare_browser_pdf_generated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


