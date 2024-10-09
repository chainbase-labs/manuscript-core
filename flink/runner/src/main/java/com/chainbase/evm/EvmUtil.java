package com.chainbase.evm;

import com.esaulpaugh.headlong.abi.ABIType;
import com.esaulpaugh.headlong.abi.Address;
import com.esaulpaugh.headlong.abi.ArrayType;
import com.esaulpaugh.headlong.abi.Event;
import com.esaulpaugh.headlong.abi.Tuple;
import com.esaulpaugh.headlong.abi.TupleType;
import com.esaulpaugh.headlong.util.FastHex;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.BigIntegerNode;
import com.fasterxml.jackson.databind.node.BooleanNode;
import com.fasterxml.jackson.databind.node.LongNode;
import com.fasterxml.jackson.databind.node.NullNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.google.common.base.Strings;
import com.joemelsha.crypto.hash.Keccak;
import java.lang.reflect.Constructor;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import javax.annotation.Nullable;

public final class EvmUtil {

  private static final ObjectMapper MAPPER = new ObjectMapper();

  private static final String EMPTY_TUPLE_STRING = "()";

  private static final List<String> BLOCK_HOLE_ADDRESS_LIST = Arrays.asList(
      HexUtil.DEFAULT_HOLE_ADDRESS, "0", "0x", "0x0");

  public static TupleType reviseOnCopyTuple(TupleType tupleType, boolean isOutput) {
    String[] elementNames = getElementNames(tupleType);
    return updateOnCopyTuple(
        tupleType,
        null,
        null,
        null,
        IntStream.range(0, elementNames.length)
            .mapToObj(
                idx -> reviseTupleTypeName(idx, elementNames[idx], isOutput ? "output" : null))
            .collect(Collectors.toList())
            .toArray(new String[elementNames.length])
    );
  }

  public static TupleType updateOnCopyTuple(
      TupleType tupleType,
      @Nullable String canonicalType,
      @Nullable Boolean dynamic,
      @Nullable ABIType<?>[] elementTypes,
      @Nullable String[] elementNames) {
    try {
      Class<?> clazz = Class.forName("com.esaulpaugh.headlong.abi.TupleType");
      Constructor<?> constructor = clazz.getDeclaredConstructors()[0];
      constructor.setAccessible(true);

      return (TupleType)
          constructor.newInstance(
              canonicalType == null ? tupleType.getCanonicalType() : canonicalType,
              dynamic == null ? tupleType.isDynamic() : dynamic,
              elementTypes == null ? getElementTypes(tupleType) : elementTypes,
              elementNames == null ? getElementNames(tupleType) : elementNames);
    } catch (Exception ex) {
      throw new RuntimeException("Failed to update-on-copy tuple", ex);
    }
  }

  private static String reviseTupleTypeName(
      Integer idx, @Nullable String name, @Nullable String prefix) {
    String revisedName;

    if (Strings.isNullOrEmpty(name)) {
      revisedName = String.format("_%d", idx);
    } else {
      revisedName = name;
    }

    if (prefix != null) {
      revisedName = String.format("%s_%s", prefix, revisedName);
    }

    return revisedName;
  }

  private static ABIType<?>[] getElementTypes(TupleType tupleType) {
    return IntStream.range(0, tupleType.size())
        .mapToObj(idx -> (ABIType<?>) tupleType.get(idx))
        .collect(Collectors.toList())
        .toArray(new ABIType<?>[tupleType.size()]);
  }

  private static String[] getElementNames(TupleType tupleType) {
    return IntStream.range(0, tupleType.size())
        .mapToObj(tupleType::getElementName)
        .collect(Collectors.toList())
        .toArray(new String[tupleType.size()]);
  }

  public static String getCanonicalSignatureWithIndexSign(Event event) {
    StringBuilder canonicalBuilder = new StringBuilder("(");
    for (int idx = 0; idx < event.getInputs().size(); idx++) {
      TupleType inputType = event.getInputs().get(idx);
      if (event.isElementIndexed(idx)) {
        canonicalBuilder.append("indexed").append(" ");
      }
      canonicalBuilder.append(inputType.getCanonicalType()).append(",");
    }

    String typeCanonical =
        canonicalBuilder.length() != 1
            ? canonicalBuilder.deleteCharAt(canonicalBuilder.length() - 1).append(")").toString()
            : EMPTY_TUPLE_STRING;

    return event.getName() + typeCanonical;
  }

