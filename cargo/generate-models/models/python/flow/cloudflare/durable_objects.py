# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class IdFromName(BaseModel):
    """
    id_from_name
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IdFromString(BaseModel):
    """
    id_from_string
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NewUniqueId(BaseModel):
    """
    new_unique_id
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetStub(BaseModel):
    """
    get_stub
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Fetch(BaseModel):
    """
    fetch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageGet(BaseModel):
    """
    storage_get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageGetMultiple(BaseModel):
    """
    storage_get_multiple
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StoragePut(BaseModel):
    """
    storage_put
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StoragePutMultiple(BaseModel):
    """
    storage_put_multiple
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageDelete(BaseModel):
    """
    storage_delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageDeleteMultiple(BaseModel):
    """
    storage_delete_multiple
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageDeleteAll(BaseModel):
    """
    storage_delete_all
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageList(BaseModel):
    """
    storage_list
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StorageTransaction(BaseModel):
    """
    storage_transaction
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlExec(BaseModel):
    """
    sql_exec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetAlarm(BaseModel):
    """
    set_alarm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAlarm(BaseModel):
    """
    get_alarm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteAlarm(BaseModel):
    """
    delete_alarm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AcceptWebsocket(BaseModel):
    """
    accept_websocket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetWebsockets(BaseModel):
    """
    get_websockets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetWebsocketAutoResponse(BaseModel):
    """
    set_websocket_auto_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HibernateWebsocket(BaseModel):
    """
    hibernate_websocket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


