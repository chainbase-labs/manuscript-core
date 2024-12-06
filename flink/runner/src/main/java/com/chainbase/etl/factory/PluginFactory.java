package com.chainbase.etl.factory;

import com.chainbase.etl.EtlSinkPlugin;
import com.chainbase.etl.sink.FileSystemSinkPlugin;
import com.chainbase.etl.sink.PostgresSinkPlugin;
import com.chainbase.etl.sink.PrintSinkPlugin;
import com.chainbase.etl.sink.StarrocksSinkPlugin;
import org.apache.flink.table.api.bridge.java.StreamTableEnvironment;

import java.util.HashMap;
import java.util.Map;

public class PluginFactory {

    private static final Map<String, Class<? extends EtlSinkPlugin>> registry = new HashMap<>();

    static {
        registry.put(FileSystemSinkPlugin.IDENTIFIER, FileSystemSinkPlugin.class);
        registry.put(PrintSinkPlugin.IDENTIFIER, PrintSinkPlugin.class);
        registry.put(StarrocksSinkPlugin.IDENTIFIER, StarrocksSinkPlugin.class);
        registry.put(PostgresSinkPlugin.IDENTIFIER, PostgresSinkPlugin.class);
    }

    public static EtlSinkPlugin getImplementation(String identifier, StreamTableEnvironment tEnv) throws Exception {
        Class<? extends EtlSinkPlugin> implClass = registry.get(identifier);
        if (implClass != null) {
            return implClass.getDeclaredConstructor(StreamTableEnvironment.class).newInstance(tEnv);
        }
        throw new IllegalArgumentException("Unsupported sink type:: " + identifier);
    }
}
