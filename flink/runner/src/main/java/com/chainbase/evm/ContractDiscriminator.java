package com.chainbase.evm;

import com.chainbase.evm.Web3jClient.MultiCallParameter;
import com.chainbase.evm.disassembler.Disassembler;
import com.chainbase.evm.disassembler.Opcode;
import com.chainbase.evm.disassembler.Opcodes;
import com.esaulpaugh.headlong.abi.Address;
import com.esaulpaugh.headlong.abi.Function;
import com.google.common.annotations.VisibleForTesting;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.ImmutableMap;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import javax.annotation.Nullable;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.ToString;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.web3j.protocol.core.DefaultBlockParameter;
import org.web3j.protocol.core.DefaultBlockParameterName;
import org.web3j.utils.Strings;
import lombok.var;


public class ContractDiscriminator {

  public static final int DEFAULT_TOKEN_META_MULTICALL_SIZE = 50;

  private static final Logger LOGGER = LoggerFactory.getLogger(ContractDiscriminator.class);
  private static final DefaultBlockParameter LATEST_BLOCK_NUMBER = DefaultBlockParameterName.LATEST;
  private static final String EIP_1167_CODE = "0x3d602d80600a3d3981f3363d3d373d3d3d363d73";
  private static final String EIP_1167_MINIMAL_PROXY_PREFIX = "0x363d3d373d3d3d363d73";
  private static final String EIP_1167_MINIMAL_PROXY_SUFFIX = "5af43d82803e903d91602b57fd5bf3";
  private static final String EIP_1967_PROXY_IMPL_CODE = "360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
  private static final String EIP_1967_PROXY_BEACON_CODE = "a3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";
  private static final String EIP_1967_PROXY_ADMIN_CODE = "b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103";
  private static final String EIP_1822_PROXIABLE_CODE = "c5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7";
  private static final String ZEPPELINOS_PROXY_IMPL_CODE = "7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3";
  private static final String EIP_3561_PROXY_NEXT_IMPL_CODE = "e8c186b11a4be12af079b0a5c235146db6f3615b2a8b1b47f9bfe3a956337ef9";
  private static final String ZEPPELINOS_BEACON_PROXY_SUB_PROXY_STORAGE_ADDRESS = "0000000000000000000000000000000000000000000000000000000000000001";
  private static final String ZEPPELINOS_BEACON_PROXY_SIGNATURE = "3659cfe6";
  private static final List<String[]> FLAG_HASH_POSITION_HASH_PAIRS = Arrays.asList(
      // eip-1967 eip1967.proxy.implementation
      // etc: 0x49542ad0f1429932e5b0590f17e676523f0a6369
      new String[]{EIP_1967_PROXY_IMPL_CODE, EIP_1967_PROXY_IMPL_CODE},

      // eip-1967 eip1967.proxy.beacon
      // etc: 0x00c6b1000e1de72e7bbbd7c286f79a6bceacf7d7
      new String[]{EIP_1967_PROXY_BEACON_CODE, EIP_1967_PROXY_BEACON_CODE},

      // eip-1967 eip1967.proxy.admin
      // etc: 0xbe86f647b167567525ccaafcd6f881f1ee558216
      new String[]{EIP_1967_PROXY_ADMIN_CODE, EIP_1967_PROXY_ADMIN_CODE},

      // eip-1822 keccak256("PROXIABLE")
      // etc: 0xc9f848a7c334d5c3790ec72826c2e5e7c465ba26
      new String[]{EIP_1822_PROXIABLE_CODE, EIP_1822_PROXIABLE_CODE},

      // org.zeppelinos.proxy.implementation
      // etc: 0x1e053d89e08c24aa2ce5c5b4206744dc2d7bd8f5
      new String[]{ZEPPELINOS_PROXY_IMPL_CODE, ZEPPELINOS_PROXY_IMPL_CODE},

      // eip-3561 eip3561.proxy.next_implementation
      new String[]{EIP_3561_PROXY_NEXT_IMPL_CODE, EIP_3561_PROXY_NEXT_IMPL_CODE});

  private final Web3jClient client;

  public ContractDiscriminator(Web3jClient client) {
    this.client = client;
  }

  public Optional<String> getContractCode(String contractAddress) throws IOException {
    return client.getContractCode(contractAddress, LATEST_BLOCK_NUMBER);
  }

