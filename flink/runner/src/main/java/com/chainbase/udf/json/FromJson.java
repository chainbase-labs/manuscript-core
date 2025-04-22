package com.chainbase.udf.json;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.flink.table.api.DataTypes;
import org.apache.flink.table.catalog.DataTypeFactory;
import org.apache.flink.table.functions.ScalarFunction;
import org.apache.flink.table.types.DataType;
import org.apache.flink.table.types.inference.TypeInference;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nonnull;
import java.util.AbstractMap;
import java.util.Map;
import java.util.Optional;
import java.util.function.IntFunction;
import java.util.stream.IntStream;

public class FromJson extends ScalarFunction {
    private static final Logger logger = LoggerFactory.getLogger(FromJson.class);

    private static final ObjectMapper objectMapper = new ObjectMapper();

    public static Map.Entry<Type, String> getNextType(String type) {
        int start = type.indexOf("<");
        if (start != -1) {
            int end = type.lastIndexOf(">");
            String nextType = type.substring(start + 1, end);

            return new AbstractMap.SimpleEntry<>(Type.getType(type.substring(0, start)), nextType);
        }

        return new AbstractMap.SimpleEntry<>(Type.getType(type), "");
    }

    private Object parseType(JsonNode node, String type) {
        Map.Entry<Type, String> pair = getNextType(type);
        switch (pair.getKey()) {
            case Array:
                Map.Entry<Type, String> itemType = getNextType(pair.getValue());

                IntFunction<Object[]> typeGenerator = itemType.getKey().getGenerator();
                return IntStream.range(0, node.size())
                        .mapToObj(index -> parseType(node.get(index), pair.getValue())).toArray(typeGenerator);
            case String:
                if (node.isTextual()) {
                    return node.asText();
                }

                return node.toString();
            default:
                return null;
        }
    }

    public Object eval(String jsonString, String type) {
        try {
            JsonNode json = objectMapper.readTree(jsonString);
            return parseType(json, type.trim().toLowerCase());
        } catch (JsonProcessingException e) {
            logger.error(String.format("json deserialization [%s] failed", jsonString));
            return null;
        } catch (Exception ex) {
            logger.error(String.format("Type conversion to %s failed ", type));
            return null;
        }
    }

    protected DataType getReturnType(String type) {
        Map.Entry<Type, String> pair = getNextType(type);
        switch (pair.getKey()) {
            case String:
                return DataTypes.STRING();
            case Array:
                return DataTypes.ARRAY(getReturnType(pair.getValue()));
            default:
                return null;
        }
    }

    @Override
    public TypeInference getTypeInference(DataTypeFactory typeFactory) {
        return TypeInference.newBuilder().typedArguments(DataTypes.STRING(), DataTypes.STRING())
                .outputTypeStrategy(callContext -> {
                    if (!callContext.isArgumentLiteral(1) || callContext.isArgumentNull(1)) {
                        throw callContext.newValidationError("The type parameter cannot be null");
                    }

                    String type = callContext.getArgumentValue(1, String.class).orElse(null);
                    return Optional.ofNullable(getReturnType(type));
                }).build();
    }

    public enum Type {
        Array, String;


        public static @Nonnull Type getType(String typeStr) {
            for (Type item : Type.values()) {
                if (item.name().equalsIgnoreCase(typeStr)) {
                    return item;
                }
            }
            throw new RuntimeException(java.lang.String.format("Unsupported types %s", typeStr));
        }

        public <A> IntFunction<A[]> getGenerator() {
            switch (this) {
                case String:
                    return (size) -> (A[]) new String[size];
                case Array:
                    return (size) -> (A[]) new Object[size];
                default:
                    return (size) -> (A[]) new Object[size];
            }
        }
    }
}
