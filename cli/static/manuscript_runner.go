package static

var ManuscriptRunner = `
import yaml
import os
import argparse
import threading
from pyflink.datastream import StreamExecutionEnvironment
from pyflink.table import StreamTableEnvironment, EnvironmentSettings

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

class ETLProcessor:
    def __init__(self, config_path):
        self.config = self.load_config(config_path)
        self.validate_config()
        self.env = StreamExecutionEnvironment.get_execution_environment()
        self.settings = EnvironmentSettings.new_instance().in_streaming_mode().build()
        self.t_env = StreamTableEnvironment.create(self.env, self.settings)
        
        # Configure Flink environment
        configure_flink_environment(self.t_env)
        
        # Create Paimon catalog
        create_paimon_catalog(self.t_env)

    def load_config(self, config_path):
        with open(config_path, 'r') as file:
            return yaml.safe_load(file)

    def validate_config(self):
        required_fields = ['name', 'specVersion', 'parallelism']
        for field in required_fields:
            if field not in self.config:
                raise ValueError(f"Missing required field: {field}")

        if 'sources' in self.config:
            for source in self.config['sources']:
                required_source_fields = ['name', 'type', 'dataset']
                for field in required_source_fields:
                    if field not in source:
                        raise ValueError(f"Missing required field in source: {field}")

        if 'transforms' in self.config:
            for transform in self.config['transforms']:
                required_transform_fields = ['name', 'sql']
                for field in required_transform_fields:
                    if field not in transform:
                        raise ValueError(f"Missing required field in transform: {field}")

        if 'sinks' in self.config:
            for sink in self.config['sinks']:
                required_sink_fields = ['name', 'type', 'from']
                for field in required_sink_fields:
                    if field not in sink:
                        raise ValueError(f"Missing required field in sink: {field}")

    def register_udfs(self):
        udf_jar = 'oss://network-testnet/flink-udf/flink-udf-1.0-SNAPSHOT.jar'
        self.t_env.execute_sql(f"""
            CREATE TEMPORARY SYSTEM FUNCTION Decode_Event AS 'com.chainbase.udf.DecodeEvent' 
            LANGUAGE JAVA USING JAR '{udf_jar}';
        """)
        self.t_env.execute_sql(f"""
            CREATE TEMPORARY SYSTEM FUNCTION Decode_Function AS 'com.chainbase.udf.DecodeFunction' 
            LANGUAGE JAVA USING JAR '{udf_jar}';
        """)
        self.t_env.execute_sql(f"""
            CREATE TEMPORARY SYSTEM FUNCTION Eth_Call AS 'com.chainbase.udf.EthCallRequest' 
            LANGUAGE JAVA USING JAR '{udf_jar}';
        """)

    def create_sources(self):
        self.t_env.use_catalog("default_catalog")
        for source in self.config['sources']:
            if source['type'] == 'dataset':
                filter_clause = f"WHERE {source['filter']}" if 'filter' in source else ""
                self.t_env.execute_sql(f"""
                    CREATE TEMPORARY VIEW {source['name']} AS
                    SELECT * FROM paimon.{source['dataset']}
                    {filter_clause}
                """)

    def create_transforms(self):
        self.t_env.use_catalog("default_catalog")
        for transform in self.config['transforms']:
            self.t_env.create_temporary_view(
                transform['name'],
                self.t_env.sql_query(transform['sql'])
            )

    def create_sinks(self):
        self.t_env.use_catalog("default_catalog")
        for sink in self.config['sinks']:
            if sink['type'] == 'postgres':
                self.create_postgres_sink(sink)
            elif sink['type'] == 'starrocks':
                self.create_starrocks_sink(sink)
            elif sink['type'] == 'print':
                self.create_print_sink(sink)

    def get_schema_from_transform(self, transform_name):
        result = self.t_env.execute_sql(f"DESCRIBE {transform_name}")
        schema = []
        for row in result.collect():
            field_name = row[0]
            field_type = row[1]
            schema.append(f"{field_name} {field_type}")
        return ", ".join(schema)

    def create_postgres_sink(self, sink):
        schema = self.get_schema_from_transform(sink['from'])
        self.t_env.execute_sql(f"""
            CREATE TABLE {sink['name']} (
                {schema}
            ) WITH (
                'connector' = 'jdbc',
                'url' = 'jdbc:postgresql://localhost:5432/{sink["database"]}',
                'table-name' = '{sink["schema"]}.{sink["table"]}',
                'username' = '{sink["config"]["username"]}',
                'password' = '{sink["config"]["password"]}'
            )
        """)

    def create_starrocks_sink(self, sink):
        schema = self.get_schema_from_transform(sink['from'])
        self.t_env.execute_sql(f"""
            CREATE TABLE {sink['name']} (
                {schema}
            ) WITH (
                'connector' = 'starrocks',
                'jdbc-url' = 'jdbc:mysql://localhost:9030/{sink["database"]}',
                'load-url' = 'localhost:8030',
                'database-name' = '{sink["database"]}',
                'table-name' = '{sink["table"]}',
                'username' = '{sink["config"]["username"]}',
                'password' = '{sink["config"]["password"]}'
            )
        """)

    def create_print_sink(self, sink):
        schema = self.get_schema_from_transform(sink['from'])
        self.t_env.execute_sql(f"""
            CREATE TABLE {sink['name']} (
                {schema}
            ) WITH ('connector' = 'print', 'standard-error' = 'true')
        """)

    def execute(self):
        def execute_flink_job():
            try:
                self.register_udfs()
                self.create_sources()
                self.create_transforms()
                self.create_sinks()

                for sink in self.config['sinks']:
                    source_view = self.t_env.from_path(sink['from'])
                    source_view.execute_insert(sink['name']).wait()
                print("Flink job submitted.")
                job_client = self.env.execute_async(self.config['name'])
                print(f"Job started with Job ID: {job_client.get_job_id()}")

            except Exception as e:
                print(f"Execution failed: {e}")
            finally:
                print("Flink job finished.")

        execution_thread = threading.Thread(target=execute_flink_job)
        execution_thread.start()
        print("Flink job started.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='ETL Processor')
    parser.add_argument('--config', type=str, default='manuscript.yaml', help='Path to the configuration file')
    args = parser.parse_args()

    processor = ETLProcessor(args.config)
    processor.execute()
    print("Job completed successfully.")
`
