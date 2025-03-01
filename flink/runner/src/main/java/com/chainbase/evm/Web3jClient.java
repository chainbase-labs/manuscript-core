package com.chainbase.evm;

import static com.chainbase.evm.HexUtil.DEV_WALLET_ADDRESS;

import com.chainbase.evm.response.BlockBalanceChangeResponse;
import com.chainbase.evm.response.TraceBlockResponse;
import com.esaulpaugh.headlong.abi.Tuple;
import com.google.common.base.Stopwatch;
import com.google.common.collect.Streams;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.BufferUnderflowException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import javax.annotation.Nullable;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.ToString;
import org.bouncycastle.util.encoders.Hex;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.web3j.abi.FunctionEncoder;
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Bool;
import org.web3j.abi.datatypes.DynamicArray;
import org.web3j.abi.datatypes.DynamicBytes;
import org.web3j.abi.datatypes.DynamicStruct;
import org.web3j.abi.datatypes.Function;
import org.web3j.abi.datatypes.Type;
import org.web3j.protocol.Web3j;
import org.web3j.protocol.Web3jService;
import org.web3j.protocol.core.BatchRequest;
import org.web3j.protocol.core.BatchResponse;
import org.web3j.protocol.core.DefaultBlockParameter;
import org.web3j.protocol.core.DefaultBlockParameterName;
import org.web3j.protocol.core.DefaultBlockParameterNumber;
import org.web3j.protocol.core.Request;
import org.web3j.protocol.core.Response;
import org.web3j.protocol.core.Response.Error;
import org.web3j.protocol.core.methods.request.Transaction;
import org.web3j.protocol.core.methods.response.EthBlock;
import org.web3j.protocol.core.methods.response.EthCall;
import org.web3j.protocol.core.methods.response.EthGetBalance;
import org.web3j.protocol.core.methods.response.EthGetCode;
import org.web3j.protocol.core.methods.response.EthGetStorageAt;
import org.web3j.protocol.exceptions.ClientConnectionException;
import org.web3j.protocol.http.HttpService;

@Getter
public class Web3jClient {

  private static final Logger LOGGER = LoggerFactory.getLogger(Web3jClient.class);

  private static final String EIP_1167_CODE = "3d602d80600a3d3981f3363d3d373d3d3d363d73";

  private static final String EIP_1167_MINIMAL_PROXY_PREFIX = "0x363d3d373d3d3d363d73";

  private static final String EIP_1167_MINIMAL_PROXY_SUFFIX = "5af43d82803e903d91602b57fd5bf3";

  private static final String EIP_1967_PROXY_IMPL_CODE =
          "360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";

  private static final String EIP_1967_PROXY_BEACON_CODE =
          "a3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";

  private static final String EIP_1822_PROXIABLE_CODE =
          "c5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7";

  private static final String ZEPPELINOS_PROXY_IMPL_CODE =
          "7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3";

  private static final String EIP_3561_PROXY_NEXT_IMPL_CODE =
          "e8c186b11a4be12af079b0a5c235146db6f3615b2a8b1b47f9bfe3a956337ef9";

  private static final String ZEPPELINOS_BEACON_PROXY_SUB_PROXY_STORAGE_ADDRESS =
          "0000000000000000000000000000000000000000000000000000000000000001";

  private static final String MULTICALL_V1_ADDRESS = "0xeefba1e63905ef1d7acba5a8513c70307c1ce441";

  private static final String MULTICALL_V1_AGGREGATE_INPUT_SIGN = "aggregate((address,bytes)[])";

  private static final String MULTICALL_V1_AGGREGATE_OUTPUT_SIGN = "(uint256,bytes[])";

  private static final String MULTICALL_V3_ADDRESS = "0xcA11bde05977b3631167028862bE2a173976CA11";

  private static final String MULTICALL_V3_AGGREGATE_INPUT_SIGN =
          "aggregate3((address,bool,bytes)[])";

  private static final String MULTICALL_V3_AGGREGATE_OUTPUT_SIGN = "((bool,bytes)[])";

  private static final String ERC20_BALANCE_OF_INPUT_SIGN = "balanceOf(address)";