  private String getByteCode(String contractAddress, String byteCode) {
    if (byteCode == null || byteCode.equals("0x")) {
      Optional<String> implCode = client.getContractCode(contractAddress, LATEST_BLOCK_NUMBER);
      return implCode.map(String::toLowerCase).orElse(null);
    }
    return HexUtil.to0xHex(byteCode).toLowerCase();
  }

  public ContractInfo getContractInfo(String contractAddress, String byteCode) {
    byteCode = getByteCode(contractAddress, byteCode);
    Optional<String[]> implAddressAndCodeOptional = getImplAddressAndCode(contractAddress, byteCode,
        LATEST_BLOCK_NUMBER, 0);

    if (implAddressAndCodeOptional.isPresent()) {
      String[] implAddressAndCode = implAddressAndCodeOptional.get();
      String proxyAddress = implAddressAndCode[0];
      String proxyCode = implAddressAndCode[1];
      ContractInfo contractInfo = ContractDiscriminator.getContractInfoWithContractType(proxyCode);
      contractInfo.setProxy(true);
      contractInfo.setImplAddress(proxyAddress.toLowerCase());
      contractInfo.setImplCode(proxyCode.toLowerCase());
      return contractInfo;
    } else {
      ContractInfo contractInfo = ContractDiscriminator.getContractInfoWithContractType(byteCode);
      contractInfo.setProxy(false);
      return contractInfo;
    }
  }

  @Nullable
  public TokenInfo getTokenInfo(String contractAddress, ContractInfo info) {
    List<TokenInfo> tokenInfos = batchGetTokenInfosWithBatchSize(Arrays.asList(contractAddress),
        Arrays.asList(info), 1);
    return tokenInfos.get(0);
  }

  /**
   * If a batch is too large, it can cause the return length of the request to exceed the limit. As
   * 10 multiCalls can be generated for a single address, it is necessary to have reasonable
   * control.
   */
  public List<TokenInfo> batchGetTokenInfosWithBatchSize(List<String> contractAddresses,
      List<ContractInfo> infos, int batchSize) {
    ArrayList<TokenInfo> tokenInfos = new ArrayList<TokenInfo>();
    for (int startAt = 0; startAt < contractAddresses.size(); startAt += batchSize) {
      int endAt = Math.min(startAt + batchSize, contractAddresses.size());
      List<String> splitContractAddresses = contractAddresses.subList(startAt, endAt);
      List<ContractInfo> splitInfos = infos.subList(startAt, endAt);
      tokenInfos.addAll(batchGetTokenInfos(splitContractAddresses, splitInfos));
    }
    return tokenInfos;
  }

  private List<TokenInfo> batchGetTokenInfos(List<String> contractAddresses,
      List<ContractInfo> infos) {
    ArrayList<TokenInfo> tokenInfos = new ArrayList<TokenInfo>();
    ArrayList<MultiCallParameter> multiCallParameters = new ArrayList<MultiCallParameter>();

    for (int idx = 0; idx < contractAddresses.size(); idx++) {
      String contractAddress = contractAddresses.get(idx);
      ContractInfo info = infos.get(idx);

      multiCallParameters.addAll(
          getMultiCallParametersWithProxyAddress(contractAddress, info.getImplAddress(), "name"));
      multiCallParameters.addAll(
          getMultiCallParametersWithProxyAddress(contractAddress, info.getImplAddress(), "symbol"));
      multiCallParameters.addAll(
          getMultiCallParametersWithProxyAddress(contractAddress, info.getImplAddress(),
              "decimals"));
      multiCallParameters.addAll(
          getMultiCallParametersWithProxyAddress(contractAddress, info.getImplAddress(),
              "totalSupply"));
    }

    List<Object> results = this.client.batchCall(multiCallParameters,
        DefaultBlockParameterName.LATEST);

    int startAtIdx = 0;
    for (int idx = 0; idx < contractAddresses.size(); idx++) {
      ContractInfo info = infos.get(idx);

      int nameResultsLen = info.isProxy() ? 12 : 6;
      int symbolResultsLen = info.isProxy() ? 12 : 6;
      int decimalsResultLen = info.isProxy() ? 4 : 2;
      int totalSupplyResultLen = info.isProxy() ? 2 : 1;

      String name = (String) parseResults(results, startAtIdx, nameResultsLen);
      startAtIdx += nameResultsLen;

      String symbol = (String) parseResults(results, startAtIdx, symbolResultsLen);
      startAtIdx += symbolResultsLen;

      Integer decimals = (Integer) parseResults(results, startAtIdx, decimalsResultLen);
      startAtIdx += decimalsResultLen;

      BigInteger totalSupply = (BigInteger) parseResults(results, startAtIdx, totalSupplyResultLen);
      startAtIdx += totalSupplyResultLen;
      // token info 不根据合约类型作筛选
      tokenInfos.add(
          new TokenInfo(removeInvalidCharacters(name), removeInvalidCharacters(symbol), decimals,
              totalSupply));
    }

    return tokenInfos;
  }

