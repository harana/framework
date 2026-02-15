# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Run(BaseModel):
    """
    run
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextGeneration(BaseModel):
    """
    text_generation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextEmbeddings(BaseModel):
    """
    text_embeddings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextClassification(BaseModel):
    """
    text_classification
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Translation(BaseModel):
    """
    translation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageClassification(BaseModel):
    """
    image_classification
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ObjectDetection(BaseModel):
    """
    object_detection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextToImage(BaseModel):
    """
    text_to_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SpeechRecognition(BaseModel):
    """
    speech_recognition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextToSpeech(BaseModel):
    """
    text_to_speech
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Summarization(BaseModel):
    """
    summarization
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageToText(BaseModel):
    """
    image_to_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


