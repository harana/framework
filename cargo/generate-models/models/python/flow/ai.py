# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GenerateText(BaseModel):
    """
    generate_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChatCompletion(BaseModel):
    """
    chat_completion
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateEmbeddings(BaseModel):
    """
    generate_embeddings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateImage(BaseModel):
    """
    generate_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TranscribeAudio(BaseModel):
    """
    transcribe_audio
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ClassifyText(BaseModel):
    """
    classify_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SummarizeText(BaseModel):
    """
    summarize_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractEntities(BaseModel):
    """
    extract_entities
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxLoadModel(BaseModel):
    """
    onnx_load_model
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxInference(BaseModel):
    """
    onnx_inference
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxInferenceBytes(BaseModel):
    """
    onnx_inference_bytes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxModelInfo(BaseModel):
    """
    onnx_model_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxUnloadModel(BaseModel):
    """
    onnx_unload_model
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxClassifyImage(BaseModel):
    """
    onnx_classify_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxDetectObjects(BaseModel):
    """
    onnx_detect_objects
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OnnxTextEmbedding(BaseModel):
    """
    onnx_text_embedding
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiModel(BaseModel):
    """
    ai_model
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiChatMessage(BaseModel):
    """
    ai_chat_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiEntity(BaseModel):
    """
    ai_entity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiTensorShape(BaseModel):
    """
    ai_tensor_shape
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiClassificationPrediction(BaseModel):
    """
    ai_classification_prediction
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AiObjectDetection(BaseModel):
    """
    ai_object_detection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


