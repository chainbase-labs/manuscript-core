package com.chainbase.udf;

import com.chainbase.common.cached.LRUCache;
import com.chainbase.evm.Web3jClient;
import com.chainbase.evm.Web3jClient.MultiCallParameter;
import com.chainbase.utils.JSONUtils;
import com.esaulpaugh.headlong.abi.Tuple;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.util.RawValue;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import org.apache.commons.lang3.StringUtils;
import org.apache.flink.table.api.DataTypes;
import org.apache.flink.table.catalog.DataTypeFactory;
import org.apache.flink.table.functions.FunctionContext;
import org.apache.flink.table.functions.ScalarFunction;
import org.apache.flink.table.types.DataType;
import org.apache.flink.table.types.inference.TypeInference;
import org.bouncycastle.util.encoders.Hex;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Type;
import org.web3j.abi.datatypes.generated.Bytes32;
import org.web3j.abi.datatypes.generated.Int128;
import org.web3j.abi.datatypes.generated.Uint256;
import org.web3j.protocol.core.DefaultBlockParameter;
import org.web3j.protocol.core.DefaultBlockParameterName;
import org.web3j.protocol.core.DefaultBlockParameterNumber;

public class EthCallRequest extends ScalarFunction {

  private static final long serialVersionUID = 4556284504115560397L;
  private final static Logger logger = LoggerFactory.getLogger(EthCallRequest.class);
  private final static ObjectMapper objectMapper = new ObjectMapper();
  public static java.util.function.Function<DefaultBlockParameter, String> outputBlockNumber = (block) -> {
    if (block instanceof DefaultBlockParameterNumber) {
      return ((DefaultBlockParameterNumber) block).getBlockNumber().toString();
    } else {
      return block.getValue();
    }
  };
  private final Map<String, Web3jClient> clientMap = new HashMap<>(1);
  private LRUCache<String, Object> cache;

  private static JsonNode parseABI(String abiJson) {
    return JSONUtils.toJSONObject(abiJson);
  }

  private static String formatParams(JsonNode params) {
    if (params.size() <= 0) {
      return null;
    }
    return IntStream.range(0, params.size())
        .mapToObj(index -> params.get(index).get("type").asText()).collect(Collectors.joining(","));
  }

  private static Type convertType(String value, String type) {
    switch (type) {
      case "address":
        return new Address(value);
      case "uint256":
        return new Uint256(new BigInteger(value));
      case "int128":
        return new Int128(new BigInteger(value));
      case "bytes32":
        return new Bytes32(Hex.decode(value.substring(2).getBytes()));
      default:
        return null;
    }
  }

  public static MultiCallParameter buildMultiCallParameter(
      JsonNode abiJsonNode, String contractAddress, String... params
  ) {

    JsonNode stateMutability = abiJsonNode.get("stateMutability");
    if (stateMutability == null || !"view".equals(stateMutability.asText())) {
      throw new RuntimeException("Functions that only support stateMutability is view");
    }

    String functionName = abiJsonNode.get("name").asText();
    String inputParamString = formatParams(abiJsonNode.get("inputs"));
    String outputParamString = formatParams(abiJsonNode.get("outputs"));

    JsonNode inputParams = abiJsonNode.get("inputs");
    List<Type> paramList = IntStream.range(0, inputParams.size()).mapToObj(index -> {
      String type = inputParams.get(index).get("type").asText();
      String value = params[index];
      return convertType(value, type);
    }).collect(Collectors.toList());

    return new MultiCallParameter(contractAddress, paramList, functionName,
        String.format("%s(%s)", functionName, inputParamString != null ? inputParamString : ""),
        outputParamString != null ? String.format("(%s)", outputParamString) : null, null);
  }

  @Override
  public void open(FunctionContext context) throws Exception {
    super.open(context);
    this.cache = new LRUCache<>(32);
  }

  private Web3jClient getWeb3jClientInstance(String endpoint) {
    return clientMap.computeIfAbsent(endpoint,
        (_endpoint) -> new Web3jClient(_endpoint, 1000, 1000, false));
  }

  private String toJSON(Object obj) {
    try {
      return objectMapper.writeValueAsString(obj);
    } catch (JsonProcessingException e) {
      return obj.toString();
    }
  }

