# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FromHtml(BaseModel):
    """
    from_html
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FromTemplate(BaseModel):
    """
    from_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Merge(BaseModel):
    """
    merge
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Split(BaseModel):
    """
    split
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractPages(BaseModel):
    """
    extract_pages
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractText(BaseModel):
    """
    extract_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractImages(BaseModel):
    """
    extract_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddWatermark(BaseModel):
    """
    add_watermark
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Compress(BaseModel):
    """
    compress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Encrypt(BaseModel):
    """
    encrypt
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Decrypt(BaseModel):
    """
    decrypt
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetMetadata(BaseModel):
    """
    get_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetMetadata(BaseModel):
    """
    set_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RotatePages(BaseModel):
    """
    rotate_pages
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddPageNumbers(BaseModel):
    """
    add_page_numbers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ToImages(BaseModel):
    """
    to_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FillForm(BaseModel):
    """
    fill_form
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFormFields(BaseModel):
    """
    get_form_fields
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfDocument(BaseModel):
    """
    pdf_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfMargin(BaseModel):
    """
    pdf_margin
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfOptions(BaseModel):
    """
    pdf_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfTemplateData(BaseModel):
    """
    pdf_template_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfPageText(BaseModel):
    """
    pdf_page_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfExtractedImage(BaseModel):
    """
    pdf_extracted_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfPermissions(BaseModel):
    """
    pdf_permissions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfFormFields(BaseModel):
    """
    pdf_form_fields
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfFormFieldValue(BaseModel):
    """
    pdf_form_field_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PdfFormField(BaseModel):
    """
    pdf_form_field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


