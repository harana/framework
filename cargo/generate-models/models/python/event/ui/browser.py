# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class BrowserVisibilityHidden(BaseModel):
    """
    browser_visibility_hidden
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserVisibilityVisible(BaseModel):
    """
    browser_visibility_visible
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserPageHide(BaseModel):
    """
    browser_page_hide
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserPageShow(BaseModel):
    """
    browser_page_show
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserBeforeUnload(BaseModel):
    """
    browser_before_unload
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserOnline(BaseModel):
    """
    browser_online
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserOffline(BaseModel):
    """
    browser_offline
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserException(BaseModel):
    """
    browser_exception
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserPromiseRejection(BaseModel):
    """
    browser_promise_rejection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserLcp(BaseModel):
    """
    browser_lcp
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserFid(BaseModel):
    """
    browser_fid
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserCls(BaseModel):
    """
    browser_cls
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserLongTask(BaseModel):
    """
    browser_long_task
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserResize(BaseModel):
    """
    browser_resize
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserOrientationChange(BaseModel):
    """
    browser_orientation_change
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserFullscreenEnter(BaseModel):
    """
    browser_fullscreen_enter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserFullscreenExit(BaseModel):
    """
    browser_fullscreen_exit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserStorageChange(BaseModel):
    """
    browser_storage_change
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserFocusGained(BaseModel):
    """
    browser_focus_gained
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserFocusLost(BaseModel):
    """
    browser_focus_lost
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserIdleStart(BaseModel):
    """
    browser_idle_start
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserIdleEnd(BaseModel):
    """
    browser_idle_end
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserBack(BaseModel):
    """
    browser_back
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserForward(BaseModel):
    """
    browser_forward
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserScrollDepth25(BaseModel):
    """
    browser_scroll_depth_25
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserScrollDepth50(BaseModel):
    """
    browser_scroll_depth_50
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserScrollDepth75(BaseModel):
    """
    browser_scroll_depth_75
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserScrollDepth100(BaseModel):
    """
    browser_scroll_depth_100
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserElementVisible(BaseModel):
    """
    browser_element_visible
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserRageClick(BaseModel):
    """
    browser_rage_click
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserMediaPlay(BaseModel):
    """
    browser_media_play
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserMediaPause(BaseModel):
    """
    browser_media_pause
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserMediaEnded(BaseModel):
    """
    browser_media_ended
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrowserMediaProgress(BaseModel):
    """
    browser_media_progress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