  private static final String ERC20_BALANCE_OF_OUTPUT_SIGN = "(uint256)";

  private static final String ERC721_OWNER_OF_INPUT_SIGN = "ownerOf(uint256)";

  private static final String ERC721_OWNER_OF_OUTPUT_SIGN = "(address)";

  private static final String ERC1155_BALANCE_OF_INPUT_SIGN = "balanceOf(address,uint256)";

  private static final String ERC1155_BALANCE_OF_OUTPUT_SIGN = "(uint256)";

  private static final String UNISWAP_GET_RESERVES_INPUT_SIGN = "getReserves()";

  private static final String UNISWAP_GET_RESERVES_OUTPUT_SIGN = "(uint256,uint256)";

  private static final String ERC20_TOTALSUPPLY_INPUT_SIGN = "totalSupply()";
  private static final String ERC20_TOTALSUPPLY_OUTPUT_SIGN = "(uint256)";

  private static final String ERC20_NAME_INPUT_SIGN = "name()";
  private static final String ERC20_NAME_OUTPUT_SIGN = "(string)";

  private static final String ERC20_SYMBOL_INPUT_SIGN = "symbol()";
  private static final String ERC20_SYMBOL_OUTPUT_SIGN = "(string)";

  private static final DefaultBlockParameter LATEST_BLOCK_NUMBER = DefaultBlockParameterName.LATEST;

  private static final Integer DEFAULT_RETRY_NUMBER = 3;

  private static final Integer DEFAULT_RETRY_INTERVAL_MS = 100;

  private static final int BATCH_CALL_LIMIT = 100;

  private final Web3j web3j;
  private final Web3jService web3Service;

  private final int retryNumber;

  private final int retryIntervalMs;

  private final boolean ignoreWeb3jIOException;

  public Web3jClient(String nodeURL) {
    this(nodeURL, DEFAULT_RETRY_NUMBER, DEFAULT_RETRY_INTERVAL_MS, false);
  }

  public Web3jClient(String nodeURL, boolean ignoreWeb3jIOException) {
    this(nodeURL, DEFAULT_RETRY_NUMBER, DEFAULT_RETRY_INTERVAL_MS, ignoreWeb3jIOException);
  }

  public Web3jClient(
          String nodeURL, int retryNumber, int retryIntervalMs, boolean ignoreWeb3jIOException) {
    this.web3Service = new HttpService(nodeURL);
    this.web3j = Web3j.build(web3Service);
    this.retryNumber = retryNumber;
    this.retryIntervalMs = retryIntervalMs;
    this.ignoreWeb3jIOException = ignoreWeb3jIOException;
  }

  // getByteCode
  public String getByteCode(String contractAddress) throws IOException {
    EthGetCode getCode = web3j.ethGetCode(contractAddress, DefaultBlockParameterName.LATEST).send();
    return getCode.getCode();
  }

  public Optional<Tuple> callFunctionWithBlockNumber(CallFunctionParameter parameter) {
    return batchCallFunctionWithBlockNumber(Arrays.asList(parameter)).get(0);
  }

  public List<Optional<Tuple>> batchCallFunctionWithBlockNumber(
          List<CallFunctionParameter> parameters) {
    return parameters.stream()
            .collect(Collectors.groupingBy(i -> (parameters.indexOf(i) / BATCH_CALL_LIMIT)))
            .values()
            .stream()
            .map(this::batchCallFunctionWithBlockNumberLimit)
            .flatMap(Collection::stream)
            .collect(Collectors.toList());
  }

