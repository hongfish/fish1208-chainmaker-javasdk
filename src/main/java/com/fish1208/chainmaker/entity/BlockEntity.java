package com.fish1208.chainmaker.entity;

import lombok.Data;

import java.math.BigInteger;

@Data
public class BlockEntity {
    private BigInteger blockNumber; //区块序号
    private String blockHash;//区块hash
    private String parentHash;//上个区块hash
    private String tranHash;//交易hash
    private String blockTime;//区块产生时间
    private Integer envelopeCount; //交易数
}
