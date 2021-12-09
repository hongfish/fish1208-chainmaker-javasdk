package com.fish1208.controller;

import com.fish1208.bean.HelloWorld;
import com.fish1208.common.response.Result;
import lombok.extern.slf4j.Slf4j;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

/**
 * HelloWorld合约(wasm版)控制器
 */
@Slf4j
@RestController
@RequestMapping("/helloworld")
public class HelloWorldController {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String CONTRACT_NAME = "helloworld";

    @Autowired
    private ChainClient chainClient;

    @PostMapping(value = "/set")
    public Result<?> set(@RequestBody HelloWorld hello) {

        Map<String, byte[]> params = new HashMap<>();
        params.put("n",  hello.getN().getBytes());
        String method = "set";

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/get")
    public Result<?> get() {

        String method = "get";
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(CONTRACT_NAME, method, null, null, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(responseInfo.getContractResult().getResult().toString());
    }

}
