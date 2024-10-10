package com.chainbase.evm.response;

import java.util.List;
import java.util.Map;
import lombok.Data;
import org.web3j.protocol.core.Response;

public class TraceBlockResponse extends Response<List<TraceBlockResponse.TraceBlockItem>> {

  @Data
  public static class TraceBlockStatus {

    private String balance;
    private String nonce;
    private String code;
    private Map<String, String> storage;
  }

  @Data
  public static class TraceBlockResult {

    private Map<String, TraceBlockStatus> post;
    private Map<String, TraceBlockStatus> pre;
  }

  @Data
  public static class TraceBlockItem {
    private TraceBlockResult result;
  }
}
