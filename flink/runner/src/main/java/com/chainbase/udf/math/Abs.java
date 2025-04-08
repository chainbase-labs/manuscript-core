package com.chainbase.udf.math;

import java.math.BigInteger;
import org.apache.flink.table.functions.ScalarFunction;

/**
 * Returns a BigInteger whose value is the absolute value of this BigInteger.
 */
public class Abs extends ScalarFunction {

  public String eval(String number) {
    return new BigInteger(number).abs().toString();
  }

}
