package com.chainbase.evm.disassembler;

public class StringToCharIterator {

  private final String str;

  private int pos = 0;

  public StringToCharIterator(String str) {
    this.str = str;
  }

  public boolean hasNext() {
    return pos + 2 <= str.length();
  }

  public String next() {
    String substring = str.substring(pos, pos + 2);
    pos = pos + 2;
    return substring;
  }

  public void remove() {
    throw new UnsupportedOperationException();
  }
}
