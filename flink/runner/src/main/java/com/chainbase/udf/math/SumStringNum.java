package com.chainbase.udf.math;

import com.chainbase.udf.math.SumStringNum.SumAccumulator;
import java.math.BigInteger;
import org.apache.flink.table.functions.AggregateFunction;

public class SumStringNum extends AggregateFunction<String, SumAccumulator> {
    private static final long serialVersionUID = -624853464825817945L;

    public static class SumAccumulator {
        public String sum = "0";

        public String getSum() {
            return sum;
        }

        public void setSum(String sum) {
            this.sum = sum;
        }
    }

    @Override
    public String getValue(SumAccumulator accumulator) {
        return accumulator.sum;
    }

    @Override
    public SumAccumulator createAccumulator() {
        return new SumAccumulator();
    }

    public void accumulate(SumAccumulator accumulator, String elem) {
        accumulator.sum = new BigInteger(accumulator.sum).add(new BigInteger(elem)).toString();
    }

    public void retract(SumAccumulator accumulator, String elem) {
        accumulator.sum = new BigInteger(accumulator.sum).subtract(new BigInteger(elem)).toString();
    }

    public void merge(SumAccumulator acc, Iterable<SumAccumulator> it) {
        for (SumAccumulator a : it) {
            acc.sum = new BigInteger(acc.sum).add(new BigInteger(a.sum)).toString();
        }
    }

    public void resetAccumulator(SumAccumulator acc) {
        acc.sum = "0";
    }
}