  /**
   * If you need to call ethCall method with a batch of different blockNumbers,I recommend to use
   * this method;
   *
   * <p>If you need to call ethCall method with a batch of same blockNumbers, I recommend to use
   * 'multiCall' method
   */
  public List<Optional<Tuple>> batchCallFunctionWithBlockNumberLimit(
          List<CallFunctionParameter> parameters) {
    BatchRequest batch = web3j.newBatch();

    List<String> functions =
            parameters.stream()
                    .map(
                            parameter ->
                                    FunctionEncoder.encode(
                                            new Function(
                                                    parameter.getFunctionName(),
                                                    parameter.getInputs(),
                                                    /**
                                                     * 由于下方代码使用 headlong 完成 decode，所以这里 outputParameters 可以传入一个空数组占位即可
                                                     * 这个参数不会参与 function decode 的计算
                                                     */
                                                    Arrays.asList())))
                    .collect(Collectors.toList());

    for (int idx = 0; idx < functions.size(); idx += 1) {
      String function = functions.get(idx);
      CallFunctionParameter parameter = parameters.get(idx);
      batch.add(
              web3j.ethCall(
                      Transaction.createEthCallTransaction(
                              DEV_WALLET_ADDRESS, parameter.getContractAddress(), function),
                      parameter.getBlockNumber()));
    }

    ArrayList<Optional<Tuple>> results = new ArrayList<>();

    List<? extends Response<?>> batchResponse;
    Optional<BatchResponse> optionalBatchResponse = batchSendWithLimitRetry(batch);
    if (optionalBatchResponse.isPresent()) {
      batchResponse = optionalBatchResponse.get().getResponses();
    } else {
      return IntStream.range(0, parameters.size())
              .mapToObj(idx -> Optional.<Tuple>empty())
              .collect(Collectors.toList());
    }

    for (int idx = 0; idx < batchResponse.size(); idx += 1) {
      Response response = batchResponse.get(idx);
      CallFunctionParameter parameter = parameters.get(idx);

      if (response.getError() != null) {
        LOGGER.warn(
                String.format(
                        "Failed to call contract, params: %s, error: %s",
                        parameter.toString(), response.getError().getMessage()));
        results.add(Optional.empty());
      } else {
        results.add(decodeCallResponse(parameter, (String) response.getResult()));
      }
    }

    return results;
  }

  public List<Object> batchCall(
          List<MultiCallParameter> parameters, DefaultBlockParameter blockParameter) {

    Stopwatch stop = Stopwatch.createStarted();
    List<CallFunctionParameter> multiFunctionParameterList =
            parameters.stream()
                    .map(CallFunctionParameter::new)
                    .peek(parameter -> parameter.setBlockNumber(blockParameter))
                    .collect(Collectors.toList());

    List<Optional<Tuple>> results = this.batchCallFunctionWithBlockNumber(multiFunctionParameterList);
    LOGGER.info(
            String.format("BatchCall execution time: %d ms", stop.elapsed(TimeUnit.MILLISECONDS)));
    return Streams.zip(
                    results.stream(),
                    multiFunctionParameterList.stream(),
                    (result, parameter) ->
                            result
                                    .map(
                                            item ->
                                                    !(parameter.getDefaultValue() instanceof Tuple)
                                                            && !parameter.getFunctionOutputSign().contains(",")
                                                            ? item.get(0)
                                                            : (Object) item)
                                    .orElse(parameter.getDefaultValue()))
            .collect(Collectors.toList());
  }

