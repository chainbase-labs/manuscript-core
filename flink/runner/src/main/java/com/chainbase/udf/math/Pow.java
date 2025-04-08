package com.chainbase.udf.math;

import java.math.BigInteger;
import org.apache.flink.table.functions.ScalarFunction;

/**
 * Returns a BigInteger whose value is (this^exponent).
 */
public class Pow extends ScalarFunction {

  public String eval(String number1, String exponent) {
    return new BigInteger(number1).pow(Integer.parseInt(exponent)).toString();
  }

}
