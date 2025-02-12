package com.chainbase.manuscript;

import com.chainbase.udf.*;
import freemarker.template.Template;
import freemarker.template.TemplateExceptionHandler;

import java.io.StringWriter;

import org.apache.flink.configuration.Configuration;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.table.api.EnvironmentSettings;
import org.apache.flink.table.api.Table;
import org.apache.flink.table.api.TableColumn;
import org.apache.flink.table.api.TableResult;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.apache.flink.table.catalog.Column;
import org.apache.flink.table.catalog.ResolvedSchema;
import org.apache.flink.table.expressions.Expression;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.yaml.snakeyaml.Yaml;

import java.io.FileInputStream;
import java.io.InputStream;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import static org.apache.flink.table.api.Expressions.$;
import static org.apache.flink.table.api.Expressions.call;

public class ETLProcessor {

  private static final Logger logger = LoggerFactory.getLogger(ETLProcessor.class);
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
    String ck_dir = (String) config.computeIfAbsent("state_checkpoints_dir", k -> "file:///opt/flink/checkpoint");
    String sv_dir = (String) config.computeIfAbsent("state_savepoints_dir", k -> "file:///opt/flink/savepoint");

    configureFlink(ck_dir, sv_dir);
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

  private void configureFlink(String ck_dir, String sv_dir) {
    Configuration conf = tEnv.getConfig().getConfiguration();
    conf.setString("table.local-time-zone", "UTC");
    conf.setString("table.exec.sink.upsert-materialize", "NONE");
    conf.setString("state.backend.type", "rocksdb");
    conf.setString("state.checkpoints.dir", ck_dir);
    conf.setString("state.savepoints.dir", sv_dir);
    conf.setString("execution.checkpointing.interval", "60s");
    conf.setString("execution.checkpointing.min-pause", "1s");
    conf.setString("execution.checkpointing.externalized-checkpoint-retention",
            "RETAIN_ON_CANCELLATION");
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
    tEnv.createTemporarySystemFunction("Decode_Event", DecodeEvent.class);
    tEnv.createTemporarySystemFunction("Decode_Function", DecodeFunction.class);
    tEnv.createTemporarySystemFunction("Eth_Call", EthCallRequest.class);

    tEnv.createTemporarySystemFunction("ROW_TO_JSON", RowToJsonFunction.class);
    tEnv.createTemporarySystemFunction("ARRAY_TO_JSON", ArrayToJsonFunction.class);
    tEnv.createTemporarySystemFunction("GET_TOKEN_META", GetTokenMeta.class);
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

  private void createTransforms() throws Exception {
    tEnv.useCatalog("default_catalog");
    List<Map<String, Object>> transforms = (List<Map<String, Object>>) config.get("transforms");
    logger.info("Creating transforms:");
    for (Map<String, Object> transform : transforms) {
      String name = transform.get("name").toString();
      String sql = transform.get("sql").toString();
      logger.info("  Creating transform: {}", name);
      Map<String, Object> params = (Map<String, Object>) transform.getOrDefault(
              "params", null);
      if (params != null) {
        sql = render(name, sql, params);
      }
      logger.info("  SQL: {}", sql);
      tEnv.createTemporaryView(name, tEnv.sqlQuery(sql));
      logger.info("  Transform created successfully: {}", name);
    }
    logger.info("All transforms created.");
  }

  private String render(String name, String sql, Map<String, Object> params) throws Exception {
    freemarker.template.Configuration freemarkerConfig = new freemarker.template.Configuration(
            freemarker.template.Configuration.VERSION_2_3_31);
    freemarkerConfig.setDefaultEncoding("UTF-8");
    freemarkerConfig.setTemplateExceptionHandler(TemplateExceptionHandler.RETHROW_HANDLER);
    Template template = new Template(name, sql, freemarkerConfig);
    StringWriter stringWriter = new StringWriter();
    template.process(params, stringWriter);
    return stringWriter.toString();
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
        case "filesystem":
          createFilesystemSink(sink);
          break;
        case "kafka":

          break;
        default:
          String errorMessage = "Unsupported sink type: " + sinkType;
          logger.error(errorMessage);
          throw new IllegalArgumentException(errorMessage);
      }
    }
  }

  private String getSchemaFromTransform(String transformName) {
    Table table = tEnv.from(transformName);
    List<TableColumn> columns = table.getSchema().getTableColumns();
    return columns.stream().map(col -> {
      switch (col.getType().getLogicalType().getTypeRoot()) {
        case ARRAY:
        case ROW:
          return "`" + col.getName() + "` STRING";
        default:
          return "`" + col.getName() + "` " + col.getType().toString();
      }
    }).collect(Collectors.joining(", "));
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
          return "TEXT";
        } else if (baseType.startsWith("ROW")) {
          return "TEXT";
        }
        throw new IllegalArgumentException("Unsupported Flink type: " + flinkType);
    }
  }

  private void createKafkaSink(Map<String, Object> sink) {
    logger.info("Creating Kafka sink...");
    String flinkSchema = getSchemaFromTransform(sink.get("from").toString());
    String sql = String.format(
            "CREATE TABLE %s (%s) WITH (" +
                    "  'connector' = 'kafka'," +
                    "  'topic' = '%s'," +
                    "  'properties.bootstrap.servers' = '%s'," +
                    "  'properties.security.protocol' = 'SASL_SSL'," +
                    "  'properties.ssl.truststore.location' = '%s'," +
                    "  'properties.ssl.truststore.password' = '%s'," +
                    "  'properties.sasl.mechanism' = 'PLAIN'," +
                    "  'properties.ssl.endpoint.identification.algorithm' = ''," +
                    "  'properties.sasl.jaas.config' = 'org.apache.kafka.common.security.plain.PlainLoginModule required username=\"%s\" password=\"%s\"'" +
                    ")",
            sink.get("name"), flinkSchema,
            sink.get("topic"),
            sink.get("kafka_servers"),
            sink.get("truststore_location"),
            sink.get("truststore_password"),
            ((Map<String, Object>) sink.get("config")).get("username"),
            ((Map<String, Object>) sink.get("config")).get("password")
    );

    logger.info("Executing SQL for Kafka sink: {}", sql);
    tEnv.executeSql(sql);
    logger.info("Kafka sink created successfully.");
  }

  private void createPostgresSink(Map<String, Object> sink) {
    logger.info("Creating PostgreSQL sink...");
    String flinkSchema = getSchemaFromTransform(sink.get("from").toString());
    String postgresSchema = getPostgresSchemaFromTransform(sink.get("from").toString());
    String database = sink.get("database").toString();
    String schemaName = sink.get("schema").toString();
    String tableName = sink.get("table").toString();
    String primaryKey = sink.get("primary_key").toString();
    String username = ((Map<String, Object>) sink.get("config")).get("username").toString();
    String password = ((Map<String, Object>) sink.get("config")).get("password").toString();
    String host = ((Map<String, Object>) sink.get("config")).get("host").toString();
    String port = ((Map<String, Object>) sink.get("config")).get("port").toString();

    logger.info("Connecting to PostgreSQL and creating database/table if not exists...");
    try (Connection conn = DriverManager.getConnection(
            "jdbc:postgresql://" + host + ":" + port + "/postgres", username, password);
         Statement stmt = conn.createStatement()) {

      // Check if database exists
      ResultSet rs = stmt.executeQuery(
              "SELECT 1 FROM pg_database WHERE datname = '" + database + "'");
      if (!rs.next()) {
        // Create database if it doesn't exist
        stmt.execute("CREATE DATABASE " + database);
        logger.info("Database created: {}", database);
      } else {
        logger.info("Database already exists: {}", database);
      }

      // Connect to the new database
      try (Connection dbConn = DriverManager.getConnection(
              "jdbc:postgresql://" + host + ":" + port + "/" + database, username, password);
           Statement dbStmt = dbConn.createStatement()) {

        // Create schema if it doesn't exist
        dbStmt.execute("CREATE SCHEMA IF NOT EXISTS " + schemaName);
        logger.info("Schema created or already exists: {}", schemaName);

        // Create table if it doesn't exist
        String createTableSQL =
                "CREATE TABLE IF NOT EXISTS " + schemaName + "." + tableName + " (" + postgresSchema
                        + ", PRIMARY KEY (" + primaryKey + "))";
        logger.info("Creating table: {}", createTableSQL);
        dbStmt.execute(createTableSQL);
        logger.info("Table created or already exists: {}.{}", schemaName, tableName);
      }
    } catch (Exception e) {
      logger.error("Error creating database or table for PostgreSQL sink: {}", e.getMessage());
      throw new RuntimeException("Error creating database or table for PostgreSQL sink", e);
    }

    logger.info("Creating Flink SQL table for PostgreSQL sink...");
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
    logger.info("Executing SQL for PostgreSQL sink: {}", sql);
    tEnv.executeSql(sql);
    logger.info("PostgreSQL sink created successfully.");
  }

  private void createStarrocksSink(Map<String, Object> sink) {
    logger.info("Creating StarRocks sink...");
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
            ((Map<String, Object>) sink.get("config")).get("username"),
            ((Map<String, Object>) sink.get("config")).get("password")
    );
    logger.info("Executing SQL for StarRocks sink: {}", sql);
    tEnv.executeSql(sql);
    logger.info("StarRocks sink created successfully.");
  }

  private void createPrintSink(Map<String, Object> sink) {
    logger.info("Creating print sink...");
    String schema = getSchemaFromTransform(sink.get("from").toString());
    String sql = String.format(
            "CREATE TABLE %s (%s) WITH ('connector' = 'print', 'standard-error' = 'true')",
            sink.get("name"), schema
    );
    logger.info("Executing SQL for print sink: {}", sql);
    tEnv.executeSql(sql);
    logger.info("Print sink created successfully.");
  }

  private void createFilesystemSink(Map<String, Object> sink) {
    logger.info("Creating filesystem sink...");
    String schema = getSchemaFromTransform(sink.get("from").toString());
    String fileName = sink.get("file_name").toString();
    String path = "/opt/flink/sink_file_path/" + fileName;

    String sql = String.format(
            "CREATE TABLE %s (%s) WITH (" +
                    "  'connector' = 'filesystem'," +
                    "  'path' = '%s'," +
                    "  'format' = 'debezium-json'" +
                    ")",
            sink.get("name"), schema, path
    );
    logger.info("Executing SQL for filesystem sink: {}", sql);
    tEnv.executeSql(sql);
    logger.info("Filesystem sink created successfully.");
  }


  public Table castTableColumns(Table sourceTable) {
    ResolvedSchema schema = sourceTable.getResolvedSchema();

    List<Expression> expressions = new ArrayList<>();

    for (Column column : schema.getColumns()) {
      String columnName = column.getName();

      switch (column.getDataType().getLogicalType().getTypeRoot()) {
        case ARRAY:
          expressions.add(call("ARRAY_TO_JSON", $(columnName)).as(columnName));
          break;
        case ROW:
          expressions.add(call("ROW_TO_JSON", $(columnName)).as(columnName));
          break;
        default:
          expressions.add($(columnName));
          break;
      }
    }

    return sourceTable.select(expressions.toArray(new Expression[0]));
  }

  public void execute() throws Exception {
    registerUDFs();
    createSources();
    createTransforms();
    createSinks();

    List<Map<String, Object>> sinks = (List<Map<String, Object>>) config.get("sinks");
    List<TableResult> results = new ArrayList<>();

    for (Map<String, Object> sink : sinks) {
      Table sourceTable = tEnv.from(sink.get("from").toString());

      Table castTable = castTableColumns(sourceTable);

      TableResult result = castTable.executeInsert(sink.get("name").toString());
      results.add(result);
    }

    for (TableResult result : results) {
      result.await();
    }

    env.execute(config.get("name").toString());
  }

  public static void main(String[] args) throws Exception {
    String configPath = args.length > 0 ? args[0] : "com/chainbase/manuscript/manuscript.yaml";
    ETLProcessor processor = new ETLProcessor(configPath);
    processor.execute();
  }
}