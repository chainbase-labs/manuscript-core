package com.chainbase.udf;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.flink.table.api.DataTypes;
import org.apache.flink.table.catalog.DataTypeFactory;
import org.apache.flink.table.functions.FunctionContext;
import org.apache.flink.table.functions.ScalarFunction;
import org.apache.flink.table.types.inference.InputTypeStrategies;
import org.apache.flink.table.types.inference.TypeInference;
import org.apache.flink.table.types.inference.TypeStrategies;
import org.apache.flink.types.Row;

import java.util.ArrayList;

public class ArrayToJsonFunction extends ScalarFunction {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final RowToJsonFunction rowConverter = new RowToJsonFunction();

    @Override
    public void open(FunctionContext context) throws Exception {
        super.open(context);
        rowConverter.open(context);
    }

    public <T> String eval(T[] array) {
        try {
            if (array == null) return null;
            ArrayList<Object> arrayNode = new ArrayList<>(array.length);
            for (T item : array) {
                if (item instanceof Row) {
                    String itemJson = rowConverter.eval((Row) item);
                    arrayNode.add(itemJson);
                } else {
                    arrayNode.add(item);
                }
            }

            return MAPPER.writeValueAsString(arrayNode);
        } catch (Exception e) {
            throw new RuntimeException("Error converting Array to JSON", e);
        }
    }

    @Override
    public void close() throws Exception {
        rowConverter.close();
        super.close();
    }

    @Override
    public TypeInference getTypeInference(DataTypeFactory typeFactory) {
        return TypeInference.newBuilder()
                .inputTypeStrategy(InputTypeStrategies.sequence(InputTypeStrategies.ANY))
                .outputTypeStrategy(TypeStrategies.explicit(DataTypes.STRING()))
                .build();
    }
}