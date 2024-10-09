package com.chainbase.evm;

import lombok.Getter;
import org.web3j.protocol.core.methods.response.EthCall;

@Getter
public class RPCCallException extends RuntimeException {

  private final EthCall ethCall;

  public RPCCallException(String message, EthCall ethCall) {
    super(message);
    this.ethCall = ethCall;
  }
}
