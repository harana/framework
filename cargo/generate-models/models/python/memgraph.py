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


class GraphNode(BaseModel):
    """
    graph_node
    
    ID fields: id
    """

    node_id: str
    labels: List[str]
    properties: str
    class Config:
        from_attributes = True
        populate_by_name = True


class QuerySummary(BaseModel):
    """
    query_summary
    
    ID fields: id
    """

    nodes_created: int
    nodes_deleted: int
    relationships_created: int
    relationships_deleted: int
    properties_set: int
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphPathElement(BaseModel):
    """
    graph_path_element
    
    ID fields: id
    """

    node_id: str
    relationship_id: str
    relationship_type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphPath(BaseModel):
    """
    graph_path
    
    ID fields: id
    """

    elements: List[str]
    length: int
    class Config:
        from_attributes = True
        populate_by_name = True


class GraphStats(BaseModel):
    """
    graph_stats
    
    ID fields: id
    """

    labels: str
    relationship_types: str
    indexes: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class AlgorithmResult(BaseModel):
    """
    algorithm_result
    
    ID fields: id
    """

    node_id: str
    score: float
    class Config:
        from_attributes = True
        populate_by_name = True