  public static String providesEventTopic0(String canonicalSignature) {
    return providesSelectorHex(canonicalSignature, null);
  }

  public static String providesFunctionSelector(String canonicalSignature) {
    return providesSelectorHex(canonicalSignature, 8);
  }

  private static String providesSelectorHex(String signature, @Nullable Integer selectorLen) {
    byte[] decodedBytes =
        com.esaulpaugh.headlong.util.Strings.decode(
            signature, com.esaulpaugh.headlong.util.Strings.ASCII);
    String hexString = FastHex.encodeToString(new Keccak(256).digest(decodedBytes));

    if (selectorLen != null) {
      hexString = hexString.substring(0, selectorLen);
    }
    return String.format("0x%s", hexString);
  }

  public static JsonNode transformAtomicValue2ObjectNode(ABIType<?> atype, Object value) {
    if (!value.getClass().equals(atype.clazz())) {
      throw new IllegalArgumentException(
          String.format(
              "The class of %s is not equals the internal class in the %s", value, atype));
    }

    if (value instanceof Byte) {
      return BigIntegerNode.valueOf(BigInteger.valueOf((Byte) value));
    } else if (value instanceof Integer) {
      return BigIntegerNode.valueOf(BigInteger.valueOf((Integer) value));
    } else if (value instanceof Long) {
      return LongNode.valueOf((Long) value);
    } else if (value instanceof BigInteger) {
      return TextNode.valueOf(value.toString());
    } else if (value instanceof BigDecimal) {
      return TextNode.valueOf(value.toString());
    } else if (value instanceof Address) {
      return TextNode.valueOf(value.toString().toLowerCase());
    } else if (value instanceof Boolean) {
      return BooleanNode.valueOf((Boolean) value);
    } else {
      throw new IllegalArgumentException(
          String.format("Can't transform %s(%s) to json value", value, atype));
    }
  }

  public static JsonNode transformArrayValue2ObjectNode(
      ArrayType<ABIType<?>, ?> atype, Object value) {
    return transformArrayValue2ObjectNode(atype, value, false);
  }

  public static JsonNode transformArrayValue2ObjectNode(
      ArrayType<ABIType<?>, ?> atype, Object value, boolean tuple2Array) {
    List<Integer> abiTypes =
        Arrays.asList(
            ABIType.TYPE_CODE_INT,
            ABIType.TYPE_CODE_LONG,
            ABIType.TYPE_CODE_BIG_INTEGER,
            ABIType.TYPE_CODE_BIG_DECIMAL,
            ABIType.TYPE_CODE_ADDRESS,
            ABIType.TYPE_CODE_BOOLEAN);

    int typeCode = atype.getElementType().typeCode();
    String canonicalType = atype.getCanonicalType();

    if (value.getClass().isArray() && abiTypes.contains(typeCode)) {
      ArrayNode arrayNode = MAPPER.createArrayNode();

      switch (typeCode) {
        case ABIType.TYPE_CODE_INT:
          for (int item : (int[]) value) {
            arrayNode.add(transformAtomicValue2ObjectNode(atype.getElementType(), item));
          }
          break;

        case ABIType.TYPE_CODE_LONG:
          for (long item : (long[]) value) {
            arrayNode.add(transformAtomicValue2ObjectNode(atype.getElementType(), item));
          }
          break;

        case ABIType.TYPE_CODE_BOOLEAN:
          for (boolean item : (boolean[]) value) {
            arrayNode.add(transformAtomicValue2ObjectNode(atype.getElementType(), item));
          }
          break;

        case ABIType.TYPE_CODE_BIG_INTEGER:
        case ABIType.TYPE_CODE_BIG_DECIMAL:
        case ABIType.TYPE_CODE_ADDRESS:
          for (Object item : (Object[]) value) {
            arrayNode.add(transformAtomicValue2ObjectNode(atype.getElementType(), item));
          }
          break;
      }

      return arrayNode;
    }

    if (value instanceof String
        && typeCode == ABIType.TYPE_CODE_BYTE
        && canonicalType.equals("string")) {
      return TextNode.valueOf((String) value);
    }

    if (value instanceof byte[]
        && typeCode == ABIType.TYPE_CODE_BYTE
        && canonicalType.equals("string")) {
      return TextNode.valueOf(new String((byte[]) value, StandardCharsets.UTF_8));
    }

    if (value instanceof byte[]
        && typeCode == ABIType.TYPE_CODE_BYTE
        && canonicalType.equals("function")) {
      return TextNode.valueOf(HexUtil.bytes20xHex((byte[]) value));
    }

    if (value instanceof byte[]
        && typeCode == ABIType.TYPE_CODE_BYTE
        && canonicalType.matches("bytes.*")) {
      return TextNode.valueOf(HexUtil.bytes20xHex((byte[]) value));
    }

    if (value.getClass().isArray() && typeCode == ABIType.TYPE_CODE_ARRAY) {
      ArrayNode arrayNode = MAPPER.createArrayNode();
      Object[] arrayValue = (Object[]) value;

      Arrays.stream(arrayValue)
          .map(
              item ->
                  transformArrayValue2ObjectNode(
                      (ArrayType<ABIType<?>, ?>) atype.getElementType(), item))
          .forEach(arrayNode::add);

      return arrayNode;
    }

    if (value instanceof Tuple[] && typeCode == ABIType.TYPE_CODE_TUPLE) {
      ArrayNode arrayNode = MAPPER.createArrayNode();
      Tuple[] arrayValue = (Tuple[]) value;

      Arrays.stream(arrayValue)
          .map(
              item -> {
                if (tuple2Array) {
                  return transformTupleValue2ArrayNode(atype.getElementType(), item);
                }
                return transformTupleValue2ObjectNode((TupleType) atype.getElementType(), item);
              })
          .forEach(arrayNode::add);

      return arrayNode;
    }

    throw new IllegalArgumentException(
        String.format("Can't transform %s(%s) to json value", value, atype));
  }

