# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AIModel(BaseModel):
    """
    AI Model
    
    Class: ai_model
    ID fields: name, version
    """

    context_window: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    max_output_tokens: Optional[int] = None
    provider: str = Field(default="openai")
    type: str = Field(default="chat")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AIModelConfiguration(BaseModel):
    """
    AI Model Configuration
    
    Class: ai_model_config
    ID fields: model_id, name
    """

    frequency_penalty: float = Field(default=0)
    max_tokens: Optional[int] = None
    model_id: str
    presence_penalty: float = Field(default=0)
    stop_sequences: Optional[str] = None
    temperature: float = Field(default="1.0")
    top_p: float = Field(default="1.0")
    class Config:
        from_attributes = True
        populate_by_name = True


class AIPromptTemplate(BaseModel):
    """
    AI Prompt Template
    
    Class: ai_prompt_template
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    model_id: Optional[str] = Field(default=None, description="Reference: AIModel.id")
    system_prompt: Optional[str] = None
    template: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    variables: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AIConversation(BaseModel):
    """
    AI Conversation
    
    Class: ai_conversation
    ID fields: user_id, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    metadata: Optional[str] = None
    model_id: str
    status: str = Field(default="active")
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AIMessage(BaseModel):
    """
    AI Message
    
    Class: ai_message
    ID fields: conversation_id, created_at
    """

    content: str
    conversation_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    metadata: Optional[str] = None
    role: str = Field(default="user")
    tokens_used: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AIEmbedding(BaseModel):
    """
    AI Embedding
    
    Class: ai_embedding
    ID fields: model_id, source_id, source_type
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dimensions: int
    metadata: Optional[str] = None
    model_id: str
    source_id: Optional[str] = None
    source_type: Optional[str] = None
    vector: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AIUsage(BaseModel):
    """
    AI Usage
    
    Class: ai_usage
    ID fields: model_id, created_at, user_id
    """

    completion_tokens: int = Field(default=0)
    cost: Optional[float] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    model_id: str
    prompt_tokens: int = Field(default=0)
    request_type: str = Field(default="chat")
    total_tokens: int = Field(default=0)
    user_id: Optional[str] = Field(default=None, description="Reference: User.id")
    class Config:
        from_attributes = True
        populate_by_name = True


