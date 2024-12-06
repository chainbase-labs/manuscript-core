package com.chainbase.etl.model;


import java.io.Serializable;
import java.util.List;

public class ETL implements Serializable {
    private static final long serialVersionUID = 5685521189643221375L;
    private String name;
    private String specVersion;
    private String parallelism;
    private String runtimeMode = "streaming";
    private List<Source> sources;
    private List<Transform> transforms;
    private List<Sink> sinks;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getSpecVersion() {
        return specVersion;
    }

    public void setSpecVersion(String specVersion) {
        this.specVersion = specVersion;
    }

    public String getParallelism() {
        return parallelism;
    }

    public void setParallelism(String parallelism) {
        this.parallelism = parallelism;
    }

    public List<Source> getSources() {
        return sources;
    }

    public void setSources(List<Source> sources) {
        this.sources = sources;
    }

    public List<Transform> getTransforms() {
        return transforms;
    }

    public void setTransforms(List<Transform> transforms) {
        this.transforms = transforms;
    }

    public List<Sink> getSinks() {
        return sinks;
    }

    public void setSinks(List<Sink> sinks) {
        this.sinks = sinks;
    }

    public String getRuntimeMode() {
        return runtimeMode;
    }

    public void setRuntimeMode(String runtimeMode) {
        this.runtimeMode = runtimeMode;
    }

    private boolean isNullOrEmpty(String str) {
        return str == null || str.trim().isEmpty();
    }

    public void validate() {
        if (isNullOrEmpty(name)) {
            throw new IllegalArgumentException("name cannot be null or empty");
        }
        if (isNullOrEmpty(specVersion)) {
            throw new IllegalArgumentException("specVersion cannot be null or empty");
        }
        if (isNullOrEmpty(parallelism)) {
            throw new IllegalArgumentException("parallelism cannot be null or empty");
        }
        if (isNullOrEmpty(runtimeMode)) {
            throw new IllegalArgumentException("runtimeMode cannot be null or empty");
        }
        if (sources == null || sources.isEmpty()) {
            throw new IllegalArgumentException("sources be null or empty");
        }
        if (transforms == null || transforms.isEmpty()) {
            throw new IllegalArgumentException("transforms cannot be null or empty");
        }
        if (sinks == null || sinks.isEmpty()) {
            throw new IllegalArgumentException("sinks cannot be null or empty");
        }
        sources.stream().forEach(e -> e.validate());
        transforms.stream().forEach(e -> e.validate());
        sinks.stream().forEach(e->{
            e.setRuntimeMode(getRuntimeMode());
            e.validate();
        });
    }
}
