/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context;
use sim_context::*;

// 安装合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn init_contract() {
    // 安装时的业务逻辑，内容可为空
    sim_context::log("init_contract");
}

// 升级合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn upgrade() {
    // 升级时的业务逻辑，内容可为空
    sim_context::log("upgrade");
    let ctx = &mut sim_context::get_sim_context();
    ctx.ok("upgrade success".as_bytes());
}

#[no_mangle]
pub extern "C" fn set() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();

    // 获取传入参数
    let n = ctx.arg_as_utf8_str("n");

    // 校验参数
    if n.len() == 0 {
        ctx.log("n is null");
        ctx.ok("".as_bytes());
        return;
    }

    let mut ec = EasyCodec::new();
    ec.add_string("n", n.as_str());

    // 存储
    ctx.put_state(
        "key_001",
        "n",
        ec.marshal().as_slice(),
    );
}

#[no_mangle]
pub extern "C" fn get() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();

    // 查询
    let r = ctx.get_state("key_001","n");

    // 校验返回结果
    if r.is_err() {
        ctx.log("get_state fail");
        ctx.error("get_state fail");
        return;
    }

    let n_vec = r.unwrap();
    if n_vec.len() == 0 {
        ctx.log("None");
        ctx.ok("Hello World".as_bytes());
        return;
    }

    // 查询
    let r = ctx.get_state("key_001","n").unwrap();
    let ec = EasyCodec::new_with_bytes(&r);

    let json_str = ec.to_json();

    // 返回查询结果
    ctx.ok(json_str.as_bytes());
    ctx.log(&json_str);
}
