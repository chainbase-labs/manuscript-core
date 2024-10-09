package com.chainbase.utils;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class JSONUtils {

  private final static ObjectMapper objectMapper = new ObjectMapper();

  public static ArrayNode createArrayNode() {
    return objectMapper.createArrayNode();
  }

  public static ObjectNode createObjectNode() {
    return objectMapper.createObjectNode();
  }

  public static String toString(Object obj) {
    try {
      return objectMapper.writeValueAsString(obj);
    } catch (JsonProcessingException e) {
      throw new RuntimeException(e);
    }
  }


  public static JsonNode toJSONObject(String jsonString) {
    try {
      return objectMapper.readTree(jsonString);
    } catch (JsonProcessingException e) {
      throw new RuntimeException(e);
    }
  }

  public static List<HashMap<String, Object>> toMap(String jsonString) {
    try {
      TypeReference<ArrayList<HashMap<String,Object>>> typeRef = new TypeReference<ArrayList<HashMap<String,Object>>>() {};
      return objectMapper.readValue(jsonString, typeRef);
    } catch (JsonProcessingException e) {
      throw new RuntimeException(e);
    }
  }

}
