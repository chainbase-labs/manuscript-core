package com.chainbase.etl.flink;

import com.chainbase.etl.PluginLoadHandler;
import com.chainbase.etl.model.ETL;
import com.chainbase.etl.model.Transform;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.table.api.EnvironmentSettings;
import org.apache.flink.table.api.Table;
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
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import static org.apache.flink.table.api.Expressions.$;
import static org.apache.flink.table.api.Expressions.call;

public class ETLProcessor {
    private static final Logger logger = LoggerFactory.getLogger(ETLProcessor.class);
    private final StreamExecutionEnvironment env;
    private final StreamTableEnvironment tEnv;
    private final ETL etl;

    public ETLProcessor(String configPath) {
        this.etl = loadConfig(configPath);
        this.etl.validate();

        EnvironmentSettings.Builder instance = EnvironmentSettings.newInstance();

        EnvironmentSettings settings = etl.getRuntimeMode() != null
                && etl.getRuntimeMode().equalsIgnoreCase("batch")
                ? instance.inBatchMode().build(): instance.inStreamingMode().build();

        this.env = StreamExecutionEnvironment.getExecutionEnvironment();
        this.tEnv = StreamTableEnvironment.create(env,settings);
    }

    private ETL loadConfig(String configPath) {
        try (InputStream input = new FileInputStream(configPath)) {
            Yaml yaml = new Yaml();
            return yaml.loadAs(input, ETL.class);
        } catch (Exception e) {
            throw new RuntimeException("Error loading configuration", e);
        }
    }

    public void execute() {
        PluginLoadHandler.load(this.tEnv, this.etl);

        Map<String, Transform> transformMap = etl.getTransforms().stream()
                .collect(Collectors.toMap(Transform::getName, obj -> obj));

        List<TableResult> results = new ArrayList<>();
        etl.getSinks().stream().forEach(e -> {
            if (transformMap.containsKey(e.getFrom())) {
                TableResult result = castTableColumns(tEnv.from(e.getFrom())).executeInsert(e.getName());
                results.add(result);
            }
        });

        try {
            for (TableResult result : results) {
                result.await();
            }
            env.execute(etl.getName());
        } catch (Exception e) {
            logger.error("flink execute error: " + e.getMessage());
        }
        env.clearJobListeners();
    }

    private Table castTableColumns(Table sourceTable) {
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

    public static void main(String[] args) {
         String configPath = args.length > 0 ? args[0] : "manuscript.yaml";
         ETLProcessor processor = new ETLProcessor(configPath);
         processor.execute();
    }
}
