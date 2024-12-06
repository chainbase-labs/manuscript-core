package com.chainbase.etl.sink;

import com.chainbase.etl.AbstractEtlSinkPlugin;
import com.chainbase.etl.model.Sink;
import org.apache.flink.annotation.docs.Documentation;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.apache.flink.table.api.config.ExecutionConfigOptions;
import org.apache.flink.table.api.config.TableConfigOptions;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class FileSystemSinkPlugin extends AbstractEtlSinkPlugin {
    public static final String IDENTIFIER = "filesystem";
    private static final Logger logger = LoggerFactory.getLogger(FileSystemSinkPlugin.class);

    public FileSystemSinkPlugin(StreamTableEnvironment tEnv) {
        super(tEnv);
    }

    @Override
    public String factoryIdentifier() {
        return IDENTIFIER;
    }

    @Override
    public void sink(Sink sink) {
        logger.info("Creating filesystem sink...");
        String schema = getSchemaFromTransform(sink.getFrom());
        String path = "/opt/flink/sink_file_path/" + sink.getFile_name();

        String format = sink.getRuntimeMode().equalsIgnoreCase("batch")?"json":"debezium-json";
        String sql = String.format(
                "CREATE TABLE %s (%s) " +
                        " WITH (" +
                        "  'connector' = 'filesystem'," +
                        "  'path' = '%s'," +
                        "  'format' = '%s'" +
                        ")",
                sink.getName(), schema, path,format
        );

        logger.info("Executing SQL for filesystem sink: {}", sql);
        tEnv.executeSql(sql);
        logger.info("Filesystem sink created successfully.");
    }
}
