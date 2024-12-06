package com.chainbase.etl.model;

import java.io.Serializable;

public class Source implements Serializable {
    private static final long serialVersionUID = 5685521189643221375L;
    private String name;
    private String type;
    private String dataset;
    private String filters;

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

    public String getDataset() {
        return dataset;
    }

    public void setDataset(String dataset) {
        this.dataset = dataset;
    }

    public String getFilters() {
        return filters;
    }

    public void setFilters(String filters) {
        this.filters = filters;
    }

    public void validate() {
        if (isNullOrEmpty(name)) {
            throw new IllegalArgumentException("Missing required field in source: name");
        }
        if (isNullOrEmpty(type)) {
            throw new IllegalArgumentException("Missing required field in source: type");
        }
        if (isNullOrEmpty(dataset)) {
            throw new IllegalArgumentException("Missing required field in source: dataset");
        }
    }
    private boolean isNullOrEmpty(String str) {
        return str == null || str.trim().isEmpty();
    }
}
