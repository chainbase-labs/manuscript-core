package com.chainbase.etl;

import com.chainbase.etl.model.Source;
import com.chainbase.etl.model.Transform;

import java.util.List;

public interface EtlSourcePlugin {

    String factoryIdentifier();

    void source(List<Source> sources);

    void transform(List<Transform> transforms);
}
