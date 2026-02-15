# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Execute(BaseModel):
    """
    execute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Select(BaseModel):
    """
    select
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Insert(BaseModel):
    """
    insert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Update(BaseModel):
    """
    update
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Batch(BaseModel):
    """
    batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BeginTransaction(BaseModel):
    """
    begin_transaction
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Commit(BaseModel):
    """
    commit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Rollback(BaseModel):
    """
    rollback
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CallProcedure(BaseModel):
    """
    call_procedure
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSchema(BaseModel):
    """
    get_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTables(BaseModel):
    """
    list_tables
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTable(BaseModel):
    """
    create_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DropTable(BaseModel):
    """
    drop_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TruncateTable(BaseModel):
    """
    truncate_table
    
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


class BulkInsert(BaseModel):
    """
    bulk_insert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PreparedStatement(BaseModel):
    """
    prepared_statement
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlQuery(BaseModel):
    """
    sql_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlRow(BaseModel):
    """
    sql_row
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlBatchQuery(BaseModel):
    """
    sql_batch_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlBatchResult(BaseModel):
    """
    sql_batch_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlOutputParameters(BaseModel):
    """
    sql_output_parameters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlResultSet(BaseModel):
    """
    sql_result_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlColumnInfo(BaseModel):
    """
    sql_column_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlIndexInfo(BaseModel):
    """
    sql_index_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqlColumnDefinition(BaseModel):
    """
    sql_column_definition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


