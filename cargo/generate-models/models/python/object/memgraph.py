# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MemgraphNode(BaseModel):
    """
    memgraph_node
    
    ID fields: node_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    labels: Optional[str] = None
    node_id: str
    properties: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class MemgraphRelationship(BaseModel):
    """
    memgraph_relationship
    
    ID fields: relationship_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    from_node_id: str
    properties: Optional[str] = None
    relationship_id: str
    to_node_id: str
    type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MemgraphIndex(BaseModel):
    """
    memgraph_index
    
    ID fields: label, property
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    label: str
    property: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MemgraphQueryLog(BaseModel):
    """
    memgraph_query_log
    
    ID fields: executed_at
    """

    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    nodes_affected: int = Field(default=0)
    query: str
    relationships_affected: int = Field(default=0)
    status: str = Field(default="success")
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


