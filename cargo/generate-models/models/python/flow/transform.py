# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class JsonToXml(BaseModel):
    """
    json_to_xml
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class XmlToJson(BaseModel):
    """
    xml_to_json
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CsvToJson(BaseModel):
    """
    csv_to_json
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonToCsv(BaseModel):
    """
    json_to_csv
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class YamlToJson(BaseModel):
    """
    yaml_to_json
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonToYaml(BaseModel):
    """
    json_to_yaml
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Base64Encode(BaseModel):
    """
    base64_encode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Base64Decode(BaseModel):
    """
    base64_decode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UrlEncode(BaseModel):
    """
    url_encode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UrlDecode(BaseModel):
    """
    url_decode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlEncode(BaseModel):
    """
    html_encode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlDecode(BaseModel):
    """
    html_decode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MarkdownToHtml(BaseModel):
    """
    markdown_to_html
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TransformResult(BaseModel):
    """
    transform_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TransformJsonObject(BaseModel):
    """
    transform_json_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MarkdownOptions(BaseModel):
    """
    markdown_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


