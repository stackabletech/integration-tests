#!/usr/bin/env python3
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


def table(db_name, table_name):
    columns = [
        ColumnBuilder("id", "string", "col comment").build()
    ]

    serde_info = SerDeInfoBuilder(
        serialization_lib="org.apache.hadoop.hive.ql.io.parquet.serde.ParquetHiveSerDe"
    ).build()

    storage_descriptor = StorageDescriptorBuilder(
        columns=columns,
        location=f"/stackable/warehouse/location_{db_name}_{table_name}",
        input_format="org.apache.hadoop.hive.ql.io.parquet.MapredParquetInputFormat",
        output_format="org.apache.hadoop.hive.ql.io.parquet.MapredParquetOutputFormat",
        serde_info=serde_info,
    ).build()

    test_table = TableBuilder(
        db_name=db_name,
        table_name=table_name,
        storage_descriptor=storage_descriptor,
    ).build()

    return test_table


if __name__ == '__main__':
    all_args = argparse.ArgumentParser(description="Test hive metastore.")
    all_args.add_argument("-p", "--port", help="Metastore server port", default="9083")
    all_args.add_argument("-d", "--database", help="Test DB name", default="test_metastore")
    all_args.add_argument("-n", "--namespace", help="The namespace to run in", required=True)
    args = vars(all_args.parse_args())

    namespace = args["namespace"]
    database_name = args["database"]
    port = args["port"]
    test_table_name = "one_column_table"
    host = 'test-hive-derby-metastore-default-0.test-hive-derby-metastore-default.' + namespace + '.svc.cluster.local'
    # Creating database object using builder
    database = DatabaseBuilder(database_name).build()

    with HiveMetastoreClient(host, port) as hive_client:
        hive_client.create_database_if_not_exists(database)
        hive_client.create_table(table(database_name, test_table_name))
        schema = hive_client.get_schema(db_name=database_name, table_name=test_table_name)
        expected = [FieldSchema(name='id', type='string', comment='col comment')]
        if schema != expected:
            print("Error: Received schema " + str(schema) + " - expected schema: " + expected)
            exit(-1)
        else:
            print("Test successful!")
            exit(0)
