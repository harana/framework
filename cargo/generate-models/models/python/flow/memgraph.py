# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateNode(BaseModel):
    """
    create_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRelationship(BaseModel):
    """
    create_relationship
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FindNodes(BaseModel):
    """
    find_nodes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FindNode(BaseModel):
    """
    find_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateNode(BaseModel):
    """
    update_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNode(BaseModel):
    """
    delete_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRelationship(BaseModel):
    """
    delete_relationship
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExecuteQuery(BaseModel):
    """
    execute_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ShortestPath(BaseModel):
    """
    shortest_path
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FindPaths(BaseModel):
    """
    find_paths
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetNeighbors(BaseModel):
    """
    get_neighbors
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDegree(BaseModel):
    """
    get_degree
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateIndex(BaseModel):
    """
    create_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DropIndex(BaseModel):
    """
    drop_index
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetStats(BaseModel):
    """
    get_stats
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RunAlgorithm(BaseModel):
    """
    run_algorithm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MatchPattern(BaseModel):
    """
    match_pattern
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphNode(BaseModel):
    """
    graph_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QuerySummary(BaseModel):
    """
    query_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphPathElement(BaseModel):
    """
    graph_path_element
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphPath(BaseModel):
    """
    graph_path
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GraphStats(BaseModel):
    """
    graph_stats
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AlgorithmResult(BaseModel):
    """
    algorithm_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


