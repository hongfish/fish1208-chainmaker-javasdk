/*
Copyright (C) THL A29 Limited, a Tencent company. All rights reserved.

SPDX-License-Identifier: Apache-2.0
*/
package org.chainmaker.sdk;

import com.google.protobuf.InvalidProtocolBufferException;
import io.grpc.stub.StreamObserver;
import org.chainmaker.pb.common.ChainmakerBlock;
import org.chainmaker.pb.common.ChainmakerTransaction;
import org.chainmaker.pb.common.ResultOuterClass;
import org.junit.Assert;
import org.junit.Test;

public class TestSubscribe extends TestBase {

    private static final boolean ONLY_HEADER = false;

    @Test
    public void testSubscribeBlock() {
        StreamObserver<ResultOuterClass.SubscribeResult> responseObserver = new StreamObserver<ResultOuterClass.SubscribeResult>() {
            @Override
            public void onNext(ResultOuterClass.SubscribeResult result) {
                try {
                    if (ONLY_HEADER) {
                        ChainmakerBlock.BlockHeader blockHeader = ChainmakerBlock.BlockHeader.parseFrom(result.getData());
                        Assert.assertNotNull(blockHeader);
                    } else {
                        ChainmakerBlock.BlockInfo blockInfo = ChainmakerBlock.BlockInfo.parseFrom(result.getData());
                        Assert.assertNotNull(blockInfo);
                    }
                } catch (InvalidProtocolBufferException e) {
                    e.printStackTrace();
                    Assert.fail(e.getMessage());
                }
            }

            @Override
            public void onError(Throwable throwable) {
                // just do nothing
            }

            @Override
            public void onCompleted() {
                // just do nothing
            }
        };
        try {
            chainClient.subscribeBlock(0, -1, true, ONLY_HEADER, responseObserver);
            Thread.sleep(500000000);
        } catch (Exception e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
    }

    @Test
    public void testSubscribeTx() {
        StreamObserver<ResultOuterClass.SubscribeResult> responseObserver = new StreamObserver<ResultOuterClass.SubscribeResult>() {
            @Override
            public void onNext(ResultOuterClass.SubscribeResult result) {
                try {
                    ChainmakerTransaction.Transaction transactionInfo = ChainmakerTransaction.Transaction.parseFrom(result.getData());
                    Assert.assertNotNull(transactionInfo);
                } catch (Exception e) {
                    e.printStackTrace();
                    Assert.fail(e.getMessage());
                }
            }

            @Override
            public void onError(Throwable throwable) {
                // can add log here
            }

            @Override
            public void onCompleted() {
                // can add log here
            }
        };
        try {
            chainClient.subscribeTx(0, -1, "", new String[]{}, responseObserver);
            Thread.sleep(500000000);
        } catch (Exception e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
    }

    @Test
    public void testSubscribeContractEvent() {
        StreamObserver<ResultOuterClass.SubscribeResult> responseObserver = new StreamObserver<ResultOuterClass.SubscribeResult>() {
            @Override
            public void onNext(ResultOuterClass.SubscribeResult result) {
                try {
                    ResultOuterClass.ContractEventInfoList contract = ResultOuterClass.ContractEventInfoList.parseFrom(result.getData());
                    Assert.assertNotNull(contract);
                } catch (Exception e) {
                    e.printStackTrace();
                    Assert.fail(e.getMessage());
                }
            }

            @Override
            public void onError(Throwable throwable) {
                // can add log here
            }

            @Override
            public void onCompleted() {
                // can add log here
            }
        };
        try {
            chainClient.subscribeContractEvent("topic_vx", "asset", responseObserver);
            Thread.sleep(500000000);
        } catch (Exception e) {
            e.printStackTrace();
            Assert.fail(e.getMessage());
        }
    }

}
