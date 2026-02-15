# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GenerateContent(BaseModel):
    """
    generate_content
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Chat(BaseModel):
    """
    chat
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateMultimodal(BaseModel):
    """
    generate_multimodal
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StreamGenerate(BaseModel):
    """
    stream_generate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmbedContent(BaseModel):
    """
    embed_content
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BatchEmbed(BaseModel):
    """
    batch_embed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CountTokens(BaseModel):
    """
    count_tokens
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListModels(BaseModel):
    """
    list_models
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetModel(BaseModel):
    """
    get_model
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AnalyzeImage(BaseModel):
    """
    analyze_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateCode(BaseModel):
    """
    generate_code
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Summarize(BaseModel):
    """
    summarize
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Translate(BaseModel):
    """
    translate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractStructured(BaseModel):
    """
    extract_structured
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiModel(BaseModel):
    """
    gemini_model
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiSafetySetting(BaseModel):
    """
    gemini_safety_setting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiSafetyRating(BaseModel):
    """
    gemini_safety_rating
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiUsage(BaseModel):
    """
    gemini_usage
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiChatMessage(BaseModel):
    """
    gemini_chat_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiEmbedding(BaseModel):
    """
    gemini_embedding
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeminiParameterRange(BaseModel):
    """
    gemini_parameter_range
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractSchema(BaseModel):
    """
    extract_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractSchemaField(BaseModel):
    """
    extract_schema_field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractedData(BaseModel):
    """
    extracted_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


