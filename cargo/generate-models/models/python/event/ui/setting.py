# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SettingChanged(BaseModel):
    """
    setting_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PreferenceToggled(BaseModel):
    """
    preference_toggled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ThemeSwitched(BaseModel):
    """
    theme_switched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LanguageChanged(BaseModel):
    """
    language_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


