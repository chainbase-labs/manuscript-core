package com.chainbase.etl.model;

import java.io.Serializable;

public class Transform implements Serializable {
    private static final long serialVersionUID = 5685521189643221375L;
    private String name;
    private String sql;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getSql() {
        return sql;
    }

    public void setSql(String sql) {
        this.sql = sql;
    }

    public void validate() {
        if (isNullOrEmpty(name)) {
            throw new IllegalArgumentException("Missing required field in transform: name");
        }
        if (isNullOrEmpty(sql)) {
            throw new IllegalArgumentException("Missing required field in transform: sql");
        }
    }

    private boolean isNullOrEmpty(String str) {
        return str == null || str.trim().isEmpty();
    }
}
