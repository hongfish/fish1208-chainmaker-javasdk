package com.fish1208.controller;

import com.fish1208.common.response.Result;
import com.google.protobuf.ByteString;
import lombok.extern.slf4j.Slf4j;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.web3j.abi.FunctionEncoder;
import org.web3j.abi.datatypes.Function;
import org.web3j.utils.Numeric;
import org.web3j.utils.Strings;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

@Slf4j
@RestController
@RequestMapping("/erc20")
public class ERC20Controller {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String CONTRACT_NAME = "erc20";
    private static final String CONTRACT_ARGS_EVM_PARAM = "data";

    @Autowired
    private ChainClient chainClient;

    @GetMapping(value = "/name")
    public Result<?> balances() throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();

        Function function = new Function( "name" , Collections.emptyList(), Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(CONTRACT_NAME, method, null, params, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(responseInfo.getContractResult().getResult().toStringUtf8());
    }

}