  /**
   * If you need to call ethCall method with a batch of different blockNumbers,I recommend to use
   * 'batchCallFunctionWithBlockNumber' method;
   *
   * <p>If you need to call ethCall method with a batch of same blockNumbers, I recommend to use
   * this method
   */
  public List<Object> multiCall(
          List<MultiCallParameter> parameters, DefaultBlockParameter blockParameter) {

    List<DynamicStruct> parameterTypes =
            parameters.stream()
                    .map(
                            p -> {
                              String encode =
                                      FunctionEncoder.encode(
                                              new Function(p.getFunctionName(), p.inputs, Collections.emptyList()));
                              return new DynamicStruct(
                                      new Address(p.getContractAddress()),
                                      new Bool(true),
                                      new DynamicBytes(Hex.decode(encode.substring(2).getBytes())));
                            })
                    .collect(Collectors.toList());

    List<Type> inputParameters = Collections.singletonList(new DynamicArray<>(DynamicStruct.class, parameterTypes));

    String aggregateFunctionEncode =
            FunctionEncoder.encode(new Function("aggregate3", inputParameters, Collections.emptyList()));

    Transaction ethCallTransaction =
            Transaction.createEthCallTransaction(
                    Address.DEFAULT.getValue(), MULTICALL_V3_ADDRESS, aggregateFunctionEncode);

    EthCall sendResult;
    Optional<EthCall> optionalSendResult = ethCall(ethCallTransaction, blockParameter);
    if (optionalSendResult.isPresent()) {
      sendResult = optionalSendResult.get();
      if (Objects.nonNull(sendResult.getError())) {
        throw new RPCCallException(sendResult.getError().getMessage(), sendResult);
      }
    } else {
      throw new RPCCallException("The call fails, and the result is empty", null);
    }

    // 2). 解析 aggregate 函数的返回值
    com.esaulpaugh.headlong.abi.Function aggregateHeadlongFunction =
            new com.esaulpaugh.headlong.abi.Function(
                    MULTICALL_V3_AGGREGATE_INPUT_SIGN, MULTICALL_V3_AGGREGATE_OUTPUT_SIGN);

    // 现在发现在某些合约的 balanceOf 方法会产生 write action，调用的时候会报错 out of gas
    Tuple[] decodedReturns;
    try {
      decodedReturns =
              aggregateHeadlongFunction.decodeReturn(
                      HexUtil.hex2Bytes(sendResult.getResult()),
                      IntStream.range(0, aggregateHeadlongFunction.getOutputs().size()).toArray());
    } catch (BufferUnderflowException ex) {
      String sendMsg = Optional.ofNullable(sendResult.getError()).map(Error::getMessage).orElse("");
      LOGGER.warn(
              String.format(
                      "Failed to decode the return, result: %s, error: %s, parameters size: %d",
                      sendResult.getResult(), sendMsg, parameters.size()),
              ex);
      throw ex;
    }

    // 3). 解析内部函数的返回值
    List<Object> result = new ArrayList<>();

    for (int idx = 0; idx < decodedReturns.length; idx += 1) {
      MultiCallParameter parameter = parameters.get(idx);
      Tuple decodedReturn = decodedReturns[idx];
      com.esaulpaugh.headlong.abi.Function headlongFunction =
              new com.esaulpaugh.headlong.abi.Function(
                      parameter.getFunctionInputSign(), parameter.getFunctionOutputSign());
      int[] indices = IntStream.range(0, headlongFunction.getOutputs().size()).toArray();

      Object functionDecodedReturn;
      try {
        functionDecodedReturn =
                headlongFunction.decodeReturn((byte[]) decodedReturn.get(1), indices);
      } catch (BufferUnderflowException ex) {
        functionDecodedReturn = parameter.getDefaultValue();
      } catch (IllegalArgumentException ex) {
        if (ex.getMessage().contains("unsigned val exceeds bit limit")) {
          functionDecodedReturn = parameter.getDefaultValue();
        } else {
          LOGGER.error(
                  String.format(
                          "The decoded return from aggregate3 can't be decoded, parameters size: %d",
                          parameters.size()),
                  ex);
          functionDecodedReturn = null;
        }
      } catch (Exception ex) {
        LOGGER.error(
                String.format(
                        "The decoded return from aggregate3 can't be decoded, parameters size: %d",
                        parameters.size()),
                ex);
        functionDecodedReturn = null;
      }

      result.add(functionDecodedReturn);
    }

    return result;
  }