  /**
   * Retrieve the actual contract address behind the proxy contract.
   */
  private Optional<String[]> getImplAddressAndCode(String proxyAddress, String proxyOpCode,
      DefaultBlockParameter blockNumber, int depth) {
    // To prevent stack overflow
    if (depth > 3) {
      LOGGER.warn("Terminate the recursion if the depth exceeds three layers.");
      return Optional.of(new String[]{proxyAddress, proxyOpCode});
    }

    // 1). Call the implementation function directly
    MultiCallParameter callImplFunctionParameter = new MultiCallParameter(proxyAddress,
        Collections.emptyList(), "implementation", "implementation()", "(address)", null);
    List<Object> implementationResult = client.batchCall(Arrays.asList(callImplFunctionParameter),
        blockNumber);
    if (implementationResult.get(0) != null) {
      String implAddress = ((Address) implementationResult.get(0)).toString();

      if (EvmUtil.isBlockHoleAddress(implAddress)) {
        return Optional.of(new String[]{proxyAddress, proxyOpCode});
      }
      Optional<String> implCode = client.getContractCode(implAddress, LATEST_BLOCK_NUMBER);
      if (!implCode.isPresent()) {
        return Optional.empty();
      }
      // bsc:0xecA88125a5ADbe82614ffC12D0DB554E2e2867C8 EIP-897
      // 这种类似代理合约在递归调用implementation()之后会返回0x,如果不兼容处理，将返回空地址，导致后续使用合约地址获取token name,symbols等信息失败
      if (implCode.get().equals("0x")) {
        return Optional.of(new String[]{proxyAddress, proxyOpCode});
      }

      return getImplAddressAndCode(implAddress, implCode.get(), blockNumber, depth + 1);
    }

    // 2). Utilizing a standardized proxy protocol to obtain the implementation slot

    // eip-1167
    // etc: 0xde400a2ed5a5f649a8cff6445a24ab934ff32b2c
    if (proxyOpCode.startsWith(EIP_1167_CODE) && proxyOpCode.length() >= 80) {
      String implAddress = HexUtil.to0xHex(proxyOpCode.substring(40, 80));

      Optional<String> implCode = client.getContractCode(implAddress, LATEST_BLOCK_NUMBER);
      if (!implCode.isPresent()) {
        return Optional.empty();
      }

      return Optional.empty();
    }

    // eip1167 minimal proxies
    // etc: 0x363d3d373d3d3d363d735af43d82803e903d91602b57fd5bf3
    if (proxyOpCode.startsWith(EIP_1167_MINIMAL_PROXY_PREFIX) && proxyOpCode.endsWith(
        EIP_1167_MINIMAL_PROXY_SUFFIX) && proxyOpCode.length() >= 80) {
      String implAddress = HexUtil.to0xHex(
          proxyOpCode.substring(EIP_1167_MINIMAL_PROXY_PREFIX.length(),
              proxyOpCode.indexOf(EIP_1167_MINIMAL_PROXY_SUFFIX)));
      Optional<String> implCode = client.getContractCode(implAddress, LATEST_BLOCK_NUMBER);
      if (implCode.isPresent()) {
        return getImplAddressAndCode(implAddress, implCode.get(), blockNumber, depth + 1);
      } else {
        return Optional.empty();
      }
    }

    for (String[] flagAndPosition : FLAG_HASH_POSITION_HASH_PAIRS) {
      if (proxyOpCode.contains(flagAndPosition[0])) {
        Optional<String> implStorageCode = client.getStorageAt(proxyAddress, flagAndPosition[1],
            LATEST_BLOCK_NUMBER);
        if (!implStorageCode.isPresent()) {
          return Optional.empty();
        }

        var implAddress = HexUtil.to0xHex(
            implStorageCode.get().substring(implStorageCode.get().length() - 40));

        if (EvmUtil.isBlockHoleAddress(implAddress)) {
          return Optional.of(new String[]{proxyAddress, proxyOpCode});
        }

        var implCode = client.getContractCode(implAddress, LATEST_BLOCK_NUMBER);
        if (!implCode.isPresent()) {
          return Optional.empty();
        }

        return getImplAddressAndCode(implAddress, implCode.get(), blockNumber, depth + 1);
      }
    }

    // Zeppelions UpgradeableBeacon
    // etc: 0x0890b2d8b6F938CF93b8b243AB8f0589031e158f
    // Placing it at the end, this rule can potentially result in incorrect matches
    if (proxyOpCode.contains(ZEPPELINOS_BEACON_PROXY_SIGNATURE)) {
      var implStorageCode = client.getStorageAt(proxyAddress,
          ZEPPELINOS_BEACON_PROXY_SUB_PROXY_STORAGE_ADDRESS, LATEST_BLOCK_NUMBER);
      if (!implStorageCode.isPresent()) {
        return Optional.empty();
      }

      var implAddress = HexUtil.to0xHex(
          implStorageCode.get().substring(implStorageCode.get().length() - 40));

      var implCode = client.getContractCode(implAddress, LATEST_BLOCK_NUMBER);
      if (!implCode.isPresent()) {
        return Optional.empty();
      }

      if (!implCode.get().equals("0x")) {
        return getImplAddressAndCode(implAddress, implCode.get(), blockNumber, depth + 1);
      }
    }

    // 3). If it is not a first level proxy,
    // then the current address is the final result instead of a proxy address.
    // Otherwise, no proxied address was found.
    if (depth != 0) {
      return Optional.of(new String[]{proxyAddress, proxyOpCode});
    } else {
      return Optional.empty();
    }
  }

