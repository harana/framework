# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateFunction(BaseModel):
    """
    create_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFunction(BaseModel):
    """
    delete_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFunction(BaseModel):
    """
    get_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListFunctions(BaseModel):
    """
    list_functions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateFunctionCode(BaseModel):
    """
    update_function_code
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateFunctionConfiguration(BaseModel):
    """
    update_function_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Invoke(BaseModel):
    """
    invoke
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InvokeAsync(BaseModel):
    """
    invoke_async
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PublishVersion(BaseModel):
    """
    publish_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListVersionsByFunction(BaseModel):
    """
    list_versions_by_function
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateAlias(BaseModel):
    """
    create_alias
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteAlias(BaseModel):
    """
    delete_alias
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAlias(BaseModel):
    """
    get_alias
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListAliases(BaseModel):
    """
    list_aliases
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateAlias(BaseModel):
    """
    update_alias
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateEventSourceMapping(BaseModel):
    """
    create_event_source_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteEventSourceMapping(BaseModel):
    """
    delete_event_source_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetEventSourceMapping(BaseModel):
    """
    get_event_source_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListEventSourceMappings(BaseModel):
    """
    list_event_source_mappings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateEventSourceMapping(BaseModel):
    """
    update_event_source_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddPermission(BaseModel):
    """
    add_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemovePermission(BaseModel):
    """
    remove_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPolicy(BaseModel):
    """
    get_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateFunctionUrlConfig(BaseModel):
    """
    create_function_url_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFunctionUrlConfig(BaseModel):
    """
    delete_function_url_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFunctionUrlConfig(BaseModel):
    """
    get_function_url_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateFunctionUrlConfig(BaseModel):
    """
    update_function_url_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListFunctionUrlConfigs(BaseModel):
    """
    list_function_url_configs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutFunctionConcurrency(BaseModel):
    """
    put_function_concurrency
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFunctionConcurrency(BaseModel):
    """
    delete_function_concurrency
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetFunctionConcurrency(BaseModel):
    """
    get_function_concurrency
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutProvisionedConcurrencyConfig(BaseModel):
    """
    put_provisioned_concurrency_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteProvisionedConcurrencyConfig(BaseModel):
    """
    delete_provisioned_concurrency_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetProvisionedConcurrencyConfig(BaseModel):
    """
    get_provisioned_concurrency_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListProvisionedConcurrencyConfigs(BaseModel):
    """
    list_provisioned_concurrency_configs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PublishLayerVersion(BaseModel):
    """
    publish_layer_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLayerVersion(BaseModel):
    """
    delete_layer_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetLayerVersion(BaseModel):
    """
    get_layer_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListLayers(BaseModel):
    """
    list_layers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListLayerVersions(BaseModel):
    """
    list_layer_versions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddLayerVersionPermission(BaseModel):
    """
    add_layer_version_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveLayerVersionPermission(BaseModel):
    """
    remove_layer_version_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetLayerVersionPolicy(BaseModel):
    """
    get_layer_version_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TagResource(BaseModel):
    """
    tag_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UntagResource(BaseModel):
    """
    untag_resource
    
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


