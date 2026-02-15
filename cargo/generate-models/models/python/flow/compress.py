# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GzipCompress(BaseModel):
    """
    gzip_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GzipDecompress(BaseModel):
    """
    gzip_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZstdCompress(BaseModel):
    """
    zstd_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZstdDecompress(BaseModel):
    """
    zstd_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrotliCompress(BaseModel):
    """
    brotli_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BrotliDecompress(BaseModel):
    """
    brotli_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Lz4Compress(BaseModel):
    """
    lz4_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Lz4Decompress(BaseModel):
    """
    lz4_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeflateCompress(BaseModel):
    """
    deflate_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeflateDecompress(BaseModel):
    """
    deflate_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AutoCompress(BaseModel):
    """
    auto_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AutoDecompress(BaseModel):
    """
    auto_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StreamCompress(BaseModel):
    """
    stream_compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StreamDecompress(BaseModel):
    """
    stream_decompress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CompressedData(BaseModel):
    """
    compressed_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


