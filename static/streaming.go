package static

var Streaming = `
import argparse
import hashlib
from datetime import datetime
import os
import sys

from pyflink.common import Row, Types
from pyflink.datastream import StreamExecutionEnvironment
from pyflink.datastream.functions import MapFunction
from pyflink.table import StreamTableEnvironment

def configure_flink_environment(t_env):
    config = t_env.get_config()
    config.set_local_timezone("UTC")
    
    conf = config.get_configuration()
    conf.set_string("table.exec.sink.upsert-materialize", "NONE")
    conf.set_string("state.backend.type", "rocksdb")
    conf.set_string("state.checkpoints.dir", "file:///opt/flink/checkpoint")
    conf.set_string("state.savepoints.dir", "file:///opt/flink/savepoint")
    conf.set_string("execution.checkpointing.interval", "60s")
    conf.set_string("execution.checkpointing.min-pause", "1s")
    conf.set_string("execution.checkpointing.externalized-checkpoint-retention", "RETAIN_ON_CANCELLATION")
    conf.set_string("execution.checkpointing.timeout", "30 min")
    conf.set_string("execution.checkpointing.max-concurrent-checkpoints", "1")
    conf.set_string("state.backend.incremental", "true")
    conf.set_string("restart-strategy.fixed-delay.delay", "10 s")
    conf.set_string("execution.checkpointing.tolerable-failed-checkpoints", "2147483647")
    conf.set_string("sql-client.execution.result-mode", "tableau")
    conf.set_string("table.exec.sink.not-null-enforcer", "ERROR")

def create_paimon_catalog(t_env):
    t_env.execute_sql(f"""
    CREATE CATALOG paimon WITH (
        'type' = 'paimon',
        'warehouse' = 'oss://network-testnet/warehouse',
        'fs.oss.endpoint' = 'network-testnet.chainbasehq.com',
        'fs.oss.accessKeyId' = '{os.environ.get('OSS_ACCESS_KEY_ID')}',
        'fs.oss.accessKeySecret' = '{os.environ.get('OSS_ACCESS_KEY_SECRET')}',
        'table-default.merge-engine' = 'deduplicate',
        'table-default.changelog-producer' = 'input',
        'table-default.metastore.partitioned-table' = 'false',
        'table-default.lookup.cache-file-retention' = '1 h',
        'table-default.lookup.cache-max-memory-size' = '256 mb',
        'table-default.lookup.cache-max-disk-size' = '10 gb',
        'table-default.log.scan.remove-normalize' = 'true',
        'table-default.changelog-producer.row-deduplicate' = 'false',
        'table-default.consumer.expiration-time' = '24 h',
        'table-default.streaming-read-mode' = 'file',
        'table-default.orc.bloom.filter.fpp' = '0.00001',
        'table-default.scan.plan-sort-partition' = 'true',
        'table-default.snapshot.expire.limit' = '10000',
        'table-default.snapshot.num-retained.max' = '2000'
    )
    """)
    t_env.use_catalog("paimon")

def perform_proof_of_work(input_hash, difficulty):
    target = 2 ** (256 - difficulty)
    nonce = 0
    while True:
        data = input_hash + nonce.to_bytes(4, 'big')
        hash_result = hashlib.sha256(data).digest()
        if int.from_bytes(hash_result, 'big') < target:
            return hash_result.hex()
        nonce += 1

class ProcessFunction(MapFunction):
    def __init__(self, chain, difficulty, task_index):
        self.chain = chain
        self.difficulty = difficulty
        self.task_index = task_index

    def map(self, value):
        is_insert, row = value
        if is_insert:
            block_number, block_hash = row
            pow_result = perform_proof_of_work(block_hash.encode(), self.difficulty)
            insert_at = datetime.now()
            return Row(self.chain, block_number, block_hash, pow_result, insert_at, self.difficulty, self.task_index)
        else:
            return None

def create_postgresql_sink(t_env):
    t_env.execute_sql("""
        CREATE TEMPORARY TABLE pg_sink (
            chain STRING,
            block_number BIGINT,
            block_hash STRING,
            pow_result STRING,
            insert_at TIMESTAMP(3),
            difficulty INT,
            task_index BIGINT,
            PRIMARY KEY (chain, block_number) NOT ENFORCED
        ) WITH (
            'connector' = 'jdbc',
            'url' = 'jdbc:postgresql://postgres:5432/manuscript_node',
            'table-name' = 'pow_results',
            'username' = 'postgres',
            'password' = 'postgres',
            'sink.max-retries' = '3',
            'sink.buffer-flush.max-rows' = '1',
            'sink.buffer-flush.interval' = '0s'
        )
    """)

def main(chain, start_at, end_at, difficulty, task_index):
    # Set up execution environment
    env = StreamExecutionEnvironment.get_execution_environment()
    t_env = StreamTableEnvironment.create(env)

    env.set_parallelism(1) 

    # Configure Flink environment
    configure_flink_environment(t_env)

    # Create Paimon Catalog
    create_paimon_catalog(t_env)

    # Build SQL query using parameters
    sql_query = f"SELECT block_number, hash FROM {chain}.blocks WHERE block_number >= {start_at} and block_number <= {end_at}"

    # Execute SQL query using Table API
    result = t_env.sql_query(sql_query)

    # Convert result to data stream
    ds = t_env.to_retract_stream(
        result,
        Types.ROW([Types.LONG(), Types.STRING()])
    )

    # Apply map function
    processed_stream = ds.map(
        ProcessFunction(chain, difficulty, task_index),
        output_type=Types.ROW([Types.STRING(), Types.LONG(), Types.STRING(), Types.STRING(), Types.SQL_TIMESTAMP(), Types.INT(), Types.LONG()])
    )

    # Create PostgreSQL sink
    create_postgresql_sink(t_env)

    # Convert processed stream to table and write to PostgreSQL using upsert mode
    t_env.from_data_stream(processed_stream).execute_insert("pg_sink")

    print("Stream processing task completed successfully.")

if __name__ == "__main__":
    # Check for required environment variables
    if 'OSS_ACCESS_KEY_ID' not in os.environ:
        print("Error: OSS_ACCESS_KEY_ID environment variable is not set.")
        sys.exit(1)
    if 'OSS_ACCESS_KEY_SECRET' not in os.environ:
        print("Error: OSS_ACCESS_KEY_SECRET environment variable is not set.")
        sys.exit(1)

    parser = argparse.ArgumentParser(description='PyFlink Proof of Work Stream Processing')
    parser.add_argument('--chain', type=str, required=True, help='Chain name')
    parser.add_argument('--start_at', type=int, required=True, help='Start block number')
    parser.add_argument('--end_at', type=int, required=True, help='End block number')
    parser.add_argument('--difficulty', type=int, required=True, help='Proof of Work difficulty', default=22)
    parser.add_argument('--task_index', type=int, required=True, help='Task index')
    args = parser.parse_args()

    main(args.chain, args.start_at, args.end_at, args.difficulty, args.task_index)
`
