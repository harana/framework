# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GoogleGeminiModel(BaseModel):
    """
    google_gemini_model
    
    ID fields: model_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    display_name: Optional[str] = None
    input_token_limit: Optional[int] = None
    model_id: str
    output_token_limit: Optional[int] = None
    supported_generation_methods: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiSafetySetting(BaseModel):
    """
    google_gemini_safety_setting
    
    ID fields: model_id, category
    """

    category: str
    model_id: str = Field(description="Reference: google_gemini_model.id")
    threshold: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiSafetyRating(BaseModel):
    """
    google_gemini_safety_rating
    
    ID fields: id
    """

    blocked: bool = Field(default=False)
    category: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    probability: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiChatMessage(BaseModel):
    """
    google_gemini_chat_message
    
    ID fields: conversation_id, created_at
    """

    content: str
    conversation_id: str = Field(description="Reference: google_gemini_conversation.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    role: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiConversation(BaseModel):
    """
    google_gemini_conversation
    
    ID fields: user_id, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    model_id: str = Field(description="Reference: google_gemini_model.id")
    status: str = Field(default="active")
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiEmbedding(BaseModel):
    """
    google_gemini_embedding
    
    ID fields: model_id, content_hash
    """

    content_hash: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dimensions: Optional[int] = None
    embedding: Optional[str] = None
    model_id: str = Field(description="Reference: google_gemini_model.id")
    task_type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiUsage(BaseModel):
    """
    google_gemini_usage
    
    ID fields: id
    """

    candidates_token_count: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    model_id: str = Field(description="Reference: google_gemini_model.id")
    prompt_token_count: Optional[int] = None
    total_token_count: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GoogleGeminiParameterRange(BaseModel):
    """
    google_gemini_parameter_range
    
    ID fields: model_id, parameter_name
    """

    max_value: Optional[float] = None
    min_value: Optional[float] = None
    model_id: str = Field(description="Reference: google_gemini_model.id")
    parameter_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiModel(BaseModel):
    """
    gemini_model
    
    ID fields: id
    """

    model_id: str
    name: str
    version: str
    input_token_limit: int
    output_token_limit: int
    supported_generation_methods: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiSafetySetting(BaseModel):
    """
    gemini_safety_setting
    
    ID fields: id
    """

    category: str
    threshold: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiSafetyRating(BaseModel):
    """
    gemini_safety_rating
    
    ID fields: id
    """

    category: str
    probability: str
    blocked: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiUsage(BaseModel):
    """
    gemini_usage
    
    ID fields: id
    """

    prompt_token_count: int
    candidates_token_count: int
    total_token_count: int
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiChatMessage(BaseModel):
    """
    gemini_chat_message
    
    ID fields: id
    """

    role: str
    content: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiEmbedding(BaseModel):
    """
    gemini_embedding
    
    ID fields: id
    """

    values: List[float]
    dimensions: int
    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiParameterRange(BaseModel):
    """
    gemini_parameter_range
    
    ID fields: id
    """

    min: float
    max: float
    default: float
    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractSchema(BaseModel):
    """
    extract_schema
    
    ID fields: id
    """

    fields: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractSchemaField(BaseModel):
    """
    extract_schema_field
    
    ID fields: id
    """

    name: str
    type: str
    description: str
    required: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractedData(BaseModel):
    """
    extracted_data
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


