/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import org.bouncycastle.pqc.math.linearalgebra.ByteUtils;
import org.bouncycastle.util.encoders.Hex;
import org.chainmaker.pb.common.Request;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.sdk.utils.FileUtils;
import org.chainmaker.sdk.utils.SdkUtils;
import org.junit.Assert;
import org.junit.Test;


public class TestBaseCertManage extends TestBase {

    @Test
    public void testAddCert() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.addCert(rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        System.out.println(Hex.toHexString(responseInfo.getContractResult().getResult().toByteArray()));
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testDeleteCert() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            //certHash获取方法见testAddCert
            chainClient.enableCertHash();
            String[] certHashes = new String[]{ByteUtils.toHexString(chainClient.getClientUser().getCertHash())};
            Request.Payload payload = chainClient.createCertDeletePayload(certHashes);
            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

            responseInfo = chainClient.deleteCert(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);

        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testQueryCert() {

        ResultOuterClass.CertInfos certInfos = null;
        try {
            chainClient.enableCertHash();
            String[] certHashes = new String[]{ByteUtils.toHexString(chainClient.getClientUser().getCertHash())};
            certInfos = chainClient.queryCert(certHashes,rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(certInfos);
    }

    @Test
    public void testfreezeCerts() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            chainClient.enableCertHash();
            String[] certHashes = new String[]{new String(FileUtils.getResourceFileBytes(CLIENT1_CERT_PATH))};
            Request.Payload payload = chainClient.createCertFreezePayload(certHashes);

            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

            responseInfo = chainClient.freezeCerts(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);

        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testUnfreezeCerts() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            chainClient.enableCertHash();
            String[] certHashes = new String[]{new String(FileUtils.getResourceFileBytes(CLIENT1_CERT_PATH))};
            Request.Payload payload = chainClient.createPayloadOfUnfreezeCerts(certHashes);

            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

            responseInfo = chainClient.unfreezeCerts(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }

    @Test
    public void testRevokeCerts() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            chainClient.enableCertHash();
            Request.Payload payload = chainClient.createPayloadOfRevokeCerts(new String(FileUtils.getResourceFileBytes(CLIENT_CRL_PATH)));

            Request.EndorsementEntry[] endorsementEntries = SdkUtils.getEndorsers(payload, new User[]{adminUser1, adminUser2, adminUser3});

            responseInfo = chainClient.revokeCerts(payload, endorsementEntries, rpcCallTimeout, syncResultTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(responseInfo);
    }
}
