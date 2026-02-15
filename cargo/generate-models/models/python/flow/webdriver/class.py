# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebDriverCapabilities(BaseModel):
    """
    web_driver_capabilities
    
    ID fields: id
    """

    accept_insecure_certs: bool
    browser_name: str
    browser_version: str
    extra: str
    headless: bool
    page_load_strategy: str
    platform_name: str
    proxy: str
    window_height: int
    window_width: int
    class Config:
        from_attributes = True
        populate_by_name = True


class WebDriverProxy(BaseModel):
    """
    web_driver_proxy
    
    ID fields: id
    """

    auto_detect: bool
    ftp_proxy: str
    http_proxy: str
    no_proxy: List[str]
    proxy_type: str
    ssl_proxy: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebDriverElement(BaseModel):
    """
    web_driver_element
    
    ID fields: id
    """

    class_name: str
    element_id: str
    id: str
    tag_name: str
    text: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebDriverCookie(BaseModel):
    """
    web_driver_cookie
    
    ID fields: id
    """

    domain: str
    expiry: int
    http_only: bool
    name: str
    path: str
    same_site: str
    secure: bool
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebDriverAction(BaseModel):
    """
    web_driver_action
    
    ID fields: id
    """

    action_type: str
    duration_ms: int
    element_id: str
    key: str
    x: int
    y: int
    class Config:
        from_attributes = True
        populate_by_name = True


class XPathQueryResult(BaseModel):
    """
    x_path_query_result
    
    ID fields: id
    """

    elements: List[str]
    error: str
    found: bool
    total: int
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ElementRect(BaseModel):
    """
    element_rect
    
    ID fields: id
    """

    height: float
    width: float
    x: float
    y: float
    class Config:
        from_attributes = True
        populate_by_name = True


class WindowRect(BaseModel):
    """
    window_rect
    
    ID fields: id
    """

    height: int
    width: int
    x: int
    y: int
    class Config:
        from_attributes = True
        populate_by_name = True