  private List<MultiCallParameter> getMultiCallParametersWithProxyAddress(String contractAddress,
      @Nullable String ImplAddress, String functionName) {
    var multiCallParametersBuilder = ImmutableList.<MultiCallParameter>builder();

    var name2Provider = ImmutableMap.of("name", nameMultiCallParametersProvider(), "symbol",
        symbolMultiCallParametersProvider(), "decimals", decimalMultiCallParametersProvider(),
        "totalSupply", totalSupplyMultiCallParametersProvider());

    var provider = name2Provider.get(functionName);
    if (provider == null) {
      throw new IllegalArgumentException(
          String.format("The function name is unknown: %s", functionName));
    }

    multiCallParametersBuilder.addAll(provider.apply(contractAddress));
    if (ImplAddress != null) {
      multiCallParametersBuilder.addAll(provider.apply(ImplAddress));
    }

    return multiCallParametersBuilder.build();
  }

  @Getter
  @Setter
  @NoArgsConstructor
  @AllArgsConstructor
  public static class ContractInfo {

    private boolean isErc20;

    private boolean isErc721;

    private boolean isErc1155;

    private boolean isProxy;

    @Nullable
    private String implAddress;

    @Nullable
    private String implCode;
  }

  @Getter
  @Setter
  @NoArgsConstructor
  @ToString
  public static class TokenInfo {

    private String name;

    private String symbol;

    private Integer decimals;

    private BigInteger totalSupply;

    public TokenInfo(@Nullable String name, @Nullable String symbol, @Nullable Integer decimals,
        @Nullable BigInteger totalSupply) {
      this.name = name == null ? "" : name;
      this.symbol = symbol == null ? "" : symbol;
      this.decimals = decimals == null ? 0 : decimals;
      this.totalSupply = totalSupply == null ? new BigInteger("0") : totalSupply;
    }
  }

  @VisibleForTesting
  static boolean isErc20(List<String> sighashes) {
    return (isImplements(sighashes, "approve(address,uint256)") || isImplements(sighashes,
        "totalSupply()")) && isImplements(sighashes, "transfer(address,uint256)")
        // To avoid misidentifying ERC721 contracts as ERC20 ones, it is recommended to exclude all
        // ERC721 contracts as well.
        && !isImplements(sighashes, "setApprovalForAll(address,bool)");
  }

  @VisibleForTesting
  static boolean isErc721(List<String> sighashes) {
    return isImplements(sighashes, "setApprovalForAll(address,bool)");
  }

  @VisibleForTesting
  static boolean isErc1155(List<String> sighashes) {
    return isImplements(sighashes, "safeTransferFrom(address,address,uint256,uint256,bytes)")
        && isImplements(sighashes,
        "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)");
  }

