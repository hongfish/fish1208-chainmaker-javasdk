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
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Function;
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
 *Chain控制器
 */
@Slf4j
@RestController
@RequestMapping("/evm/contract")
public class EvmContractController {

    private static long rpcCallTimeout = 10000;
    private static long syncResultTimeout = 10000;

    private static final String EVM_CONTRACT_FILE_PATH = "contract/token.bin";
    private static final String CONTRACT_NAME = "token";
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

    @GetMapping(value = "/create")
    public Result<?> create() throws IOException, SdkException{

        makeAddrFromCert();
        //创建合约构造参数扽RLP编码值
        Function function = new Function( "" , Arrays.asList(new Address(ADDRESS)),
                Collections.emptyList());
        String methodDataStr = FunctionEncoder.encode(function);

        Map<String, byte[]> paramMap = new HashMap<>();
        paramMap.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.substring(10).getBytes());


        ResultOuterClass.TxResponse responseInfo = null;

        byte[] byteCode = IOUtils.toByteArray(new FileInputStream(EVM_CONTRACT_FILE_PATH));
        // 1. create payload
        Request.Payload payload = chainClient.createContractCreatePayload(Utils.calcContractName(CONTRACT_NAME),
                "1", Hex.decode(new String(byteCode)),
                ContractOuterClass.RuntimeType.EVM, paramMap);

        //2. create payloads with endorsement
        Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

        // 3. send request
        responseInfo = chainClient.sendContractManageRequest(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/transfer")
    public Result<?> transfer(@RequestParam Integer amount) throws IOException, SdkException{

        Map<String, byte[]> params = new HashMap<>();
        String toAddress = CryptoUtils.makeAddrFromCert(adminUser2.getTlsCertificate());
        log.info("transfer, toAddress={}", toAddress);
        BigInteger value = BigInteger.valueOf(amount);
        Function function = new Function( "transfer" , Arrays.asList(new Address(toAddress), new Uint256(value)),
                Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(Utils.calcContractName(CONTRACT_NAME),
                    method, null, params,rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }

        return Result.data(responseInfo.getCode());
    }

    @GetMapping(value = "/balanceOf")
    public Result<?> balanceOf() throws IOException, SdkException{
        makeAddrFromCert();
        Map<String, byte[]> params = new HashMap<>();

        String toAddress = CryptoUtils.makeAddrFromCert(adminUser2.getTlsCertificate());
        log.info("balanceOf, toAddress={}", toAddress);
        Function function = new Function( "balanceOf" , Arrays.asList(new Address(toAddress)),
                Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(Utils.calcContractName(CONTRACT_NAME),
                    method, null, params,rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
        }
        return Result.data(Numeric.toBigInt(responseInfo.getContractResult().getResult().toByteArray()));
    }

}
