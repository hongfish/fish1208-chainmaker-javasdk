package com.fish1208.controller;

import com.fish1208.common.response.Result;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.io.IOUtils;
import org.bouncycastle.util.encoders.Hex;
import org.chainmaker.pb.common.ContractOuterClass;
import org.chainmaker.pb.common.Request;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.chainmaker.sdk.User;
import org.chainmaker.sdk.utils.CryptoUtils;
import org.chainmaker.sdk.utils.SdkUtils;
import org.chainmaker.sdk.utils.Utils;
import org.chainmaker.sdk.utils.UtilsException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.web3j.abi.FunctionEncoder;
import org.web3j.abi.TypeReference;
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Function;
import org.web3j.abi.datatypes.Type;
import org.web3j.abi.datatypes.Utf8String;
import org.web3j.abi.datatypes.generated.Uint256;
import org.web3j.utils.Numeric;

import java.io.FileInputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

/**
 * HelloWorld合约控制器
 */
@Slf4j
@RestController
@RequestMapping("/helloworld")
public class HelloWorldController {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String CONTRACT_NAME = "helloworld02";
    private static final String CONTRACT_ARGS_EVM_PARAM = "data";
    private static String ADDRESS = "";

    @Autowired
    private ChainClient chainClient;

    @Autowired
    @Qualifier("adminUser1")
    private User adminUser1;

    @Autowired
    @Qualifier("adminUser2")
    private User adminUser2;

    @Autowired
    @Qualifier("adminUser3")
    private User adminUser3;

    private void makeAddrFromCert() {
        try {
            ADDRESS = CryptoUtils.makeAddrFromCert(chainClient.getClientUser().getTlsCertificate());
            log.info("ADDRESS={}", ADDRESS);
        } catch (UtilsException e) {
            e.printStackTrace();
        }
    }

    @GetMapping(value = "/set")
    public Result<?> set(@RequestParam String n) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function("set", Arrays.asList(new Utf8String(n)), Collections.emptyList());
//        Function function = new Function( "set", Arrays.asList(), Collections.emptyList());
        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(Utils.calcContractName(CONTRACT_NAME),
                    method, null, params, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/get")
    public Result<?> get() throws IOException, SdkException{
        makeAddrFromCert();
        Map<String, byte[]> params = new HashMap<>();

        Function function = new Function("get", Arrays.asList(), Arrays.asList(new TypeReference<Utf8String>() {}));
//        Function function = new Function( "get", Arrays.asList(), Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(Utils.calcContractName(CONTRACT_NAME),
                    method, null, params,rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(responseInfo.getContractResult().getResult());
    }

}
