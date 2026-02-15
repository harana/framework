# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployKubernetes(BaseModel):
    """
    deploy_kubernetes
    
    ID fields: id
    """

    context: str
    enabled: bool = Field(default=True)
    health_check: Optional[str] = None
    helm: Optional[str] = None
    image: str
    kubectl: Optional[str] = None
    kustomize: Optional[str] = None
    manifests: List[str]
    namespace: str = Field(default="default")
    post_deploy: List[str]
    pre_deploy: List[str]
    replicas: int = Field(default=3)
    rolling_update: Optional[str] = None
    strategy: str = Field(default="rolling_update")
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployKubernetesRollingUpdate(BaseModel):
    """
    deploy_kubernetes_rolling_update
    
    ID fields: id
    """

    max_surge: int = Field(default=1)
    max_unavailable: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployKubernetesHelm(BaseModel):
    """
    deploy_kubernetes_helm
    
    ID fields: id
    """

    chart: str
    enabled: bool = Field(default=False)
    release_name: str
    set: List[str]
    timeout: int = Field(default=600)
    values_file: Optional[str] = None
    wait: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployKubernetesKubectl(BaseModel):
    """
    deploy_kubernetes_kubectl
    
    ID fields: id
    """

    apply_options: List[str]
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployKubernetesKustomize(BaseModel):
    """
    deploy_kubernetes_kustomize
    
    ID fields: id
    """

    dir: str
    enabled: bool = Field(default=False)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployKubernetesHealthCheck(BaseModel):
    """
    deploy_kubernetes_health_check
    
    ID fields: id
    """

    enabled: bool = Field(default=True)
    endpoint: str
    interval: int = Field(default=10)
    timeout: int = Field(default=30)
    class Config:
        from_attributes = True
        populate_by_name = True


