package com.fish1208.controller;

import com.fish1208.common.response.Result;
import lombok.extern.slf4j.Slf4j;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.chainmaker.sdk.utils.CryptoUtils;
import org.chainmaker.sdk.utils.Utils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.web3j.abi.FunctionEncoder;
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Function;
import org.web3j.abi.datatypes.generated.Uint256;
import org.web3j.utils.Numeric;

import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

@Slf4j
@RestController
@RequestMapping("/balance001")
public class LedgerBalanceController {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String CONTRACT_NAME = "balance001";
    private static final String CONTRACT_ARGS_EVM_PARAM = "data";

    @Autowired
    private ChainClient chainClient;

    @GetMapping(value = "/updateMyBalance")
    public Result<?> updateMyBalance(@RequestParam Integer balance) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();

        Function function = new Function( "updateMyBalance" , Arrays.asList(new Uint256(balance)), Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/transfer")
    public Result<?> transfer(@RequestParam String to, @RequestParam Integer amount) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function( "transfer" , Arrays.asList(new Address(to), new Uint256(BigInteger.valueOf(amount))),
                Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params,rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/updateBalance")
    public Result<?> updateBalance(@RequestParam Integer balance, @RequestParam String to) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();

        Function function = new Function( "updateBalance" , Arrays.asList(new Uint256(balance), new Address(to)), Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/balances")
    public Result<?> balances( String addr) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();

        Function function = new Function( "balances" , Arrays.asList(new Address(addr)), Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(CONTRACT_NAME, method, null, params, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(Numeric.toBigInt(responseInfo.getContractResult().getResult().toByteArray()));
    }

}
