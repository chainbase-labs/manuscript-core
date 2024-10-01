package com.chainbase.manuscript;

import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.table.api.EnvironmentSettings;
import org.apache.flink.table.api.Table;
import org.apache.flink.table.api.TableColumn;
import org.apache.flink.table.api.TableResult;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.apache.flink.configuration.Configuration;
import org.yaml.snakeyaml.Yaml;

import java.io.FileInputStream;
import java.io.InputStream;
import java.sql.ResultSet;
import java.util.Map;
import java.util.List;
import java.util.stream.Collectors;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.Statement;
import java.util.ArrayList;

public class ETLProcessor {
    private Map<String, Object> config;
    private StreamExecutionEnvironment env;
    private StreamTableEnvironment tEnv;

    public ETLProcessor(String configPath) {
        this.config = loadConfig(configPath);
        validateConfig();
        this.env = StreamExecutionEnvironment.getExecutionEnvironment();
        this.env.setParallelism((Integer) config.get("parallelism"));
        EnvironmentSettings settings = EnvironmentSettings.newInstance().inStreamingMode().build();
        this.tEnv = StreamTableEnvironment.create(env, settings);

        configureFlink();
        createPaimonCatalog();
    }

    private Map<String, Object> loadConfig(String configPath) {
        try (InputStream input = new FileInputStream(configPath)) {
            Yaml yaml = new Yaml();
            return yaml.load(input);
        } catch (Exception e) {
            throw new RuntimeException("Error loading configuration", e);
        }
    }

    private void validateConfig() {
        List<String> requiredFields = List.of("name", "specVersion", "parallelism");
        for (String field : requiredFields) {
            if (!config.containsKey(field)) {
                throw new IllegalArgumentException("Missing required field: " + field);
            }
        }

        if (config.containsKey("sources")) {
            List<Map<String, Object>> sources = (List<Map<String, Object>>) config.get("sources");
            for (Map<String, Object> source : sources) {
                List<String> requiredSourceFields = List.of("name", "type", "dataset");
                for (String field : requiredSourceFields) {
                    if (!source.containsKey(field)) {
                        throw new IllegalArgumentException("Missing required field in source: " + field);
                    }
                }
            }
        }

        if (config.containsKey("transforms")) {
            List<Map<String, Object>> transforms = (List<Map<String, Object>>) config.get("transforms");
            for (Map<String, Object> transform : transforms) {
                List<String> requiredTransformFields = List.of("name", "sql");
                for (String field : requiredTransformFields) {
                    if (!transform.containsKey(field)) {
                        throw new IllegalArgumentException("Missing required field in transform: " + field);
                    }
                }
            }
        }

        if (config.containsKey("sinks")) {
            List<Map<String, Object>> sinks = (List<Map<String, Object>>) config.get("sinks");
            for (Map<String, Object> sink : sinks) {
                List<String> requiredSinkFields = List.of("name", "type", "from");
                for (String field : requiredSinkFields) {
                    if (!sink.containsKey(field)) {
                        throw new IllegalArgumentException("Missing required field in sink: " + field);
                    }
                }
            }
        }
    }

    private void configureFlink() {
        Configuration conf = tEnv.getConfig().getConfiguration();
        conf.setString("table.local-time-zone", "UTC");
        conf.setString("table.exec.sink.upsert-materialize", "NONE");
        conf.setString("state.backend.type", "rocksdb");
        conf.setString("state.checkpoints.dir", "file:///opt/flink/checkpoint");
        conf.setString("state.savepoints.dir", "file:///opt/flink/savepoint");
        conf.setString("execution.checkpointing.interval", "60s");
        conf.setString("execution.checkpointing.min-pause", "1s");
        conf.setString("execution.checkpointing.externalized-checkpoint-retention", "RETAIN_ON_CANCELLATION");
        conf.setString("execution.checkpointing.timeout", "30 min");
        conf.setString("execution.checkpointing.max-concurrent-checkpoints", "1");
        conf.setString("state.backend.incremental", "true");
        conf.setString("restart-strategy.fixed-delay.delay", "10 s");
        conf.setString("execution.checkpointing.tolerable-failed-checkpoints", "2147483647");
        conf.setString("sql-client.execution.result-mode", "tableau");
        conf.setString("table.exec.sink.not-null-enforcer", "ERROR");
    }

