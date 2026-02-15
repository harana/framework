# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Environment(BaseModel):
    """
    environment
    
    ID fields: id
    """

    blob: Optional[str] = None
    cache: Optional[str] = None
    deployment: Optional[str] = None
    environment: str = Field(default="development")
    http: Optional[str] = None
    jwt: Optional[str] = None
    log: Optional[str] = None
    oauth: Optional[str] = None
    passkey: Optional[str] = None
    session: str
    storage: Optional[str] = None
    tls: Optional[str] = None
    tracing: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class HttpOptions(BaseModel):
    """
    http_options
    
    ID fields: id
    """

    allowed_origins: Optional[List[str]] = None
    enable_cors: bool = Field(default=True)
    graceful_shutdown_seconds: int = Field(default=30)
    host: str = Field(default=""127.0.0.1"")
    max_concurrent_requests: Optional[int] = None
    port: int = Field(default=3000)
    read_timeout_seconds: int = Field(default=15)
    write_timeout_seconds: int = Field(default=15)
    class Config:
        from_attributes = True
        populate_by_name = True


class TlsOptions(BaseModel):
    """
    tls_options
    
    ID fields: id
    """

    alpn_protocols: Optional[List[str]] = None
    cert_path: Optional[str] = None
    enabled: bool = Field(default=False)
    key_path: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class LogOptions(BaseModel):
    """
    log_options
    
    ID fields: id
    """

    level: str = Field(default="info")
    format: str = Field(default="text")
    metrics_enabled: bool = Field(default=True)
    metrics_listen: Optional[str] = None
    sentry_dsn: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class TracingOptions(BaseModel):
    """
    tracing_options
    
    ID fields: id
    """

    enabled: bool = Field(default=False)
    provider: str = Field(default="none")
    endpoint: Optional[str] = None
    sample_rate: float = Field(default="1.0")
    class Config:
        from_attributes = True
        populate_by_name = True


class StorageOptions(BaseModel):
    """
    storage_options
    
    ID fields: id
    """

    backend: str = Field(default="durable_object")
    mongodb_url: Optional[str] = None
    mongodb_database: Optional[str] = None
    d1_binding: Optional[str] = None
    durable_object: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheOptions(BaseModel):
    """
    cache_options
    
    ID fields: id
    """

    backend: str = Field(default="memory")
    kv_binding: Optional[str] = None
    mongodb_url: Optional[str] = None
    mongodb_database: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class BlobOptions(BaseModel):
    """
    blob_options
    
    ID fields: id
    """

    backend: str = Field(default="file")
    file_base_path: Optional[str] = None
    r2_binding: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class JwtOptions(BaseModel):
    """
    jwt_options
    
    ID fields: id
    """

    secret: str
    issuer: str = Field(default=""harana"")
    audience: str = Field(default=""harana-app"")
    access_token_expiry: int = Field(default=3600)
    refresh_token_expiry: int = Field(default=604800)
    class Config:
        from_attributes = True
        populate_by_name = True


class OauthOptions(BaseModel):
    """
    oauth_options
    
    ID fields: id
    """

    providers: List[str]
    auth_code_expiry: int = Field(default=600)
    issuer: Optional[str] = None
    discovery_cache_ttl: int = Field(default=3600)
    state_ttl: int = Field(default=600)
    class Config:
        from_attributes = True
        populate_by_name = True


class OauthProvider(BaseModel):
    """
    oauth_provider
    
    ID fields: id
    """

    name: str
    kind: str = Field(default="generic")
    client_id: str
    client_secret: str
    discovery_url: Optional[str] = None
    auth_url: Optional[str] = None
    token_url: Optional[str] = None
    userinfo_url: Optional[str] = None
    scopes: Optional[List[str]] = None
    cloudflare_team_domain: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyOptions(BaseModel):
    """
    passkey_options
    
    ID fields: id
    """

    relying_party_id: str = Field(default=""localhost"")
    relying_party_name: str = Field(default=""Harana"")
    relying_party_origin: str = Field(default=""http://localhost:3000"")
    timeout: int = Field(default=60000)
    class Config:
        from_attributes = True
        populate_by_name = True


class SessionOptions(BaseModel):
    """
    session_options
    
    ID fields: id
    """

    cookie_name: str = Field(default=""harana_session"")
    ttl: int = Field(default=86400)
    secure: bool = Field(default=False)
    domain: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DurableObjectOptions(BaseModel):
    """
    durable_object_options
    
    ID fields: id
    """

    binding: str
    namespace: str
    alarm_interval_seconds: Optional[int] = None
    max_stored_bytes: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeploymentOptions(BaseModel):
    """
    deployment_options
    
    ID fields: id
    """

    target: str = Field(default="standalone")
    cloudflare_worker: Optional[str] = None
    standalone: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareWorkerOptions(BaseModel):
    """
    cloudflare_worker_options
    
    ID fields: id
    """

    worker_name: str
    compatibility_date: str
    account_id: Optional[str] = None
    route: Optional[str] = None
    zone_id: Optional[str] = None
    durable_object_binding: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class StandaloneOptions(BaseModel):
    """
    standalone_options
    
    ID fields: id
    """

    workers: int = Field(default=4)
    max_blocking_threads: Optional[int] = None
    enable_http2: bool = Field(default=True)
    pid_file: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


