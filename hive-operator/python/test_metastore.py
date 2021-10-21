#!/usr/bin/env python3
"""
Test if a database with one table in the Hive metastore can be created.

Requirements: pip3 install -r requirements.txt
"""

from hive_metastore_client import HiveMetastoreClient
from hive_metastore_client.builders import (
    DatabaseBuilder,
    ColumnBuilder,
    SerDeInfoBuilder,
    StorageDescriptorBuilder,
    TableBuilder,
)

from thrift_files.libraries.thrift_hive_metastore_client.ttypes import (
    FieldSchema,
)
import argparse


def parse_args():
    parser = argparse.ArgumentParser(description="Test hive metastore.")
    parser.add_argument("-a", "--address", help="Metastore host address", default="127.0.0.1")
    parser.add_argument("-p", "--port", help="Metastore server port", default="9083")
    parser.add_argument("-d", "--database", help="Test DB name", default="test_metastore")
    return parser.parse_args()

def table(db_name, table_name):
    columns = [
        ColumnBuilder("id", "string", "col comment").build()
    ]

    serde_info = SerDeInfoBuilder(
        serialization_lib="org.apache.hadoop.hive.ql.io.parquet.serde.ParquetHiveSerDe"
    ).build()

    storage_descriptor = StorageDescriptorBuilder(
        columns=columns,
        location=f"/location_{db_name}_{table_name}",
        input_format="org.apache.hadoop.hive.ql.io.parquet.MapredParquetInputFormat",
        output_format="org.apache.hadoop.hive.ql.io.parquet.MapredParquetOutputFormat",
        serde_info=serde_info,
    ).build()

    table = TableBuilder(
        db_name=db_name,
        table_name=table_name,
        storage_descriptor=storage_descriptor,
    ).build()

    return table

def main():
    table_name="one_column_table"
    args = parse_args()

    # Creating database object using builder
    database = DatabaseBuilder(args.database).build()

    with HiveMetastoreClient(args.address, args.port) as hive_client:
        hive_client.create_database_if_not_exists(database)
        hive_client.create_table(table(args.database, table_name))
        #print(hive_client.get_all_tables(db_name=args.database))
        schema = hive_client.get_schema(db_name=args.database, table_name=table_name)
        assert schema == [FieldSchema(name='id', type='string', comment='col comment')]


if __name__ == '__main__':
    main()
