package com.fish1208.controller.erc20.input;

import lombok.Data;

@Data
public class TransferFromRequest {

    private String fromAccount;

    private String toAccount;

    private Integer amount;

}
