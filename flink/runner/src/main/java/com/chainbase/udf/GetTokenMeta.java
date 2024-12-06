package com.chainbase.udf;

import com.chainbase.common.cached.LRUCache;
import com.chainbase.evm.ContractDiscriminator;
import com.chainbase.evm.ContractDiscriminator.ContractInfo;
import com.chainbase.evm.ContractDiscriminator.TokenInfo;
import com.chainbase.evm.HexUtil;
import com.chainbase.evm.Web3jClient;
import com.chainbase.utils.JSONUtils;
import com.google.common.base.Stopwatch;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.concurrent.TimeUnit;
import javax.annotation.Nullable;
import org.apache.flink.table.functions.FunctionContext;
import org.apache.flink.table.functions.ScalarFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class GetTokenMeta extends ScalarFunction {

  private static final long serialVersionUID = 1607389111240208924L;
  private final static Logger LOGGER = LoggerFactory.getLogger(GetTokenMeta.class);
  private final Map<String, Web3jClient> clientMap = new HashMap<>(1);
  private LRUCache<String, String> cache;

  private Web3jClient getWeb3jClientInstance(String endpoint) {
    return clientMap.computeIfAbsent(endpoint, Web3jClient::new);
  }

  @Override
  public void open(FunctionContext context) throws Exception {
    super.open(context);
    LOGGER.debug("GetTokenMeta open invoke");
    this.cache = new LRUCache<>(32);
  }

  public String eval(String endpoint, String contractAddress, String bytecode) {
    String key = String.format("%s-%s-%s", endpoint, contractAddress, bytecode);

    return cache.computeIfAbsent(key, (String innerKey) -> {
      Stopwatch timer = Stopwatch.createStarted();
      String result = invoke(endpoint, contractAddress, bytecode);
      timer.stop();
      LOGGER.debug(String.format("GetTokenMeta(%s) %sms -> %s", contractAddress,
          timer.elapsed(TimeUnit.MILLISECONDS), result));
      return result;
    });
  }

  public String eval(String endpoint, String contractAddress, byte[] bytecode) {
    return eval(endpoint, contractAddress, HexUtil.bytes20xHex(bytecode));
  }

  public String invoke(String endpoint, String contractAddress, @Nullable String bytecode) {
    Web3jClient web3jClient = getWeb3jClientInstance(endpoint);

    if (Objects.isNull(contractAddress)) {
      return JSONUtils.toString(Collections.emptyMap());
    }
    contractAddress = contractAddress.toLowerCase();

    ContractDiscriminator contractDiscriminator = new ContractDiscriminator(web3jClient);
    ContractInfo contractInfo = contractDiscriminator.getContractInfo(contractAddress, bytecode);

    if (Objects.isNull(contractInfo)) {
      return JSONUtils.toString(Collections.emptyMap());
    }

    Map<String, Object> result = new HashMap<>(8);
    result.put("is_erc20", contractInfo.isErc20());
    result.put("is_erc721", contractInfo.isErc721());
    result.put("is_erc1155", contractInfo.isErc1155());
    result.put("is_proxy", contractInfo.isProxy());

    TokenInfo tokenInfo = contractDiscriminator.getTokenInfo(contractAddress, contractInfo);

    if (Objects.nonNull(tokenInfo)) {
      result.put("name", tokenInfo.getName());
      result.put("symbol", tokenInfo.getSymbol());
      result.put("decimals", tokenInfo.getDecimals());
      result.put("total_supply", tokenInfo.getTotalSupply());
    }
    return JSONUtils.toString(result);
  }
}
