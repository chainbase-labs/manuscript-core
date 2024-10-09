package com.chainbase.udf;

import com.chainbase.evm.EvmUtil;
import com.chainbase.evm.HexUtil;
import com.esaulpaugh.headlong.abi.ABIObject;
import com.esaulpaugh.headlong.abi.Function;
import com.esaulpaugh.headlong.abi.Tuple;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.ByteBuffer;
import org.apache.flink.table.functions.ScalarFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DecodeFunction extends ScalarFunction {

  private final static Logger logger = LoggerFactory.getLogger(DecodeFunction.class);
  private static final long serialVersionUID = 2004249297425923280L;


  private Tuple decode(java.util.function.Function<ByteBuffer, Tuple> func, byte[] hex) {
    ByteBuffer buffer = ByteBuffer.wrap(hex);
    if (buffer.capacity() <= 0) {
      return Tuple.EMPTY;
    }
    return func.apply(buffer);
  }

  private String decodeFunction(Function function, byte[] input, byte[] output) {

    Tuple inputParam = decode(function::decodeCall, input);
    Tuple resultParam = decode(function::decodeReturn, output);

    ObjectNode objectNode =
        EvmUtil.transformTupleValue2ObjectNode(
            EvmUtil.reviseOnCopyTuple(function.getInputs(), false), inputParam);
    ObjectNode outputObjectNode =
        EvmUtil.transformTupleValue2ObjectNode(
            EvmUtil.reviseOnCopyTuple(function.getOutputs(), true), resultParam);
    return objectNode.setAll(outputObjectNode).toString();
  }

  public String eval(String abi, byte[] input, byte[] output) {
    try {
      ABIObject abiFunctionObject = ABIObject.fromJson(abi);
      if (!abiFunctionObject.isFunction()) {
        throw new RuntimeException("Only decode functions are supported !");
      }
      return decodeFunction(abiFunctionObject.asFunction(), input, output);
    } catch (Exception e) {
      logger.debug("decoding failure, %s", e);
      return null;
    }
  }

  public String eval(String abi, String input, String output) {
    return eval(abi, HexUtil.decodeHex(input), HexUtil.decodeHex(output));
  }
}