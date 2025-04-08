package com.chainbase.udf.math;

import java.math.BigInteger;
import org.apache.flink.table.functions.ScalarFunction;

/**
 * Returns a BigInteger whose value is (-this).
 */
public class Negate extends ScalarFunction {

  public String eval(String number1) {
    return new BigInteger(number1).negate().toString();
  }

}
