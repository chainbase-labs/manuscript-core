package com.chainbase.udf;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class EthCallRequestTest {
  private final EthCallRequest rpcRequest;

  public EthCallRequestTest() throws Exception {
    this.rpcRequest = new EthCallRequest();
    this.rpcRequest.open(null);
  }

  @Test
  public void testBalanceOf() {
    String balanceOfABI = "{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}";

    Object result = rpcRequest.eval(
            "https://rpc.ankr.com/eth", balanceOfABI,
            "0xdAC17F958D2ee523a2206206994597C13D831ec7", 18111298L,
            "0x4d880E7d7D03122bB9dA9CBb5e408cd18E8D996d");
    Assertions.assertEquals("13644832905", result);
  }

}