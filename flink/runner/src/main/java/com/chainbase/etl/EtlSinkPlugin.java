package com.chainbase.etl;

import com.chainbase.etl.model.Sink;


public interface EtlSinkPlugin {

    String factoryIdentifier();

    void sink(Sink sink);
}
