# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PasskeyCredential(BaseModel):
    """
    passkey_credential
    
    ID fields: credential_id
    """

    aaguid: Optional[str] = None
    backed_up: bool = Field(default=False)
    counter: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    credential_id: str
    device_type: Optional[str] = None
    friendly_name: Optional[str] = None
    is_active: bool = Field(default=True)
    last_used_at: Optional[datetime] = None
    public_key_fingerprint: Optional[str] = None
    transports: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyChallenge(BaseModel):
    """
    passkey_challenge
    
    ID fields: challenge
    """

    challenge: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: datetime
    rp_id: str
    type: str = Field(default="registration")
    used: bool = Field(default=False)
    user_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthLog(BaseModel):
    """
    passkey_auth_log
    
    ID fields: credential_id, authenticated_at
    """

    authenticated_at: datetime = Field(default_factory=datetime.utcnow)
    credential_id: str
    ip_address: Optional[str] = None
    success: bool = Field(default=True)
    user_agent: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class Passkey(BaseModel):
    """
    passkey
    
    ID fields: id
    """

    passkey_id: str
    credential_id: str
    user_id: str
    credential_public_key: str
    counter: int
    device_type: str
    backed_up: bool
    friendly_name: str
    created_at: datetime
    last_used_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorSelection(BaseModel):
    """
    passkey_authenticator_selection
    
    ID fields: id
    """

    authenticator_attachment: str
    resident_key: str
    require_resident_key: bool
    user_verification: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyCredentialDescriptor(BaseModel):
    """
    passkey_credential_descriptor
    
    ID fields: id
    """

    id: str
    type: str
    transports: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRegistrationOptions(BaseModel):
    """
    passkey_registration_options
    
    ID fields: id
    """

    rp: str
    user: str
    challenge: str
    pub_key_cred_params: List[str]
    timeout: int
    attestation: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRelyingParty(BaseModel):
    """
    passkey_relying_party
    
    ID fields: id
    """

    id: str
    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyUser(BaseModel):
    """
    passkey_user
    
    ID fields: id
    """

    id: str
    name: str
    display_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyCredParam(BaseModel):
    """
    passkey_cred_param
    
    ID fields: id
    """

    type: str
    alg: int
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyRegistrationResponse(BaseModel):
    """
    passkey_registration_response
    
    ID fields: id
    """

    id: str
    raw_id: str
    response: str
    type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorAttestationResponse(BaseModel):
    """
    passkey_authenticator_attestation_response
    
    ID fields: id
    """

    client_data_json: str
    attestation_object: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticationOptions(BaseModel):
    """
    passkey_authentication_options
    
    ID fields: id
    """

    challenge: str
    timeout: int
    rp_id: str
    allow_credentials: List[str]
    user_verification: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticationResponse(BaseModel):
    """
    passkey_authentication_response
    
    ID fields: id
    """

    id: str
    raw_id: str
    response: str
    type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthenticatorAssertionResponse(BaseModel):
    """
    passkey_authenticator_assertion_response
    
    ID fields: id
    """

    client_data_json: str
    authenticator_data: str
    signature: str
    user_handle: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyInfo(BaseModel):
    """
    passkey_info
    
    ID fields: id
    """

    passkey_id: str
    credential_id: str
    friendly_name: str
    device_type: str
    created_at: datetime
    last_used_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


