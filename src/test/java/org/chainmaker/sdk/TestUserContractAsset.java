/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import org.chainmaker.pb.common.ContractOuterClass;
import org.chainmaker.pb.common.Request;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.utils.FileUtils;
import org.chainmaker.sdk.utils.SdkUtils;
import org.junit.Assert;
import org.junit.Test;

import java.util.HashMap;
import java.util.Map;

public class TestUserContractAsset extends TestBase {

    private static final String QUERY_CONTRACT_METHOD_BALANCE = "balance_of";
    private static final String QUERY_CONTRACT_METHOD_ADDR = "query_address";
    private static final String INVOKE_CONTRACT_METHOD = "issue_amount";
    private static final String CONTRACT_NAME = "asset";
    private static final String CONTRACT_FILE_PATH = "rust-asset-management-1.0.0.wasm";
    private static final String CONTRACT_ARGS_ISSUE_LIMIT = "issue_limit";
    private static final String CONTRACT_ARGS_TOTAL_SUPPLY = "total_supply";
    private static final String CONTRACT_ARGS_DEFAULT_AMOUNT = "100000000";

    @Test
    public void testCreateContract() {
        Map<String, byte[]> paramMap = new HashMap<>();
        paramMap.put(CONTRACT_ARGS_ISSUE_LIMIT, "1000".getBytes());
        paramMap.put(CONTRACT_ARGS_TOTAL_SUPPLY, CONTRACT_ARGS_DEFAULT_AMOUNT.getBytes());
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            byte[] byteCode = FileUtils.getResourceFileBytes(CONTRACT_FILE_PATH);

            // 1. create payload
            Request.Payload payload = chainClient.createContractCreatePayload(CONTRACT_NAME, "1", byteCode,
                    ContractOuterClass.RuntimeType.WASMER, paramMap);

            //2. create payloads with endorsement
            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

            // 3. send request
            responseInfo = chainClient.sendContractManageRequest(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testInvokeContract() {
        Map<String, byte[]> paramMap = new HashMap<>();
        paramMap.put("amount", "1000".getBytes());
        paramMap.put("to", "cb7fd709631cdf6e82f071c066fee8c9ec77d3425207d5421e378d142521799f".getBytes());
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.invokeContract(CONTRACT_NAME, INVOKE_CONTRACT_METHOD,
                    null, paramMap, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testQueryContractAddr() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(CONTRACT_NAME, QUERY_CONTRACT_METHOD_ADDR, null,  null, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testQueryContractBalance() {
        Map<String, byte[]> paramMap = new HashMap<>();
        paramMap.put("owner", "cb7fd709631cdf6e82f071c066fee8c9ec77d3425207d5421e378d142521799f".getBytes());
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.queryContract(CONTRACT_NAME,
                    QUERY_CONTRACT_METHOD_BALANCE, null,  paramMap, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }
}
