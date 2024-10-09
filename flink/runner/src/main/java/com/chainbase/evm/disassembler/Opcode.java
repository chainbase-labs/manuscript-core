package com.chainbase.evm.disassembler;

import java.math.BigInteger;

public class Opcode {

  private int offset;

  private Opcodes opcode;

  private BigInteger parameter;

  public Opcode() {}

  public Opcode(int offset, Opcodes opcode, BigInteger parameter) {
    this.offset = offset;
    this.opcode = opcode;
    this.parameter = parameter;
  }

  public Opcode(Opcodes opcode, BigInteger parameter) {
    this.opcode = opcode;
    this.parameter = parameter;
  }

  public Opcodes getOpcode() {
    return opcode;
  }

  void setOpcode(Opcodes opcode) {
    this.opcode = opcode;
  }

  public BigInteger getParameter() {
    return parameter;
  }

  public String getHexParameter() {
    return String.format("%8s", this.parameter.toString(16)).replaceAll(" ", "0");
  }

  void setParameter(BigInteger parameter) {
    this.parameter = parameter;
  }

  public int getOffset() {
    return offset;
  }

  public void setOffset(int offset) {
    this.offset = offset;
  }

  @Override
  public String toString() {
    String toString = "0x" + String.format("%03X", this.offset) + " " + this.opcode.name();
    if (parameter != null) {
      toString += " 0x" + parameter.toString(16);
    }
    return toString;
  }
}