    private void createPaimonCatalog() {
        String createCatalogSQL = String.format(
                "CREATE CATALOG paimon WITH (" +
                        "  'type' = 'paimon'," +
                        "  'warehouse' = 'oss://network-testnet/warehouse'," +
                        "  'fs.oss.endpoint' = 'network-testnet.chainbasehq.com'," +
                        "  'fs.oss.accessKeyId' = '%s'," +
                        "  'fs.oss.accessKeySecret' = '%s'," +
                        "  'table-default.merge-engine' = 'deduplicate'," +
                        "  'table-default.changelog-producer' = 'input'," +
                        "  'table-default.metastore.partitioned-table' = 'false'," +
                        "  'table-default.lookup.cache-file-retention' = '1 h'," +
                        "  'table-default.lookup.cache-max-memory-size' = '256 mb'," +
                        "  'table-default.lookup.cache-max-disk-size' = '10 gb'," +
                        "  'table-default.log.scan.remove-normalize' = 'true'," +
                        "  'table-default.changelog-producer.row-deduplicate' = 'false'," +
                        "  'table-default.consumer.expiration-time' = '24 h'," +
                        "  'table-default.streaming-read-mode' = 'file'," +
                        "  'table-default.orc.bloom.filter.fpp' = '0.00001'," +
                        "  'table-default.scan.plan-sort-partition' = 'true'," +
                        "  'table-default.snapshot.expire.limit' = '10000'," +
                        "  'table-default.snapshot.num-retained.max' = '2000'" +
                        ")",
                System.getenv("OSS_ACCESS_KEY_ID"),
                System.getenv("OSS_ACCESS_KEY_SECRET")
        );
        tEnv.executeSql(createCatalogSQL);
    }

    private void registerUDFs() {
        String udfJar = "oss://network-testnet/flink-udf/flink-udf-1.0-SNAPSHOT.jar";
        tEnv.executeSql(String.format(
                "CREATE TEMPORARY SYSTEM FUNCTION Decode_Event AS 'com.chainbase.udf.DecodeEvent' " +
                        "LANGUAGE JAVA USING JAR '%s'", udfJar
        ));
        tEnv.executeSql(String.format(
                "CREATE TEMPORARY SYSTEM FUNCTION Decode_Function AS 'com.chainbase.udf.DecodeFunction' " +
                        "LANGUAGE JAVA USING JAR '%s'", udfJar
        ));
        tEnv.executeSql(String.format(
                "CREATE TEMPORARY SYSTEM FUNCTION Eth_Call AS 'com.chainbase.udf.EthCallRequest' " +
                        "LANGUAGE JAVA USING JAR '%s'", udfJar
        ));
    }

    private void createSources() {
        tEnv.useCatalog("default_catalog");
        List<Map<String, Object>> sources = (List<Map<String, Object>>) config.get("sources");
        for (Map<String, Object> source : sources) {
            if ("dataset".equals(source.get("type"))) {
                String filterClause = source.containsKey("filter") ? "WHERE " + source.get("filter") : "";
                String sql = String.format(
                        "CREATE TEMPORARY VIEW %s AS " +
                                "SELECT * FROM paimon.%s %s",
                        source.get("name"), source.get("dataset"), filterClause
                );
                tEnv.executeSql(sql);
            }
        }
    }

    private void createTransforms() {
        tEnv.useCatalog("default_catalog");
        List<Map<String, Object>> transforms = (List<Map<String, Object>>) config.get("transforms");
        System.out.println("Creating transforms:");
        for (Map<String, Object> transform : transforms) {
            String name = transform.get("name").toString();
            String sql = transform.get("sql").toString();
            System.out.println("  Creating transform: " + name);
            System.out.println("  SQL: " + sql);
            tEnv.createTemporaryView(name, tEnv.sqlQuery(sql));
            System.out.println("  Transform created successfully: " + name);
        }
        System.out.println("All transforms created.");
    }

    private void createSinks() {
        tEnv.useCatalog("default_catalog");
        List<Map<String, Object>> sinks = (List<Map<String, Object>>) config.get("sinks");
        for (Map<String, Object> sink : sinks) {
            String sinkType = sink.get("type").toString();
            switch (sinkType) {
                case "postgres":
                    createPostgresSink(sink);
                    break;
                case "starrocks":
                    createStarrocksSink(sink);
                    break;
                case "print":
                    createPrintSink(sink);
                    break;
                default:
                    String errorMessage = "Unsupported sink type: " + sinkType;
                    System.err.println(errorMessage);
                    throw new IllegalArgumentException(errorMessage);
            }
        }
    }

