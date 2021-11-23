package com.fish1208.controller;

import com.fish1208.chainmaker.entity.BlockEntity;
import com.fish1208.common.response.Result;
import lombok.extern.slf4j.Slf4j;
import org.chainmaker.pb.common.ChainmakerBlock;
import org.chainmaker.pb.config.ChainConfigOuterClass;
import org.chainmaker.sdk.ChainClient;
import org.chainmaker.sdk.SdkException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

/**
 *Chain控制器
 */
@Slf4j
@RestController
@RequestMapping("/chain")
public class ChainController {

    private long rpcCallTimeout = 10000;

    @Autowired
    private ChainClient chainClient;

    @GetMapping(value = "/getBlockInfo")
    public Result<?> getBlockInfo(@RequestParam Long blockHeight) throws SdkException{

        ChainmakerBlock.BlockInfo blockInfo = chainClient.getBlockByHeight(blockHeight, true, rpcCallTimeout);
        ChainmakerBlock.BlockHeader blockHeader = blockInfo.getBlock().getHeader();
        return Result.data(blockHeader.getBlockTimestamp());
    }

}
