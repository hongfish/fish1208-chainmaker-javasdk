/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import org.bouncycastle.util.encoders.Hex;
import org.chainmaker.pb.common.ChainmakerBlock;
import org.chainmaker.pb.common.ChainmakerTransaction;
import org.chainmaker.pb.config.ChainConfigOuterClass;
import org.chainmaker.pb.discovery.Discovery;
import org.chainmaker.pb.store.Store;
import org.junit.Assert;
import org.junit.Test;

public class TestSystemContract extends TestBase {
    private static final String TX_ID = "3b7d53728ba473ac0b37d0fcde7f9cdd316f255ad8eda5855db7164ae85067ff";
    private static final String BLOCK_HASH = "022491de8c66cfd9916ecb1ca5c5c0c1c9aaa1516345065d6dc47120592987cd";
    @Test
    public void testGetTxByTxId() {
        ChainmakerTransaction.TransactionInfo response = null;
        try {
            response = chainClient.getTxByTxId(TX_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(response);
    }

    @Test
    public void testGetBlockByHeight() {

        ChainmakerBlock.BlockInfo blockInfo = null;
        try {
            blockInfo = chainClient.getBlockByHeight(11, false, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockInfo);
    }

    @Test
    public void testGetBlockByHash() {
        //BLOCK_HASH的计算方式为Hex.toHexString(blockInfo.getBlock().getHeader().getBlockHash().toByteArray())
        ChainmakerBlock.BlockInfo blockInfo = null;
        try {
            blockInfo = chainClient.getBlockByHash(BLOCK_HASH, false, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockInfo);
    }

    @Test
    public void testGetBlockByTxId() {
        ChainmakerBlock.BlockInfo blockInfo = null;
        try {
            blockInfo = chainClient.getBlockByTxId(TX_ID, false, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        System.out.println(Hex.toHexString(blockInfo.getBlock().getHeader().getBlockHash().toByteArray()));
        Assert.assertNotNull(blockInfo);
    }

    @Test
    public void testGetLastConfigBlock() {
        ChainmakerBlock.BlockInfo blockInfo = null;
        try {
            blockInfo = chainClient.getLastConfigBlock(false, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockInfo);
    }

    @Test
    public void testGetNodeChainList() {
        Discovery.ChainList chainList = null;
        try {
            chainList = chainClient.getNodeChainList(rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(chainList);
    }

    @Test
    public void testGetChainInfo() {
        Discovery.ChainInfo chainInfo = null;
        try {
            chainInfo = chainClient.getChainInfo(rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(chainInfo);
    }

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
    public void testEnableCertHash() {
        boolean success = false;
        try {
            success = chainClient.enableCertHash();
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertTrue(success);
    }

    @Test
    public void testGetBlockHeightByTxId() {
        long blockheight = 0;
        try {
            blockheight = chainClient.getBlockHeightByTxId(TX_ID, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockheight);
    }

    @Test
    public void testGetBlockHeightByBlockHash() {
        long blockheight = 0;
        try {
            blockheight = chainClient.getBlockHeightByBlockHash(BLOCK_HASH, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockheight);
    }

    @Test
    public void tetsGetFullBlockByHeight() {
        Store.BlockWithRWSet fullBlock = null;
        try {
            fullBlock = chainClient.getFullBlockByHeight(1, rpcCallTimeout);
        } catch (SdkException e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(fullBlock);
    }

    @Test
    public void tetsGetLatestBlock() {
        ChainmakerBlock.BlockInfo blockInfo = null;
        try {
            blockInfo = chainClient.getLastBlock(false, rpcCallTimeout);
        } catch (SdkException e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
        Assert.assertNotNull(blockInfo);
    }

    @Test
    public void testGetCurrentBlockHeight() {
        long blockHeight = 0;
        try {
            blockHeight = chainClient.getCurrentBlockHeight(rpcCallTimeout);
        } catch (SdkException e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(blockHeight);
    }

    @Test
    public void testGetBlockHeaderByHeight() {
        ChainmakerBlock.BlockHeader blockHeader = null;
        try {
            blockHeader = chainClient.getBlockHeaderByHeight(2, rpcCallTimeout);
        } catch (SdkException e) {
            Assert.fail(e.getMessage());
            e.printStackTrace();
        }
        Assert.assertNotNull(blockHeader);
    }

}
