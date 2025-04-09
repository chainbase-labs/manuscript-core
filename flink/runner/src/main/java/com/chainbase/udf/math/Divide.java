package com.chainbase.udf.math;

import java.math.BigDecimal;

import org.apache.flink.table.annotation.DataTypeHint;
import org.apache.flink.table.functions.ScalarFunction;

/**
 * Returns a BigInteger whose value is (number1 / number2).
 */
public class Divide extends ScalarFunction {

    public @DataTypeHint("STRING") String eval(@DataTypeHint("STRING") String number1, @DataTypeHint("STRING") String number2) {
        return new BigDecimal(number1).divide(new BigDecimal(number2)).toString();
    }

}
