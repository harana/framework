# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SqlConnection(BaseModel):
    """
    sql_connection
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    database: str
    driver: str = Field(default="postgres")
    host: str
    is_active: bool = Field(default=True)
    max_connections: int = Field(default=10)
    password: Optional[str] = None
    port: int
    ssl_mode: str = Field(default="prefer")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    username: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlTable(BaseModel):
    """
    sql_table
    
    ID fields: connection_id, schema_name, table_name
    """

    connection_id: str = Field(description="Reference: sql_connection.id")
    row_count: Optional[int] = None
    schema_name: str = Field(default="public")
    table_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlColumn(BaseModel):
    """
    sql_column
    
    ID fields: table_id, column_name
    """

    column_name: str
    data_type: str
    default_value: Optional[str] = None
    is_nullable: bool = Field(default=True)
    is_primary_key: bool = Field(default=False)
    max_length: Optional[int] = None
    sort_order: int = Field(default=0)
    table_id: str = Field(description="Reference: sql_table.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlIndex(BaseModel):
    """
    sql_index
    
    ID fields: table_id, index_name
    """

    columns: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    index_name: str
    is_unique: bool = Field(default=False)
    table_id: str = Field(description="Reference: sql_table.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlQueryLog(BaseModel):
    """
    sql_query_log
    
    ID fields: connection_id, executed_at
    """

    affected_rows: Optional[int] = None
    connection_id: str = Field(description="Reference: sql_connection.id")
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    query_hash: Optional[str] = None
    query_text: Optional[str] = None
    query_type: str = Field(default="select")
    status: str = Field(default="success")
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlTransaction(BaseModel):
    """
    sql_transaction
    
    ID fields: transaction_id
    """

    completed_at: Optional[datetime] = None
    connection_id: str = Field(description="Reference: sql_connection.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    isolation_level: str = Field(default="read_committed")
    status: str = Field(default="active")
    transaction_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlQuery(BaseModel):
    """
    sql_query
    
    ID fields: id
    """

    query: str
    parameters: List[str]
    database: str
    affected_rows: int
    rows: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlRow(BaseModel):
    """
    sql_row
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlBatchQuery(BaseModel):
    """
    sql_batch_query
    
    ID fields: id
    """

    query: str
    parameters: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlBatchResult(BaseModel):
    """
    sql_batch_result
    
    ID fields: id
    """

    success: bool
    affected_rows: int
    error: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlOutputParameters(BaseModel):
    """
    sql_output_parameters
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlResultSet(BaseModel):
    """
    sql_result_set
    
    ID fields: id
    """

    columns: List[str]
    rows: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlColumnInfo(BaseModel):
    """
    sql_column_info
    
    ID fields: id
    """

    name: str
    data_type: str
    nullable: bool
    default_value: str
    max_length: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlIndexInfo(BaseModel):
    """
    sql_index_info
    
    ID fields: id
    """

    name: str
    columns: List[str]
    unique: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class SqlColumnDefinition(BaseModel):
    """
    sql_column_definition
    
    ID fields: id
    """

    name: str
    data_type: str
    nullable: bool
    default_value: str
    primary_key: bool
    class Config:
        from_attributes = True
        populate_by_name = True


