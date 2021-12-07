package com.fish1208.controller;

import com.fish1208.bean.HelloWorld;
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
import org.springframework.web.bind.annotation.*;
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
 * HelloWorld合约(wasm版)控制器
 */
@Slf4j
@RestController
@RequestMapping("/helloworld")
public class HelloWorldController {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String CONTRACT_NAME = "helloworld";
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

    @PostMapping(value = "/set")
    public Result<?> set(@RequestBody HelloWorld hello) throws IOException, SdkException{
//        makeAddrFromCert();

        Map<String, byte[]> params = new HashMap<>();
        params.put("n",  hello.getN().getBytes());
        String method = "set";

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME,
                    method, null, params, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/get")
    public Result<?> get() throws IOException, SdkException{
//        makeAddrFromCert();
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