  private static boolean isImplements(List<String> sighashes, String functionSignature) {
    return sighashes.contains(new Function(functionSignature).selectorHex());
  }

  private static ContractInfo getContractInfoWithContractType(String byteCode) {
    List<Opcode> opcodes = new Disassembler(byteCode).getOpcodes();

    List<String> sighashes = opcodes.stream()
        .filter(opcode -> opcode.getOpcode().equals(Opcodes.PUSH4)).map(Opcode::getHexParameter)
        .collect(Collectors.toList());

    return getContractInfoWithContractType(sighashes);
  }

  private static ContractInfo getContractInfoWithContractType(List<String> sighashes) {
    ContractInfo contractInfo = new ContractInfo();

    if (isErc20(sighashes) && !isErc721(sighashes)) {
      contractInfo.setErc20(true);
    }

    if (isErc721(sighashes)) {
      contractInfo.setErc721(true);
    }

    if (isErc1155(sighashes)) {
      contractInfo.setErc1155(true);
    }

    return contractInfo;
  }

  // NUL（0x00）
  // Tab（0x09）
  // Line Feed（0x0A）
  // Vertical Tab（0x0B）
  // Form Feed（0x0C）
  // Carriage Return（0x0D）
  // Control Code（0x7F）
  @Nullable
  private static String removeInvalidCharacters(@Nullable String str) {
    if (str == null) {
      return null;
    }
    return str.replaceAll("[\\x00-\\x08\\x0B\\x0C\\x0E-\\x1F\\x7F]", "");
  }

  @Nullable
  private Object parseResults(List<Object> multiCallResults, int startAt, int length) {
    for (int idx = startAt; idx < startAt + length; idx += 1) {
      Object result = multiCallResults.get(idx);

      if (result == null) {
        continue;
      }

      if (result instanceof String) {
        if (!Strings.isBlank((String) result) && !Strings.isEmpty((String) result)) {
          return result;
        }
      } else if (result instanceof Integer) {
        return result;
      } else if (result instanceof BigInteger) {
        return result;
      } else if (result instanceof byte[]) {
        byte[] bytesResult = (byte[]) result;
        if (bytesResult.length > 0) {
          String bytesStr = new String(bytesResult, StandardCharsets.US_ASCII);
          if (!Strings.isBlank(bytesStr) && !Strings.isEmpty(bytesStr)) {
            return bytesStr;
          }
        }
      } else {
        throw new RuntimeException(
            String.format("The type of result isn't be supported: %s", result));
      }
    }

    return null;
  }

  private java.util.function.Function<String, List<MultiCallParameter>> nameMultiCallParametersProvider() {
    return address -> Arrays.asList(
        new MultiCallParameter(address, Collections.emptyList(), "name", "name()", "(string)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "Name", "Name()", "(string)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "name", "name()", "(bytes)", null),
        new MultiCallParameter(address, Collections.emptyList(), "Name", "Name()", "(bytes)", null),
        new MultiCallParameter(address, Collections.emptyList(), "name", "name()", "(bytes32)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "Name", "Name()", "(bytes32)",
            null));
  }

  private java.util.function.Function<String, List<MultiCallParameter>> symbolMultiCallParametersProvider() {
    return address -> Arrays.asList(
        new MultiCallParameter(address, Collections.emptyList(), "symbol", "symbol()", "(string)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "SYMBOL", "SYMBOL()", "(string)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "symbol", "symbol()", "(bytes)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "SYMBOL", "SYMBOL()", "(bytes)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "SYMBOL", "SYMBOL()", "(bytes32)",
            null),
        new MultiCallParameter(address, Collections.emptyList(), "symbol", "symbol()", "(bytes32)",
            null));
  }

  private java.util.function.Function<String, List<MultiCallParameter>> decimalMultiCallParametersProvider() {
    return address -> Arrays.asList(
        new MultiCallParameter(address, Collections.emptyList(), "decimals", "decimals()",
            "(uint8)", null),
        new MultiCallParameter(address, Collections.emptyList(), "DECIMALS", "DECIMALS()",
            "(uint8)", null));
  }

  private java.util.function.Function<String, List<MultiCallParameter>> totalSupplyMultiCallParametersProvider() {
    return address -> Arrays.asList(
        new MultiCallParameter(address, Collections.emptyList(), "totalSupply", "totalSupply()",
            "(uint256)", null));
  }
}