  public static JsonNode transformTupleValue2ArrayNode(
      ABIType<?> elementType, Object elementValue) {

    switch (elementType.typeCode()) {
      case ABIType.TYPE_CODE_TUPLE:
        ArrayNode arrayNode = MAPPER.createArrayNode();
        TupleType tupleTypes = (TupleType) elementType;
        Tuple tupleElementValue = (Tuple) elementValue;
        IntStream.range(0, tupleTypes.size())
            .mapToObj(
                index ->
                    transformAtomicValue2ObjectNode(
                        tupleTypes.get(index), tupleElementValue.get(index)))
            .forEach(arrayNode::add);
        return arrayNode;

      case ABIType.TYPE_CODE_ARRAY:
        return transformArrayValue2ObjectNode(
            (ArrayType<ABIType<?>, ?>) elementType, elementValue, true);

      default:
        return transformAtomicValue2ObjectNode(elementType, elementValue);
    }
  }

  public static ObjectNode transformTupleValue2ObjectNode(TupleType atype, Tuple tuple) {
    ObjectNode objectNode = MAPPER.createObjectNode();

    for (int idx = 0; idx < atype.size(); idx++) {
      String name = atype.getElementName(idx);
      ABIType<?> elementType = atype.get(idx);

      // If the input in the trace is null or the output in trace is null,
      // we will return an empty Tuple
      if (tuple.size() <= idx) {
        objectNode.set(name, NullNode.instance);
      } else {
        Object elementValue = tuple.get(idx);

        JsonNode elementObjectValue;
        switch (elementType.typeCode()) {
          case ABIType.TYPE_CODE_TUPLE:
            elementObjectValue =
                transformTupleValue2ObjectNode((TupleType) elementType, (Tuple) elementValue);
            break;

          case ABIType.TYPE_CODE_ARRAY:
            elementObjectValue =
                transformArrayValue2ObjectNode(
                    (ArrayType<ABIType<?>, ?>) elementType, elementValue);
            break;

          default:
            elementObjectValue = transformAtomicValue2ObjectNode(elementType, elementValue);
        }

        objectNode.set(name, elementObjectValue);
      }
    }

    return objectNode;
  }

  public static boolean isBlockHoleAddress(String address) {
    return BLOCK_HOLE_ADDRESS_LIST.contains(address);
  }
}
