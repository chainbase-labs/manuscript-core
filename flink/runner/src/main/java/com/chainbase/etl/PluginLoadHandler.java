package com.chainbase.etl;

import com.chainbase.etl.factory.PluginFactory;
import com.chainbase.etl.model.ETL;
import com.chainbase.etl.model.Sink;
import com.chainbase.etl.model.Source;
import com.chainbase.etl.model.Transform;
import com.chainbase.etl.source.EtlPaimonSourcePlugin;
import com.chainbase.udf.*;
import org.apache.commons.lang3.StringUtils;
import org.apache.flink.configuration.Configuration;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class PluginLoadHandler {
    private static final Logger logger = LoggerFactory.getLogger(PluginLoadHandler.class);
    public static void load(StreamTableEnvironment tEnv, ETL etl) {
        new PluginLoadHandler.PluginLoadBuilder(tEnv)
                .setUpFlinkConfig(etl)
                .registerUDFs()
                .createPaimonCatalog()
                .sources(etl.getSources(), etl.getTransforms())
                .sink(etl.getSinks());
    }

    public static class PluginLoadBuilder {
        StreamTableEnvironment tEnv;

        public PluginLoadBuilder(StreamTableEnvironment tEnv) {
            this.tEnv = tEnv;
        }

        private PluginLoadBuilder registerUDFs() {
            tEnv.createTemporarySystemFunction("Decode_Event", DecodeEvent.class);
            tEnv.createTemporarySystemFunction("Decode_Function", DecodeFunction.class);
            tEnv.createTemporarySystemFunction("Eth_Call", EthCallRequest.class);
            tEnv.createTemporarySystemFunction("ROW_TO_JSON", RowToJsonFunction.class);
            tEnv.createTemporarySystemFunction("ARRAY_TO_JSON", ArrayToJsonFunction.class);
            logger.info("register udf success");
            return this;
        }

        private PluginLoadBuilder createPaimonCatalog() {
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

            logger.info("create paimon catalog success");
            return this;
        }

        private PluginLoadBuilder setUpFlinkConfig(ETL etl) {
            Configuration conf = tEnv.getConfig().getConfiguration();
            conf.setString("table.local-time-zone", "UTC");
            conf.setString("table.exec.sink.upsert-materialize", "NONE");
            conf.setString("state.backend.type", "rocksdb");
            conf.setString("sql-client.execution.result-mode", "tableau");
            conf.setString("table.exec.sink.not-null-enforcer", "ERROR");
            if (!judgeRuntimeMode(etl)) {
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
            }
            logger.info("setup flink config success");
            return this;
        }

        private boolean judgeRuntimeMode(ETL etl) {
            return StringUtils.isNotBlank(etl.getRuntimeMode())
                    && (etl.getRuntimeMode()).equalsIgnoreCase("batch");
        }

        private PluginLoadBuilder sources(List<Source> sources, List<Transform> transforms) {
            EtlPaimonSourcePlugin sourcePlugin = new EtlPaimonSourcePlugin(this.tEnv);
            sourcePlugin.source(sources);
            sourcePlugin.transform(transforms);

            return this;
        }

        private void sink(List<Sink> sinks) {
            sinks.stream().forEach(z -> {
                try {
                    EtlSinkPlugin sinkPlugin = PluginFactory.getImplementation(z.getType(),tEnv);
                    sinkPlugin.sink(z);
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            });
        }
    }
}

