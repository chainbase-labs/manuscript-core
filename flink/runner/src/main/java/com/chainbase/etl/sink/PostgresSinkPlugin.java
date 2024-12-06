package com.chainbase.etl.sink;

import com.chainbase.etl.AbstractEtlSinkPlugin;
import com.chainbase.etl.model.Sink;
import org.apache.flink.table.api.Table;
import org.apache.flink.table.api.TableColumn;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.apache.flink.table.types.logical.LogicalType;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.List;
import java.util.stream.Collectors;

public class PostgresSinkPlugin extends AbstractEtlSinkPlugin {
    public static final String IDENTIFIER = "postgres";
    private static final Logger logger = LoggerFactory.getLogger(PostgresSinkPlugin.class);

    public PostgresSinkPlugin(StreamTableEnvironment tEnv) {
        super(tEnv);
    }

    @Override
    public String factoryIdentifier() {
        return IDENTIFIER;
    }

    @Override
    public void sink(Sink sink) {
        logger.info("Creating PostgreSQL sink...");
        String flinkSchema = getSchemaFromTransform(sink.getFrom());
        String postgresSchema = getPostgresSchemaFromTransform(tEnv, sink.getFrom());
        createDbAndTable(sink, postgresSchema);
        logger.info("Creating Flink SQL table for PostgreSQL sink...");
        String sql = String.format(
                "CREATE TABLE %s (%s, PRIMARY KEY (%s) NOT ENFORCED) " +
                        " WITH (" +
                        "  'connector' = 'jdbc'," +
                        "  'url' = 'jdbc:postgresql://%s:%s/%s'," +
                        "  'table-name' = '%s.%s'," +
                        "  'username' = '%s'," +
                        "  'password' = '%s'" +
                        ")",
                sink.getName(), flinkSchema, sink.getPrimary_key(), sink.getConfig().get("host"), sink.getConfig().get("port"), sink.getDatabase(),
                sink.getSchema(), sink.getTable(), sink.getConfig().get("username"), sink.getConfig().get("password")
        );
        logger.info("Executing SQL for PostgreSQL sink: {}", sql);
        tEnv.executeSql(sql);
        logger.info("PostgreSQL sink created successfully.");
    }

    private void createDbAndTable(Sink sink, String postgresSchema) {
        String database = sink.getDatabase();
        String schemaName = sink.getSchema();
        String tableName = sink.getTable();
        String primaryKey = sink.getPrimary_key();
        String username = sink.getConfig().get("username");
        String password = sink.getConfig().get("password");
        String host = sink.getConfig().get("host");
        String port = sink.getConfig().get("port");

        logger.info("Connecting to PostgreSQL and creating database/table if not exists...");
        try (Connection conn = DriverManager.getConnection("jdbc:postgresql://" + host + ":" + port + "/postgres", username, password);
             Statement stmt = conn.createStatement()) {

            // Check if database exists
            ResultSet rs = stmt.executeQuery("SELECT 1 FROM pg_database WHERE datname = '" + database + "'");
            if (!rs.next()) {
                // Create database if it doesn't exist
                stmt.execute("CREATE DATABASE " + database);
                logger.info("Database created: {}", database);
            } else {
                logger.info("Database already exists: {}", database);
            }

            try (Connection dbConn = DriverManager.getConnection("jdbc:postgresql://" + host + ":" + port + "/" + database, username, password);
                 Statement dbStmt = dbConn.createStatement()) {

                // Create schema if it doesn't exist
                dbStmt.execute("CREATE SCHEMA IF NOT EXISTS " + schemaName);
                logger.info("Schema created or already exists: {}", schemaName);

                // Create table if it doesn't exist
                String createTableSQL = "CREATE TABLE IF NOT EXISTS " + schemaName + "." + tableName + " (" + postgresSchema + ", PRIMARY KEY (" + primaryKey + "))";
                logger.info("Creating table: {}", createTableSQL);
                dbStmt.execute(createTableSQL);
                logger.info("Table created or already exists: {}.{}", schemaName, tableName);
            }
        } catch (Exception e) {
            logger.error("Error creating database or table for PostgreSQL sink: {}", e.getMessage());
            throw new RuntimeException("Error creating database or table for PostgreSQL sink", e);
        }
    }

    private String getPostgresSchemaFromTransform(StreamTableEnvironment tEnv, String transformName) {
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

    private String getPostgresSchemaFromTransformV2(StreamTableEnvironment tEnv, String transformName) {
        Table table = tEnv.from(transformName);
        return table.getResolvedSchema().getColumns().stream()
                .map(e -> createInternalConverter(e.getDataType().getLogicalType()))
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


    protected String createInternalConverter(LogicalType type) {
        switch (type.getTypeRoot()) {
            case BOOLEAN:
            case FLOAT:
            case DOUBLE:
                return "DOUBLE";
            case TINYINT:
            case SMALLINT:
                return "SMALLINT";
            case INTEGER:
                return "INTEGER";
            case BIGINT:
                return "BIGINT";
            case DECIMAL:
                return "NUMERIC";
            case DATE:
                return "DATE";
            case TIME_WITHOUT_TIME_ZONE:
                return "TIME_WITHOUT_TIME_ZONE";
            case TIMESTAMP_WITH_TIME_ZONE:
                return "TIMESTAMP_WITH_TIME_ZONE";
            case TIMESTAMP_WITHOUT_TIME_ZONE:
                return "TIMESTAMP_WITHOUT_TIME_ZONE";
            case CHAR:
            case VARCHAR:
            case BINARY:
            case VARBINARY:
            case ARRAY:
                return "VARCHAR";
            case ROW:
            case MAP:
            case MULTISET:
            case RAW:
            default:
                throw new UnsupportedOperationException("Unsupported type:" + type);
        }
    }
}
