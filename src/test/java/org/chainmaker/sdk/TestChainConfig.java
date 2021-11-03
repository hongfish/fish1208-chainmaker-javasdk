/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import org.chainmaker.pb.accesscontrol.PolicyOuterClass;
import org.chainmaker.pb.common.Request;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.pb.config.ChainConfigOuterClass;
import org.chainmaker.sdk.utils.FileUtils;
import org.chainmaker.sdk.utils.SdkUtils;
import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;

import java.util.HashMap;
import java.util.Map;

public class TestChainConfig extends TestBase {

    private final String ORG_ID = "wx-org1";
    private final String NODE_ID = "QmQVkTSF6aWzRSddT3rro6Ve33jhKpsHFaQoVxHKMWzhuN";

    @Test
    public void testGetChainConfig() {
        ChainConfigOuterClass.ChainConfig chainConfig = null;
        try {
            chainConfig = chainClient.getChainConfig(rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(chainConfig.toString());
    }

    @Test
    public void testGetChainConfigByBlockHeight() {
        ChainConfigOuterClass.ChainConfig chainConfig = null;
        try {
            chainConfig = chainClient.getChainConfigByBlockHeight(10, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(chainConfig.toString());
    }

    @Test
    public void testGetChainConfigSequence() {
        long sequence = 0;
        try {
            sequence = chainClient.getChainConfigSequence(20000);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotEquals(-1, sequence);
    }

    @Test
    public void testCreatePayloadOfChainConfigCoreUpdate() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigCoreUpdate(20,30, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigBlockUpdate() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigBlockUpdate(false,
                    800, 1000, 1000, 5000, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigTrustRootAdd() {
        Request.Payload payload = null;

        try {
            String[] certList = new String[]{new String(FileUtils.getResourceFileBytes(ADMIN2_CERT_PATH))};
            payload = chainClient.createPayloadOfChainConfigTrustRootAdd(ORG_ID, certList, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigTrustRootUpdate() {
        Request.Payload payload = null;
        try {
            String[] certList = new String[]{new String(FileUtils.getResourceFileBytes(ADMIN2_CERT_PATH))};
            payload = chainClient.createPayloadOfChainConfigTrustRootUpdate(ORG_ID, certList, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigTrustRootDelete() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigTrustRootDelete(ORG_ID,rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigPermissionAdd() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigPermissionAdd(ORG_ID,
                    PolicyOuterClass.Policy.getDefaultInstance(), rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigPermissionUpdate() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigPermissionUpdate(ORG_ID,
                    PolicyOuterClass.Policy.getDefaultInstance(), rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigPermissionDelete() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigPermissionDelete(ORG_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusNodeAddrAdd() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeAddrAdd(ORG_ID,
                    new String[]{NODE_ID}, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusNodeAddrUpdate() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeAddrUpdate(ORG_ID,
                    NODE_ID, NODE_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusNodeAddrDelete() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeAddrDelete(ORG_ID,
                    NODE_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusNodeOrgAdd() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeOrgAdd(ORG_ID,
                    new String[]{NODE_ID}, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusNodeOrgUpdate() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeOrgUpdate(ORG_ID,
                    new String[]{NODE_ID}, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreateChainConfigConsensusNodeOrgDeletePayload() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusNodeOrgDelete(ORG_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusExtAdd() {
        Map<String, byte[]> params = new HashMap<>();
        params.put("aaaa", "bbbb".getBytes());
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusExtAdd(params, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusExtUpdatePayload() {
        Map<String, byte[]> params = new HashMap<>();
        params.put("aaaa", "bbbb".getBytes());
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusExtUpdate(params, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testCreatePayloadOfChainConfigConsensusExtDelete() {
        Request.Payload payload = null;
        try {
            payload = chainClient.createPayloadOfChainConfigConsensusExtDelete(new String[]{"aaaa"}, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(payload);
    }

    @Test
    public void testUpdateChainConfig() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            Request.Payload payload = chainClient.createPayloadOfChainConfigBlockUpdate(false,
                    9001, 200, 225, 2000, rpcCallTimeout);
            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload,
                    new User[]{adminUser1, adminUser2, adminUser3});
            responseInfo = chainClient.updateChainConfig(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }
}