  private Optional<Tuple> decodeCallResponse(CallFunctionParameter parameter, String response) {
    try {
      com.esaulpaugh.headlong.abi.Function headlongFunction =
              new com.esaulpaugh.headlong.abi.Function(
                      parameter.getFunctionInputSign(), parameter.getFunctionOutputSign());

      /**
       * 这里看起来会比较奇怪，第二个参数是一个optional的参数，但是如果不传入的话 decodeReturn 函数在decoded 完成以后会做一个判断，看是不是所有的 byte
       * 都被使用到了，如果没有被使用到的会抛出异常， 可见 decode
       * 源代码：https://github.com/esaulpaugh/headlong/blob/master/src/main/java/com/esaulpaugh/headlong/abi/ABIType.java#L165
       * 填入第二个 indices 数组就是为了规避这个问题
       *
       * <p>本来也是可以直接使用 web3j 提供的 FunctionReturnDecoder 完成 decode，但是需要动态的生成 outputParameters
       * 参数，会比较麻烦： FunctionReturnDecoder.decode(response.getValue, function.getOutputParameters)
       *
       * <p>一个例子： call
       * 0xed279fdd11ca84beef15af5d39bb4d4bee23f0ca.balanceOf(0x94a0321182f95b4b54537a6dd78df5750df3513c)
       * 在 16181083 区块的数据时会返回：
       * 0x00000000000000000000000000000000000000000000000000000000000000000df3513c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
       * 但实际只需要使用到前32位，decode之后的结果是 uint256 的 0
       */
      Object decodedReturn =
              headlongFunction.decodeReturn(
                      HexUtil.hex2Bytes(response),
                      IntStream.range(0, headlongFunction.getOutputs().size()).toArray());

      /**
       * decodeReturn 方法会根据 indices 的个数来动态决定返回类型： 1. 如果 indices.size == 1，返回 Object 2. 如果
       * indices.size > 1，返回 Tuple 这里需要保证返回的类型一致
       */
      Tuple decodedReturnTuple;
      if (decodedReturn instanceof Tuple) {
        decodedReturnTuple = (Tuple) decodedReturn;
      } else {
        decodedReturnTuple = Tuple.of(decodedReturn);
      }

      return Optional.of(decodedReturnTuple);
    } catch (Exception ex) {
      String blockNumberStr;
      if (parameter.getBlockNumber() instanceof DefaultBlockParameterNumber) {
        blockNumberStr = parameter.getBlockNumber().toString();
      } else if (parameter.getBlockNumber() instanceof DefaultBlockParameterName) {
        blockNumberStr = parameter.getBlockNumber().getValue();
      } else {
        throw new RuntimeException(
                "The blockNumber does not correspond to either DefaultBlockParameterNumber nor DefaultBlockParameterName");
      }
      return Optional.empty();
    }
  }

  public Optional<EthCall> ethCall(Transaction transaction, DefaultBlockParameter blockNumber) {
    return this.sendWithLimitedRetry(web3j.ethCall(transaction, blockNumber));
  }

  public Optional<EthBlock> ethGetBlockByNumber(
          DefaultBlockParameter defaultBlockParameter, boolean returnFullTransactionObjects) {
    return this.sendWithLimitedRetry(
            web3j.ethGetBlockByNumber(defaultBlockParameter, returnFullTransactionObjects));
  }

  public Optional<EthBlock> ethGetBlockByHash(
          String blockHash, boolean returnFullTransactionObjects) {
    return this.sendWithLimitedRetry(
            web3j.ethGetBlockByHash(blockHash, returnFullTransactionObjects));
  }

  public Optional<EthBlock> ethGetUncleByBlockNumberAndIndex(
          DefaultBlockParameter defaultBlockParameter, BigInteger inex) {
    return this.sendWithLimitedRetry(
            web3j.ethGetUncleByBlockNumberAndIndex(defaultBlockParameter, inex));
  }

  public Optional<EthGetBalance> ethGetBalance(
          String address, DefaultBlockParameter defaultBlockParameter) {
    return this.sendWithLimitedRetry(web3j.ethGetBalance(address, defaultBlockParameter));
  }

  public Optional<BigInteger> balanceOf(String walletAddress, DefaultBlockParameter blockNumber) {
    return this.sendWithLimitedRetry(web3j.ethGetBalance(walletAddress, blockNumber))
            .map(EthGetBalance::getBalance);
  }

  public Optional<String> getStorageAt(
          String contractAddress, String position, DefaultBlockParameter blockNumber) {
    return this.sendWithLimitedRetry(
                    web3j.ethGetStorageAt(
                            contractAddress, new BigInteger(HexUtil.toPureHex(position), 16), blockNumber))
            .map(EthGetStorageAt::getData);
  }

  public Optional<String> getContractCode(
          String contractAddress, DefaultBlockParameter blockNumber) {
    return this.sendWithLimitedRetry(web3j.ethGetCode(contractAddress, blockNumber))
            .map(EthGetCode::getCode);
  }