    private String getSchemaFromTransform(String transformName) {
        Table table = tEnv.from(transformName);
        List<TableColumn> columns = table.getSchema().getTableColumns();
        return columns.stream()
                .map(col -> "`" + col.getName() + "` " + col.getType().toString())
                .collect(Collectors.joining(", "));
    }

    private String getPostgresSchemaFromTransform(String transformName) {
        Table table = tEnv.from(transformName);
        List<TableColumn> columns = table.getSchema().getTableColumns();
        return columns.stream()
                .map(col -> {
                    String flinkType = col.getType().toString();
                    String postgresType = mapFlinkTypeToPostgresType(flinkType);
                    return col.getName() + " " + postgresType;
                })
                .collect(Collectors.joining(", "));
    }

    private String mapFlinkTypeToPostgresType(String flinkType) {
        // Remove NOT NULL from the type string if present
        String baseType = flinkType.replace(" NOT NULL", "").toUpperCase();
        
        if (baseType.startsWith("VARCHAR(") && baseType.endsWith(")")) {
            return baseType;
        }
        
        switch (baseType) {
            case "VARCHAR":
                return "VARCHAR";
            case "TINYINT":
                return "SMALLINT";
            case "SMALLINT":
                return "SMALLINT";
            case "INT":
                return "INTEGER";
            case "BIGINT":
                return "BIGINT";
            case "DECIMAL":
                if (baseType.startsWith("DECIMAL(") && baseType.endsWith(")")) {
                    String[] parts = baseType.substring(8, baseType.length() - 1).split(",");
                    return "NUMERIC(" + parts[0].trim() + "," + parts[1].trim() + ")";
                }
                return "NUMERIC";
            case "FLOAT":
                return "REAL";
            case "DOUBLE":
                return "DOUBLE PRECISION";
            case "BOOLEAN":
                return "BOOLEAN";
            case "DATE":
                return "DATE";
            case "TIME":
            case "TIME WITHOUT TIMEZONE":
                return "TIME WITHOUT TIME ZONE";
            case "TIMESTAMP":
            case "TIMESTAMP WITHOUT TIMEZONE":
                return "TIMESTAMP WITHOUT TIME ZONE";
            case "STRING":
                return "TEXT";
            case "BYTES":
                return "BYTEA";
            default:
                if (baseType.startsWith("DECIMAL")) {
                    return baseType.replace("DECIMAL", "NUMERIC");
                } else if (baseType.startsWith("TIME(")) {
                    return baseType.replace("TIME", "TIME") + " WITHOUT TIME ZONE";
                } else if (baseType.startsWith("TIMESTAMP(")) {
                    return baseType.replace("TIMESTAMP", "TIMESTAMP") + " WITHOUT TIME ZONE";
                } else if (baseType.startsWith("ARRAY")) {
                    return baseType;
                }
                throw new IllegalArgumentException("Unsupported Flink type: " + flinkType);
        }
    }

