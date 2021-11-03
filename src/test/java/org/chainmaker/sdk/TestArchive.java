/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import org.bouncycastle.util.encoders.Hex;
import org.chainmaker.pb.common.ChainmakerBlock;
import org.chainmaker.pb.common.ChainmakerTransaction;
import org.chainmaker.pb.common.Request;
import org.chainmaker.pb.common.ResultOuterClass;
import org.chainmaker.pb.store.Store;
import org.chainmaker.sdk.crypto.ChainMakerCryptoSuiteException;
import org.junit.Assert;
import org.junit.Test;


public class TestArchive extends TestBase {

    // 风险提示!!!!,本方法是删除区块，使用前需用归档工具将对应区块归档!!!
    // 数据归档
    @Test
    public void testArchive() throws ChainMakerCryptoSuiteException {
        Request.Payload payload = chainClient.createArchiveBlockPayload(3);
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            responseInfo = chainClient.sendArchiveBlockRequest(payload, rpcCallTimeout);
        } catch (Exception e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(responseInfo.toString());
    }

    //归档恢复
    @Test
    public void testRestore() {
        ResultOuterClass.TxResponse responseInfo = null;
        try {
            Store.BlockWithRWSet fullBlock = chainClient.getArchivedFullBlockByHeight(2);
            Request.Payload payload = chainClient.createRestoreBlockPayload(fullBlock.toByteArray());
            responseInfo = chainClient.sendRestoreBlockRequest(payload, rpcCallTimeout);
        } catch (Exception e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(responseInfo.toString());
    }

    //归档查询
    @Test
    public void testGetFromArchiveStore() {
        Store.BlockWithRWSet fullBlock = null;
        ChainmakerBlock.BlockInfo blockInfo = null;
        ChainmakerTransaction.TransactionInfo transactionInfo = null;

        try {
            fullBlock = chainClient.getFromArchiveStore(2);
            Assert.assertNotNull(fullBlock);

            fullBlock = chainClient.getArchivedFullBlockByHeight(2);
            Assert.assertNotNull(fullBlock);

            blockInfo = chainClient.getArchivedBlockByHeight(2, true);
            Assert.assertNotNull(blockInfo);

            blockInfo = chainClient.getArchivedBlockByHash(Hex.toHexString(blockInfo.getBlock().getHeader().getBlockHash().toByteArray()),
                    false, rpcCallTimeout);
            Assert.assertNotNull(blockInfo);

            String txid = blockInfo.getBlock().getTxs(0).getPayload().getTxId();
            transactionInfo = chainClient.getArchivedTxByTxId(txid, rpcCallTimeout);
        } catch (Exception e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(transactionInfo);
    }

}
