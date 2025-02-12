package com.chainbase.udf;
import com.chainbase.evm.Web3jClient;
import com.chainbase.common.cached.LRUCache;
import com.chainbase.utils.JSONUtils;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.math.BigInteger;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import org.apache.flink.table.api.DataTypes;
import org.apache.flink.table.catalog.DataTypeFactory;
import org.apache.flink.table.functions.FunctionContext;
import org.apache.flink.table.functions.ScalarFunction;
import org.apache.flink.table.types.inference.TypeInference;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.web3j.protocol.core.Response;

public class RpcRequest extends ScalarFunction {
    private final static Logger logger = LoggerFactory.getLogger(RpcRequest.class);
    private final static ObjectMapper objectMapper = new ObjectMapper();
    private static final long serialVersionUID = -4525805997990479092L;
    private final Map<String, Web3jClient> clientMap = new HashMap<>(1);
    private LRUCache<String, Object> cache;

    private static JsonNode parseABI(String abiJson) {
        return JSONUtils.toJSONObject(abiJson);
    }

    @Override
    public void open(FunctionContext context) throws Exception {
        super.open(context);
        logger.info("Rpc request open invoke");
        this.cache = new LRUCache<>(32);
    }

    private Web3jClient getWeb3jClientInstance(String endpoint) {
        return clientMap.computeIfAbsent(endpoint, Web3jClient::new);
    }

    private String toJSON(Object obj) {
        try {
            return objectMapper.writeValueAsString(obj);
        } catch (JsonProcessingException e) {
            return obj.toString();
        }
    }

    private String convertOutputType(Object value) {
        if (null == value) {
            return null;
        } else if (value instanceof BigInteger) {
            return value.toString();
        } else if (value instanceof Map) {
            try {
                return objectMapper.writeValueAsString(value).toString();
            } catch (JsonProcessingException e) {
                throw new RuntimeException(e);
            }
        }
        return value.toString();
    }

    /*Return type is Object is not allowed
    1. Modify Return type as String or Other RawTypes
    2. Override getTypeInference
     */
    public Object eval(String endpoint, String functionName, Object... params) {
        Web3jClient web3jClient = getWeb3jClientInstance(endpoint);
        try {
            Response<?> send = web3jClient.rpcRequest(functionName, params).send();
            Object result = send.getResult();
            logger.info(String.format("RPC Result: %s(%s) %s%n",
                    functionName, toJSON(params), result));
            return convertOutputType(result);

        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public TypeInference getTypeInference(DataTypeFactory typeFactory) {
        return TypeInference.newBuilder().outputTypeStrategy(callContext -> {
            return Optional.of(DataTypes.STRING());
        }).build();
    }
}