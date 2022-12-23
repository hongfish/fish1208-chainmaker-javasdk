package com.fish1208.controller.erc20.input;

import lombok.Data;

@Data
public class TransferRequest {

    private String to;

    private Integer amount;

}
