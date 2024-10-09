package com.chainbase.udf;


import com.chainbase.evm.ABIDecoder;
import com.chainbase.evm.EvmUtil;
import com.chainbase.common.cached.LRUCache;
import com.chainbase.evm.HexUtil;
import com.chainbase.utils.JSONUtils;
import com.esaulpaugh.headlong.abi.ABIObject;
import com.esaulpaugh.headlong.abi.Event;
import com.esaulpaugh.headlong.abi.Tuple;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import org.apache.commons.lang3.StringUtils;
import org.apache.flink.table.functions.FunctionContext;
import org.apache.flink.table.functions.ScalarFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DecodeEvent extends ScalarFunction {

  private final static Logger logger = LoggerFactory.getLogger(DecodeEvent.class);
  private static final long serialVersionUID = -7436945467996032199L;

  private final Map<String, Event> eventMap = new HashMap<>(1);
  private LRUCache<String, String> cache;

  private static JsonNode parseABI(String abiJson) {
    return JSONUtils.toJSONObject(abiJson);
  }

  @Override
  public void open(FunctionContext context) throws Exception {
    super.open(context);
    this.cache = new LRUCache<>(32);
  }

  private String decodeEvent(Event event, String[] topicsHex, byte[] data) {
    String[] topics = Arrays.stream(topicsHex).filter(Objects::nonNull).filter(StringUtils::isNotBlank)
        .toArray(String[]::new);
    if (topics.length != event.getIndexedParams().size() + 1) {
      return null;
    }

    Tuple tuple = ABIDecoder.decodeEvent(topics, data, event);
    ObjectNode result = EvmUtil.transformTupleValue2ObjectNode(
        EvmUtil.reviseOnCopyTuple(event.getInputs(), false), tuple);
    return result.toString();
  }

  private String handle(String abi, String topic0, String topic1, String topic2, String topic3,
      byte[] data) {
    try {
      String[] topics = new String[]{topic0, topic1, topic2, topic3};
      Event abiEventObject = eventMap.computeIfAbsent(abi, ABIObject::fromJson);
      if (!abiEventObject.isEvent()) {
        throw new RuntimeException("Only decode events are supported !");
      }
      return decodeEvent(abiEventObject.asEvent(), topics, data);
    } catch (Exception e) {
      logger.debug("decoding failure, %s", e);
      return null;
    }
  }

  public String eval(String abi, String topic0, String topic1, String topic2, String topic3,
      byte[] data) {
    String key = String.join("-",
        Arrays.asList(abi, topic0, topic1, topic2, topic3, Arrays.toString(data)));
    return cache.computeIfAbsent(key,
        (innerKey) -> handle(abi, topic0, topic1, topic2, topic3, data));
  }

  public String eval(String abi, String topic0, String topic1, String topic2, String topic3,
      String data) {
    return eval(abi, topic0, topic1, topic2, topic3, HexUtil.decodeHex(data));
  }
}
