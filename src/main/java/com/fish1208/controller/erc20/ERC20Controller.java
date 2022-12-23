package com.fish1208.controller.erc20;

import cn.hutool.core.util.StrUtil;
import com.fish1208.common.response.Result;
import com.fish1208.controller.erc20.input.BalanceOfOwnerRequest;
import com.fish1208.controller.erc20.input.TransferFromRequest;
import com.fish1208.controller.erc20.input.TransferRequest;
import com.google.protobuf.ByteString;
import lombok.extern.slf4j.Slf4j;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;
import org.web3j.abi.FunctionEncoder;
import org.web3j.abi.datatypes.Address;
import org.web3j.abi.datatypes.Function;
import org.web3j.abi.datatypes.generated.Uint256;
import org.web3j.utils.Numeric;
import org.web3j.utils.Strings;

import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
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

    @PostMapping(value = "/transfer")
    public Result<?> transfer(@RequestBody TransferRequest input) throws SdkException{
        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function( "transfer" ,
                Arrays.asList(new Address(input.getTo()), new Uint256(BigInteger.valueOf(input.getAmount()))),
                Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params,rpcCallTimeout, syncResultTimeout);
        return Result.data(responseInfo.getCode());
    }

    @PostMapping(value = "/transferFrom")
    public Result<?> transferFrom(@RequestBody TransferFromRequest input) throws SdkException {

        if(approve(input.getFromAccount(), input.getAmount())){
            Map<String, byte[]> params = new HashMap<>();
            Function function = new Function( "transferFrom" ,
                    Arrays.asList(new Address(input.getFromAccount()), new Address(input.getToAccount()), new Uint256(BigInteger.valueOf(input.getAmount()))),
                    Collections.emptyList());

            String methodDataStr = FunctionEncoder.encode(function);
            String method = methodDataStr.substring(0,10);
            params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

            ResultOuterClass.TxResponse responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params,rpcCallTimeout, syncResultTimeout);
            return Result.data(responseInfo.getCode());
        }
        return Result.fail("授权失败");
    }

    private boolean approve(String fromAccount, Integer amount) throws SdkException {
        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function( "approve" ,
                Arrays.asList(new Address(fromAccount), new Uint256(BigInteger.valueOf(amount))),
                Collections.emptyList());

        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = chainClient.invokeContract(CONTRACT_NAME, method, null, params,rpcCallTimeout, syncResultTimeout);
        return StrUtil.equals(responseInfo.getCode().toString(), "SUCCESS");
    }

    @GetMapping(value = "/balance")
    public Result<?> balanceOfOwner(BalanceOfOwnerRequest input) throws SdkException {
        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function( "balanceOfOwner" , Arrays.asList(new Address(input.getAccount())), Collections.emptyList());
        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = chainClient.queryContract(CONTRACT_NAME, method, null, params, rpcCallTimeout);
        return Result.data(Numeric.toBigInt(responseInfo.getContractResult().getResult().toByteArray()));
    }

    @GetMapping(value = "/name")
    public Result<?> balances() throws SdkException {
        Map<String, byte[]> params = new HashMap<>();
        Function function = new Function( "name" , Collections.emptyList(), Collections.emptyList());
        String methodDataStr = FunctionEncoder.encode(function);
        String method = methodDataStr.substring(0,10);
        params.put(CONTRACT_ARGS_EVM_PARAM, methodDataStr.getBytes());

        ResultOuterClass.TxResponse responseInfo = chainClient.queryContract(CONTRACT_NAME, method, null, params, rpcCallTimeout);
        return Result.data(responseInfo.getContractResult().getResult().toStringUtf8());
    }

}
