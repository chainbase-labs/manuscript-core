package com.chainbase.udf.math;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import org.apache.flink.table.functions.AggregateFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class CountDistinct extends AggregateFunction<Long, CountDistinct.CountAccumulator> {

  private final static Logger LOGGER = LoggerFactory.getLogger(CountDistinct.class);

  @Override
  public CountAccumulator createAccumulator() {
    return new CountAccumulator();
  }

  @Override
  public Long getValue(CountAccumulator accumulator) {
    if (accumulator.distinctSet == null || accumulator.distinctSet.isEmpty()) {
      return 0L;
    }
    HashSet<String> set = new HashSet<>(accumulator.distinctSet);
    return (long) set.size();
  }

  public void accumulate(CountAccumulator accumulator, String elem) {
    if (elem != null) {
      accumulator.distinctSet.add(elem);
    }
  }

  public void retract(CountAccumulator accumulator, String elem) {
    accumulator.distinctSet.remove(elem);
  }


  public void merge(CountAccumulator acc, Iterable<CountAccumulator> it) {
    for (CountAccumulator a : it) {
      acc.distinctSet.addAll(a.distinctSet);
    }
  }

  public void resetAccumulator(CountAccumulator acc) {
    acc.distinctSet = new ArrayList<>();
  }

  public static class CountAccumulator {

    public List<String> distinctSet = new ArrayList<>();

    public List<String> getDistinctSet() {
      return distinctSet;
    }

    public void setDistinctSet(List<String> ds) {
      this.distinctSet = ds;
    }
  }
}