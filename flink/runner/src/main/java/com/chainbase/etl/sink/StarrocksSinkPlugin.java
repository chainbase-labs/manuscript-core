package com.chainbase.etl.sink;

import com.chainbase.etl.AbstractEtlSinkPlugin;
import com.chainbase.etl.model.Sink;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class StarrocksSinkPlugin extends AbstractEtlSinkPlugin {

    public static final String IDENTIFIER = "starrocks";
    private static final Logger logger = LoggerFactory.getLogger(StarrocksSinkPlugin.class);

    public StarrocksSinkPlugin(StreamTableEnvironment tEnv) {
        super(tEnv);
    }

    @Override
    public String factoryIdentifier() {
        return IDENTIFIER;
    }

    @Override
    public void sink(Sink sink) {
        logger.info("Creating StarRocks sink...");
        String schema = getSchemaFromTransform(sink.getFrom());
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
                sink.getName(), schema, sink.getDatabase(),
                sink.getDatabase(), sink.getTable(),
                sink.getConfig().get("username"),
                sink.getConfig().get("password")
        );
        logger.info("Executing SQL for StarRocks sink: {}", sql);
        tEnv.executeSql(sql);
        logger.info("StarRocks sink created successfully.");
    }
}
