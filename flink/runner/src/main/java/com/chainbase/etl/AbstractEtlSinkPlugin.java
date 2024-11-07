package com.chainbase.etl;

import org.apache.flink.table.api.Table;
import org.apache.flink.table.api.TableColumn;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;

import java.util.List;
import java.util.stream.Collectors;

public abstract class AbstractEtlSinkPlugin implements EtlSinkPlugin {

    protected StreamTableEnvironment tEnv;

    public AbstractEtlSinkPlugin(StreamTableEnvironment tEnv){
        this.tEnv = tEnv;
    }

    protected String getSchemaFromTransform(String transformName) {
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
}
