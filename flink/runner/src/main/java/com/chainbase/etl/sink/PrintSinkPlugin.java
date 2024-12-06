package com.chainbase.etl.sink;

import com.chainbase.etl.AbstractEtlSinkPlugin;
import com.chainbase.etl.model.Sink;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class PrintSinkPlugin extends AbstractEtlSinkPlugin {

    public static final String IDENTIFIER = "print";
    private static final Logger logger = LoggerFactory.getLogger(PrintSinkPlugin.class);

    public PrintSinkPlugin(StreamTableEnvironment tEnv) {
        super(tEnv);
    }

    @Override
    public String factoryIdentifier() {
        return IDENTIFIER;
    }

    @Override
    public void sink(Sink sink) {
        logger.info("Creating print sink...");
        String schema = getSchemaFromTransform(sink.getFrom());
        String sql = String.format(
                "CREATE TABLE %s (%s)" +
                        " WITH (" +
                        "  'connector' = 'print', " +
                        "  'standard-error' = 'true')",
                sink.getName(), schema
        );
        logger.info("Executing SQL for print sink: {}", sql);
        tEnv.executeSql(sql);
        logger.info("Print sink created successfully.");
    }
}