  public Object eval(String endpoint, String abiJson, String contractAddress,
      DefaultBlockParameter blockNumber, String... params) {
    String key = String.join("-",
        Arrays.asList(endpoint, abiJson, contractAddress, blockNumber.getValue(),
            Arrays.toString(params)));
    return cache.computeIfAbsent(key,
        (innerKey) -> invoke(endpoint, abiJson, contractAddress, blockNumber, params));
  }

  public Object invoke(String endpoint, String abiJson, String contractAddress,
      DefaultBlockParameter blockNumber, String... params) {
    try {
      JsonNode abiJsonNode = parseABI(abiJson);
      String functionName = abiJsonNode.get("name").asText();
      MultiCallParameter requestParam = buildMultiCallParameter(abiJsonNode, contractAddress,
          params);

      Web3jClient web3jClient = getWeb3jClientInstance(endpoint);
      List<Object> result = web3jClient.batchCall(Arrays.asList(requestParam), blockNumber);
      logger.debug(String.format("RPC Result: %s(%s) ^%s^ %s%n",
          functionName, toJSON(params), outputBlockNumber.apply(blockNumber), result));
      if (result.isEmpty()) {
        return null;
      }
      return convertOutputType(abiJsonNode,
          result.stream().filter(Objects::nonNull).findFirst().orElse(null));
    } catch (RuntimeException ex) {
      throw ex;
    } catch (Exception ex) {
      logger.debug(String.format("Call eth_call failed, %s, params: %s", ex,
          toJSON(Arrays.asList(abiJson, contractAddress, blockNumber, params))));
      return null;
    }
  }

  private Object convertOutputType(JsonNode abiJsonNode, Object value) {
    if (null == value) {
      return null;
    } else if (value instanceof BigInteger) {
      return value.toString();
    } else if (value instanceof Tuple) {
      JsonNode outputs = abiJsonNode.get("outputs");
      if (outputs.size() > 1) {
        ObjectNode objectNode = objectMapper.createObjectNode();

        IntStream.range(0, outputs.size()).forEach(index -> {
          String name = outputs.get(index).get("name").asText();
          String itemName = StringUtils.isBlank(name) ? String.format("_%s", index) : name;
          Object itemValue = ((Tuple) value).get(index);
          if (itemValue.getClass().isArray()) {
            ArrayNode node = objectNode.putArray(itemName);
            for (Object o : (Object[]) itemValue) {
              node.add(o.toString());
            }

          } else {
            objectNode.putRawValue(itemName, new RawValue(itemValue.toString()));
          }
        });
        return objectNode.toString();
      }
      return value;
    } else if (value instanceof com.esaulpaugh.headlong.abi.Address[]) {
      return Arrays.stream((com.esaulpaugh.headlong.abi.Address[]) value)
          .map(com.esaulpaugh.headlong.abi.Address::toString).toArray(String[]::new);
    }
    return value.toString();
  }


  private DataType getDataTypeByEvmRawType(String type) {
    switch (type) {
      case "bool":
        return DataTypes.BOOLEAN();
      case "uint256":
      case "string":
      default:
        return DataTypes.STRING();
    }
  }

  @Override
  public TypeInference getTypeInference(DataTypeFactory typeFactory) {
    int abiParamIndex = 1;

    return TypeInference.newBuilder().outputTypeStrategy(callContext -> {
      if (!callContext.isArgumentLiteral(abiParamIndex) || callContext.isArgumentNull(
          abiParamIndex)) {
        throw callContext.newValidationError("The ABI parameter cannot be empty.");
      }

      String abiJSON = callContext.getArgumentValue(abiParamIndex, String.class).orElse(null);
      JsonNode abiEvent = parseABI(abiJSON);
      JsonNode outputs = abiEvent.get("outputs");
      if (outputs.size() <= 0) {
        return Optional.empty();
      } else if (outputs.size() == 1) {
        DataType valueType = getDataTypeByEvmRawType(outputs.get(0).get("type").asText());
        return Optional.of(valueType);
      } else if (outputs.size() > 1) {
        return Optional.of(DataTypes.STRING());
      }
      throw new RuntimeException("Not yet implemented return complex type!");
    }).build();
  }

  public Object eval(String endpoint, String abiJson, String contractAddress, Long blockNumber,
      String... params) {
    return this.eval(endpoint, abiJson, contractAddress,
        DefaultBlockParameter.valueOf(BigInteger.valueOf(blockNumber)), params);
  }

  public Object eval(String endpoint, String abiJson, String contractAddress, String... params) {
    return eval(endpoint, abiJson, contractAddress, DefaultBlockParameterName.LATEST, params);
  }
}
