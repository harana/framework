# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GenerateRegistrationOptions(BaseModel):
    """
    generate_registration_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyRegistration(BaseModel):
    """
    verify_registration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateAuthenticationOptions(BaseModel):
    """
    generate_authentication_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifyAuthentication(BaseModel):
    """
    verify_authentication
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StoreCredential(BaseModel):
    """
    store_credential
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetUserPasskeys(BaseModel):
    """
    get_user_passkeys
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPasskey(BaseModel):
    """
    get_passkey
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateCounter(BaseModel):
    """
    update_counter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeletePasskey(BaseModel):
    """
    delete_passkey
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdatePasskeyName(BaseModel):
    """
    update_passkey_name
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckSupport(BaseModel):
    """
    check_support
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Passkey(BaseModel):
    """
    passkey
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorSelection(BaseModel):
    """
    passkey_authenticator_selection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyCredentialDescriptor(BaseModel):
    """
    passkey_credential_descriptor
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRegistrationOptions(BaseModel):
    """
    passkey_registration_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRelyingParty(BaseModel):
    """
    passkey_relying_party
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyUser(BaseModel):
    """
    passkey_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyCredParam(BaseModel):
    """
    passkey_cred_param
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRegistrationResponse(BaseModel):
    """
    passkey_registration_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorAttestationResponse(BaseModel):
    """
    passkey_authenticator_attestation_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticationOptions(BaseModel):
    """
    passkey_authentication_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticationResponse(BaseModel):
    """
    passkey_authentication_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorAssertionResponse(BaseModel):
    """
    passkey_authenticator_assertion_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyInfo(BaseModel):
    """
    passkey_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


