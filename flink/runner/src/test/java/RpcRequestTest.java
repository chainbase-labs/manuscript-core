package com.chainbase.udf;
import java.io.IOException;
import org.junit.jupiter.api.Test;

public class RpcRequestTest {

    private final RpcRequest rpcRequest;

    public RpcRequestTest() throws Exception {
        this.rpcRequest = new RpcRequest();
        this.rpcRequest.open(null);
    }

    @Test
    public void testLatestBlockNumber_whenReturnString() {
        Object result = rpcRequest.eval(
                "https://rpc.ankr.com/eth",
                "eth_blockNumber");
        System.out.println(result);
    }

    @Test
    void getTransactionByBlockHashAndIndex_whenReturnJson() throws IOException {
        Object result = rpcRequest.eval("https://rpc.ankr.com/eth",
                "eth_getTransactionByBlockHashAndIndex",
                "0x829df9bb801fc0494abf2f443423a49ffa32964554db71b098d332d87b70a48b", "0x0");

        System.out.println(result.toString());
    }

    @Test
    void getBlockByNumberWithLatest_whenReturnJson() throws IOException {
        Object result = rpcRequest.eval("https://rpc.ankr.com/eth",
                "eth_getBlockByNumber",
                "8418440", false);

        System.out.println(result.toString());
    }

    @Test
    void getBlockByNumberWithBlockNumber_whenReturnJson() throws IOException {
        Object result = rpcRequest.eval("https://rpc.ankr.com/eth",
                "eth_getBlockByNumber",
                "0x11dbbfb", false);

        System.out.println(result.toString());
    }

    @Test
    void getEthCode_whenReturnJson() throws IOException {
        Object result = rpcRequest.eval("https://rpc.ankr.com/eth",
                "eth_getCode",
                "0xdeaddeaddeaddeaddeaddeaddeaddeaddead0000", "latest");

        System.out.println(result.toString());
    }

}