package com.chainbase.etl.model;

import java.io.Serializable;
import java.util.Map;

public class Sink implements Serializable {
    private static final long serialVersionUID = 5685521189643221375L;
    private String name;
    private String type;
    private String from;
    private String file_name;
    private String schema;
    private String database;
    private String table;

    private String runtimeMode = "streaming";

    private String primary_key;
    private Map<String, String> config;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    public String getFrom() {
        return from;
    }

    public void setFrom(String from) {
        this.from = from;
    }

    public String getFile_name() {
        return file_name;
    }

    public void setFile_name(String file_name) {
        this.file_name = file_name;
    }

    public Map<String, String> getConfig() {
        return config;
    }

    public void setConfig(Map<String, String> config) {
        this.config = config;
    }

    public String getDatabase() {
        return database;
    }

    public void setDatabase(String database) {
        this.database = database;
    }

    public String getTable() {
        return table;
    }

    public void setTable(String table) {
        this.table = table;
    }

    public String getSchema() {
        return schema;
    }

    public void setSchema(String schema) {
        this.schema = schema;
    }

    public String getPrimary_key() {
        return primary_key;
    }

    public void setPrimary_key(String primary_key) {
        this.primary_key = primary_key;
    }

    public void setRuntimeMode(String runtimeMode) {
        this.runtimeMode = runtimeMode;
    }

    public String getRuntimeMode() {
        return runtimeMode;
    }

    public void validate() {
        if (isNullOrEmpty(name)) {
            throw new IllegalArgumentException("Missing required field in sink: name");
        }
        if (isNullOrEmpty(type)) {
            throw new IllegalArgumentException("Missing required field in sink: type");
        }
        if (isNullOrEmpty(from)) {
            throw new IllegalArgumentException("Missing required field in sink: from");
        }
    }
    private boolean isNullOrEmpty(String value) {
        return value == null || value.trim().isEmpty();
    }
}