  private Optional<BatchResponse> batchSendWithLimitRetry(BatchRequest batchRequest) {
    return tryRequest(batchRequest::send);
  }

  @FunctionalInterface
  private interface RequestSupplier<T> {

    T get() throws IOException, ClientConnectionException;
  }

  private <Resp> Optional<Resp> tryRequest(RequestSupplier<Resp> handle) {
    int sendRetryNumber = 0;
    Exception lastException = null;

    while (sendRetryNumber < this.retryNumber) {
      try {
        return Optional.of(handle.get());
      } catch (IOException | ClientConnectionException ex) {
        try {
          Thread.sleep(this.retryIntervalMs);
        } catch (InterruptedException sleepException) {
          throw new RuntimeException(sleepException);
        }
        sendRetryNumber += 1;
        lastException = ex;
      }
    }

    if (lastException != null && !this.ignoreWeb3jIOException) {
      throw new RuntimeException("Failed to send request to web3 node", lastException);
    }

    LOGGER.error(
            String.format("Failed to send request to web3 node with %d retry", this.retryNumber));
    return Optional.empty();
  }

  private <T extends Response> Optional<T> sendWithLimitedRetry(Request<?, T> request) {
    return tryRequest(request::send);
  }

  @Getter
  @Setter
  @NoArgsConstructor
  @AllArgsConstructor
  @ToString
  public static class MultiCallParameter {

    private String contractAddress;

    private List<Type> inputs;

    private String functionName;

    private String functionInputSign;

    private String functionOutputSign;

    @Nullable
    private Object defaultValue;
  }

  @Getter
  @Setter
  @NoArgsConstructor
  public static class CallFunctionParameter extends MultiCallParameter {

    public CallFunctionParameter(MultiCallParameter multiCallParameter) {
      this.setContractAddress(multiCallParameter.getContractAddress());
      this.setInputs(multiCallParameter.getInputs());
      this.setFunctionName(multiCallParameter.getFunctionName());
      this.setFunctionInputSign(multiCallParameter.getFunctionInputSign());
      this.setFunctionOutputSign(multiCallParameter.getFunctionOutputSign());
      this.setDefaultValue(multiCallParameter.getDefaultValue());
    }

    public CallFunctionParameter(
            MultiCallParameter multiCallParameter, DefaultBlockParameter blockNumber) {
      this(multiCallParameter);
      this.setBlockNumber(blockNumber);
    }

    private DefaultBlockParameter blockNumber;

    @Override
    public String toString() {
      return "CallFunctionParameter{"
              + "blockNumber="
              + blockNumber
              + ", contractAddress="
              + getContractAddress()
              + ", functionName="
              + getFunctionName()
              + ", inputs="
              + getInputs()
              + '}';
    }
  }

  public Request<?, ? extends Response<?>> ethEstimateGas(Transaction transaction) {
    return web3j.ethEstimateGas(transaction);
  }

  private static class DynamicResponse extends Response<Object> {

  }

  public Request<?, ? extends Response<?>> rpcRequest(String functionName, Object... params) {
    return new Request<>(
            functionName, Arrays.asList(params), this.web3Service, DynamicResponse.class);
  }

  public Optional<TraceBlockResponse> debugTraceBlockByNumber(
          DefaultBlockParameter blockParameter) {
    Map<String, Object> params = new HashMap<>();
    params.put("tracerConfig", Collections.singletonMap("diffMode", true));
    params.put("tracer", "prestateTracer");
    Request<?, TraceBlockResponse> request =
            new Request<>(
                    "debug_traceBlockByNumber",
                    Arrays.asList(blockParameter.getValue(), params),
                    this.web3Service,
                    TraceBlockResponse.class);
    return tryRequest(request::send);
  }

  public Optional<BlockBalanceChangeResponse> getBalanceChangesInBlock(
          DefaultBlockParameter blockParameter) {
    Request<?, BlockBalanceChangeResponse> request =
            new Request<>(
                    "erigon_getBalanceChangesInBlock",
                    Collections.singletonList(blockParameter.getValue()),
                    this.web3Service,
                    BlockBalanceChangeResponse.class);
    return this.sendWithLimitedRetry(request);
  }
}
