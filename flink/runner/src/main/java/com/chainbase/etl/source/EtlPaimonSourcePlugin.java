package com.chainbase.etl.source;

import com.chainbase.etl.EtlSourcePlugin;
import com.chainbase.etl.model.Source;
import com.chainbase.etl.model.Transform;
import org.apache.commons.lang3.StringUtils;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class EtlPaimonSourcePlugin implements EtlSourcePlugin {
    private static final Logger logger = LoggerFactory.getLogger(EtlPaimonSourcePlugin.class);
    public static final String IDENTIFIER = "paimon";

    public StreamTableEnvironment tEnv;
    public EtlPaimonSourcePlugin(StreamTableEnvironment tEnv) {
        this.tEnv = tEnv;
    }

    @Override
    public String factoryIdentifier() {
        return IDENTIFIER;
    }

    @Override
    public void source(List<Source> sources) {
        tEnv.useCatalog("default_catalog");
        sources.stream().forEach(e -> {
            if ("dataset".equalsIgnoreCase(e.getType())) {
                String filterClause = StringUtils.isNotBlank(e.getFilters()) ? "WHERE " + e.getFilters() : "";
                String sql = String.format(
                        "CREATE TEMPORARY VIEW %s AS " +
                                "SELECT * FROM paimon.%s %s",
                        e.getName(), e.getDataset(), filterClause
                );
                logger.info("Executing SQL for source sink: {}", sql);
                tEnv.executeSql(sql);
            }
        });
    }

    @Override
    public void transform(List<Transform> transforms) {
        tEnv.useCatalog("default_catalog");
        transforms.stream().forEach(e -> {
            tEnv.createTemporaryView(e.getName(), tEnv.sqlQuery(e.getSql()));
            logger.info("Executing SQL for transforms sink: {}", e.getSql());
        });
    }
}