    private void createPostgresSink(Map<String, Object> sink) {
        System.out.println("Creating PostgreSQL sink...");
        String flinkSchema = getSchemaFromTransform(sink.get("from").toString());
        String postgresSchema = getPostgresSchemaFromTransform(sink.get("from").toString());
        String database = sink.get("database").toString();
        String schemaName = sink.get("schema").toString();
        String tableName = sink.get("table").toString();
        String primaryKey = sink.get("primary_key").toString();
        String username = ((Map<String, Object>)sink.get("config")).get("username").toString();
        String password = ((Map<String, Object>)sink.get("config")).get("password").toString();
        String host = ((Map<String, Object>)sink.get("config")).get("host").toString();
        String port = ((Map<String, Object>)sink.get("config")).get("port").toString();

        System.out.println("Connecting to PostgreSQL and creating database/table if not exists...");
        try (Connection conn = DriverManager.getConnection("jdbc:postgresql://" + host + ":" + port + "/postgres", username, password);
             Statement stmt = conn.createStatement()) {
            
            // Check if database exists
            ResultSet rs = stmt.executeQuery("SELECT 1 FROM pg_database WHERE datname = '" + database + "'");
            if (!rs.next()) {
                // Create database if it doesn't exist
                stmt.execute("CREATE DATABASE " + database);
                System.out.println("Database created: " + database);
            } else {
                System.out.println("Database already exists: " + database);
            }
            
            // Connect to the new database
            try (Connection dbConn = DriverManager.getConnection("jdbc:postgresql://" + host + ":" + port + "/" + database, username, password);
                 Statement dbStmt = dbConn.createStatement()) {
                
                // Create schema if it doesn't exist
                dbStmt.execute("CREATE SCHEMA IF NOT EXISTS " + schemaName);
                System.out.println("Schema created or already exists: " + schemaName);
                
                // Create table if it doesn't exist
                String createTableSQL = "CREATE TABLE IF NOT EXISTS " + schemaName + "." + tableName + " (" + postgresSchema + ", PRIMARY KEY (" + primaryKey + "))";
                System.out.println("Creating table: " + createTableSQL);
                dbStmt.execute(createTableSQL);
                System.out.println("Table created or already exists: " + schemaName + "." + tableName);
            }
        } catch (Exception e) {
            System.err.println("Error creating database or table for PostgreSQL sink: " + e.getMessage());
            throw new RuntimeException("Error creating database or table for PostgreSQL sink", e);
        }

        System.out.println("Creating Flink SQL table for PostgreSQL sink...");
        String sql = String.format(
                "CREATE TABLE %s (%s, PRIMARY KEY (%s) NOT ENFORCED) WITH (" +
                        "  'connector' = 'jdbc'," +
                        "  'url' = 'jdbc:postgresql://%s:%s/%s'," +
                        "  'table-name' = '%s.%s'," +
                        "  'username' = '%s'," +
                        "  'password' = '%s'" +
                        ")",
                sink.get("name"), flinkSchema, primaryKey, host, port, database,
                schemaName, tableName, username, password
        );
        System.out.println("Executing SQL for PostgreSQL sink: " + sql);
        tEnv.executeSql(sql);
        System.out.println("PostgreSQL sink created successfully.");
    }

    private void createStarrocksSink(Map<String, Object> sink) {
        System.out.println("Creating StarRocks sink...");
        String schema = getSchemaFromTransform(sink.get("from").toString());
        String sql = String.format(
                "CREATE TABLE %s (%s) WITH (" +
                        "  'connector' = 'starrocks'," +
                        "  'jdbc-url' = 'jdbc:mysql://localhost:9030/%s'," +
                        "  'load-url' = 'localhost:8030'," +
                        "  'database-name' = '%s'," +
                        "  'table-name' = '%s'," +
                        "  'username' = '%s'," +
                        "  'password' = '%s'" +
                        ")",
                sink.get("name"), schema, sink.get("database"),
                sink.get("database"), sink.get("table"),
                ((Map<String, Object>)sink.get("config")).get("username"),
                ((Map<String, Object>)sink.get("config")).get("password")
        );
        System.out.println("Executing SQL for StarRocks sink: " + sql);
        tEnv.executeSql(sql);
        System.out.println("StarRocks sink created successfully.");
    }

    private void createPrintSink(Map<String, Object> sink) {
        System.out.println("Creating print sink...");
        String schema = getSchemaFromTransform(sink.get("from").toString());
        String sql = String.format(
                "CREATE TABLE %s (%s) WITH ('connector' = 'print', 'standard-error' = 'true')",
                sink.get("name"), schema
        );
        System.out.println("Executing SQL for print sink: " + sql);
        tEnv.executeSql(sql);
        System.out.println("Print sink created successfully.");
    }

    public void execute() throws Exception {
        registerUDFs();
        createSources();
        createTransforms();
        createSinks();

        List<Map<String, Object>> sinks = (List<Map<String, Object>>) config.get("sinks");
        List<TableResult> results = new ArrayList<>();

        for (Map<String, Object> sink : sinks) {
            TableResult result = tEnv.from(sink.get("from").toString())
                                     .executeInsert(sink.get("name").toString());
            results.add(result);
        }

        for (TableResult result : results) {
            result.await();
        }

        env.execute(config.get("name").toString());
    }

    public static void main(String[] args) throws Exception {
        String configPath = args.length > 0 ? args[0] : "manuscript.yaml";
        ETLProcessor processor = new ETLProcessor(configPath);
        processor.execute();
    }
}