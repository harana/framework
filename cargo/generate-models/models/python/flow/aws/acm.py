# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RequestCertificate(BaseModel):
    """
    request_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImportCertificate(BaseModel):
    """
    import_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteCertificate(BaseModel):
    """
    delete_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeCertificate(BaseModel):
    """
    describe_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCertificates(BaseModel):
    """
    list_certificates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCertificate(BaseModel):
    """
    get_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddTags(BaseModel):
    """
    add_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveTags(BaseModel):
    """
    remove_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTags(BaseModel):
    """
    list_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RenewCertificate(BaseModel):
    """
    renew_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExportCertificate(BaseModel):
    """
    export_certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResendValidationEmail(BaseModel):
    """
    resend_validation_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateCertificateOptions(BaseModel):
    """
    update_certificate_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAccountConfiguration(BaseModel):
    """
    get_account_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutAccountConfiguration(BaseModel):
    """
    put_account_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


