package com.chainbase.udf;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.apache.flink.table.functions.ScalarFunction;
import org.apache.flink.types.Row;

import java.util.Set;

public class RowToJsonFunction extends ScalarFunction {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public String eval(Row row) {
        try {
            if (row == null) return null;
            ObjectNode jsonNode = MAPPER.createObjectNode();
            Set<String> fieldNames = row.getFieldNames(true);
            if (fieldNames == null) return null;

            String[] fieldNameStrings = fieldNames.toArray(new String[0]);

            for (int i = 0; i < fieldNameStrings.length; i++) {
                Object value = row.getField(i);
                addValueToJson(jsonNode, fieldNameStrings[i], value);
            }

            return MAPPER.writeValueAsString(jsonNode);
        } catch (Exception e) {
            throw new RuntimeException("Error converting ROW to JSON", e);
        }
    }

    private void addValueToJson(ObjectNode jsonNode, String fieldName, Object value) {
        if (value == null) {
            jsonNode.putNull(fieldName);
        } else if (value instanceof Row) {
            jsonNode.put(fieldName, eval((Row) value));
        } else if (value instanceof Object[]) {
            ArrayNode arrayNode = jsonNode.putArray(fieldName);
            for (Object item : (Object[]) value) {
                if (item instanceof Row) {
                    arrayNode.add(eval((Row) item));
                } else {
                    arrayNode.add(String.valueOf(item));
                }
            }
        } else {
            jsonNode.put(fieldName, String.valueOf(value));
        }
    }
}