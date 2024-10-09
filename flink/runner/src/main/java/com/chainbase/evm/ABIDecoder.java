package com.chainbase.evm;

import com.esaulpaugh.headlong.abi.Event;
import com.esaulpaugh.headlong.abi.Tuple;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.Arrays;
import java.util.Objects;
import java.util.stream.Collectors;

public class ABIDecoder {

  private static final ObjectMapper mapper = new ObjectMapper();


  public static Tuple decodeEvent(String[] topicsHex, String dataHex, Event event) {
    return decodeEvent(topicsHex, HexUtil.decodeHex(dataHex), event);
  }

  public static Tuple decodeEvent(String[] topicsHex, byte[] data, Event event) {
    byte[][] topics = Arrays.stream(topicsHex).filter(Objects::nonNull).map(HexUtil::decodeHex)
        .collect(Collectors.toList()).toArray(new byte[topicsHex.length][]);

    return event.decodeArgs(topics, data);
  }
}
