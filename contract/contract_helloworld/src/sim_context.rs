/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context_bulletproofs::{BulletproofsSimContext, BulletproofsSimContextImpl};
use crate::sim_context_paillier::{PaillierSimContext, PaillierSimContextImpl};
use crate::sim_context_rs;
use crate::vec_box;
use sim_context_rs::{ResultSet, SqlSimContext, SqlSimContextImpl};
use std::os::raw::c_void;
use std::slice;
use std::str;
use vec_box::VecBox;

extern "C" {
    pub fn sys_call(
        req_header_ptr: *const u8,
        req_header_len: usize,
        req_body_ptr: *const u8,
        req_body_len: usize,
    ) -> i32;

    fn log_message(pointer: *const u8, length: i32);
}

// special parameters passed to contract
const CONTRACT_PARAM_CREATOR_ORG_ID: &str = "__creator_org_id__";
const CONTRACT_PARAM_CREATOR_ROLE: &str = "__creator_role__";
const CONTRACT_PARAM_CREATOR_PK: &str = "__creator_pk__";
const CONTRACT_PARAM_SENDER_ORG_ID: &str = "__sender_org_id__";
const CONTRACT_PARAM_SENDER_ROLE: &str = "__sender_role__";
const CONTRACT_PARAM_SENDER_PK: &str = "__sender_pk__";
const CONTRACT_PARAM_BLOCK_HEIGHT: &str = "__block_height__";
const CONTRACT_PARAM_TX_ID: &str = "__tx_id__";
const CONTRACT_PARAM_CONTEXT_PTR: &str = "__context_ptr__";

// method name used by smart contract sdk;
const CONTRACT_METHOD_LOG_MESSAGE: &str = "LogMessage";
const CONTRACT_METHOD_GET_STATE_LEN: &str = "GetStateLen";
const CONTRACT_METHOD_GET_STATE: &str = "GetState";
const CONTRACT_METHOD_PUT_STATE: &str = "PutState";
const CONTRACT_METHOD_DELETE_STATE: &str = "DeleteState";
const CONTRACT_METHOD_SUCCESS_RESULT: &str = "SuccessResult";
const CONTRACT_METHOD_ERROR_RESULT: &str = "ErrorResult";
const CONTRACT_METHOD_CALL_CONTRACT: &str = "CallContract";
const CONTRACT_METHOD_CALL_CONTRACT_LEN: &str = "CallContractLen";
// emit event
const CONTRACT_METHOD_EMIT_EVENT: &str = "EmitEvent";
// iterator
const CONTRACT_METHOD_KV_ITERATOR: &str = "KvIterator";
const CONTRACT_METHOD_KV_PRE_ITERATOR: &str = "KvPreIterator";
const CONTRACT_METHOD_KV_ITERATOR_HAS_NEXT: &str = "KvIteratorHasNext";
const CONTRACT_METHOD_KV_ITERATOR_NEXT_LEN: &str = "KvIteratorNextLen";
const CONTRACT_METHOD_KV_ITERATOR_NEXT: &str = "KvIteratorNext";
const CONTRACT_METHOD_KV_ITERATOR_CLOSE: &str = "KvIteratorClose";

const CONTRACT_VERSION: &str = "v1.2.0";

// i32的大小 4字节
pub const I_32_LENGTH: usize = 4;

#[allow(non_camel_case_types)]
pub type result_code = i32;
pub const SUCCESS_CODE: result_code = 0;
pub const ERROR_CODE: result_code = 1;

#[no_mangle]
pub extern "C" fn runtime_type() -> i32 {
    let contract_runtime_type: i32 = 2;
    contract_runtime_type
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> i32 {
    let vb = VecBox::<u8>::get_instance(size);

    let vb_ref = vb.as_ref();
    let mut vb_obj = vb_ref.lock().unwrap();
    let old_len = vb_obj.len();
    if size != old_len {
        vb_obj.reset_data_size(size);
        // log(&format!("new len {}, old len {}", vb_obj.len(), old_len));
    }
    let data_par = vb_obj.data_ptr();
    data_par as i32
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void) {
    let vb = VecBox::<u8>::get_instance(pointer as usize);
    let vb_ref = vb.as_ref();
    let mut vb_obj = vb_ref.lock().unwrap();
    vb_obj.deallocated();
}

pub fn log(msg: &str) {
    unsafe { log_message(msg.as_ptr(), msg.len() as i32) }
}

/// SimContext is a interface with chainmaker interaction
pub trait SimContext {
    // common method
    fn call_contract(
        &self,
        contract_name: &str,
        method: &str,
        param: EasyCodec,
    ) -> Result<Vec<u8>, result_code>;
    fn ok(&self, value: &[u8]) -> result_code;
    fn error(&self, body: &str) -> result_code;
    fn log(&self, msg: &str);
    fn arg(&self, key: &str) -> Result<Vec<u8>, String>;
    fn arg_as_utf8_str(&self, key: &str) -> String;
    fn args(&self) -> &EasyCodec;
    fn get_creator_org_id(&self) -> String;
    fn get_creator_pub_key(&self) -> String;
    fn get_creator_role(&self) -> String;
    fn get_sender_org_id(&self) -> String;
    fn get_sender_pub_key(&self) -> String;
    fn get_sender_role(&self) -> String;
    fn get_block_height(&self) -> u64;
    fn get_tx_id(&self) -> String;
    fn emit_event(&mut self, topic: &str, data: &Vec<String>) -> result_code;
    // paillier
    fn get_paillier_sim_context(&self) -> Box<dyn PaillierSimContext>;
    // bulletproofs
    fn get_bulletproofs_sim_context(&self) -> Box<dyn BulletproofsSimContext>;
    // sql
    fn get_sql_sim_context(&self) -> Box<dyn SqlSimContext>;

    // KV method
    fn get_state(&self, key: &str, field: &str) -> Result<Vec<u8>, result_code>;
    fn get_state_from_key(&self, key: &str) -> Result<Vec<u8>, result_code>;
    fn put_state(&self, key: &str, field: &str, value: &[u8]) -> result_code;
    fn put_state_from_key(&self, key: &str, value: &[u8]) -> result_code;
    fn delete_state(&self, key: &str, field: &str) -> result_code;
    fn delete_state_from_key(&self, key: &str) -> result_code;

    /// new_iterator range of [startKey, limitKey), front closed back open
    fn new_iterator(
        &self,
        start_key: &str,
        limit_key: &str,
    ) -> Result<Box<dyn ResultSet>, result_code>;

    /// new_iterator_with_field range of [key+"#"+startField, key+"#"+limitField), front closed back open
    fn new_iterator_with_field(
        &self,
        key: &str,
        start_field: &str,
        limit_field: &str,
    ) -> Result<Box<dyn ResultSet>, result_code>;

    /// new_iterator_prefix_with_key_field range of [key+"#"+field, key+"#"+field], front closed back closed
    fn new_iterator_prefix_with_key_field(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Box<dyn ResultSet>, result_code>;

    /// new_iterator_prefix_with_key range of [key, key], front closed back closed
    fn new_iterator_prefix_with_key(&self, key: &str) -> Result<Box<dyn ResultSet>, result_code>;
}
pub struct SimContextImpl {
    pub ctx_ptr: i32,
    params: EasyCodec,
    pub common: CommonUtils,
}

/// helper method
pub struct CommonUtils {
    pub ctx_ptr: i32,
}
impl CommonUtils {
    /// allocate req manual memory management
    pub fn allocate(&self, capacity: usize) -> VecBox<u8> {
        VecBox::new(capacity)
    }
    /// get req header pointer and len
    pub fn get_req_header(&self, method: &str) -> Vec<u8> {
        let mut ec = EasyCodec::new();
        ec.add_val(
            EASY_KEY_TYPE_SYSTEM,
            "ctx_ptr",
            EASY_VALUE_TYPE_INT32,
            self.ctx_ptr.to_le_bytes().to_vec(),
        );
        ec.add_val(
            EASY_KEY_TYPE_SYSTEM,
            "version",
            EASY_VALUE_TYPE_STRING,
            CONTRACT_VERSION.as_bytes().to_vec(),
        );
        ec.add_val(
            EASY_KEY_TYPE_SYSTEM,
            "method",
            EASY_VALUE_TYPE_STRING,
            method.as_bytes().to_vec(),
        );
        ec.marshal()
    }

    /// 从链上获取bytes
    pub fn get_bytes_from_chain(
        &self,
        ec: &mut EasyCodec,
        method_len: &str,
        method: &str,
    ) -> Result<Vec<u8>, result_code> {
        // # get data len
        let mut i32_vec = self.allocate(I_32_LENGTH as usize);

        // ## header
        let req_header = self.get_req_header(method_len);
        let (req_header_ptr, req_header_len) = (req_header.as_ptr(), req_header.len());

        // ## body
        ec.add_i32("value_ptr", i32_vec.as_ptr_i32());
        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        // ## parse response
        if resp_code != SUCCESS_CODE {
            let msg = format!("{:?} error code {}", method_len, resp_code);
            log(&msg);
            i32_vec.drop();
            return Err(resp_code);
        }
        let data_length = mutu8_ptr_to_i32(i32_vec.as_mut_ptr());
        if data_length == 0 {
            return Ok(Vec::new());
        }
        i32_vec.drop();

        // # get data
        let mut value = self.allocate(data_length as usize);
        // ## header
        let req_header_str = self.get_req_header(method);
        let (req_header_ptr, req_header_len) = (req_header_str.as_ptr(), req_header_str.len());

        // ## body
        ec.remove("value_ptr");
        ec.add_i32("value_ptr", value.as_ptr_i32());
        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        // ## parse response
        if resp_code != SUCCESS_CODE {
            let msg = format!("{:?} error code {}", method, resp_code);
            log(msg.as_str());
            value.drop();
            return Err(resp_code);
        }

        // Read bytes of the value and return
        let vec: Vec<u8> =
            unsafe { slice::from_raw_parts(value.as_mut_ptr(), data_length as usize).to_vec() };

        value.drop();
        return Ok(vec);
    }

    /// 根据参数执行方法，从链上获取int32
    pub fn get_i32_from_chain(&self, ec: &mut EasyCodec, method: &str) -> Result<i32, result_code> {
        // # get ResultSet index
        let mut i32_vec = self.allocate(I_32_LENGTH as usize);

        // ## header
        let req_header = self.get_req_header(method);
        let (req_header_ptr, req_header_len) = (req_header.as_ptr(), req_header.len());

        // ## body
        ec.add_i32("value_ptr", i32_vec.as_ptr_i32());

        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };

        // ## parse response
        if resp_code != SUCCESS_CODE {
            let msg = format!("query_multi_row error code {}", resp_code);
            log(&msg);
            i32_vec.drop();
            return Err(resp_code);
        }

        let value = mutu8_ptr_to_i32(i32_vec.as_mut_ptr());
        i32_vec.drop();
        Ok(value)
    }
}

impl SimContextImpl {
    pub fn new(ctx_ptr: i32, params: EasyCodec) -> SimContextImpl {
        SimContextImpl {
            ctx_ptr,
            params,
            common: CommonUtils { ctx_ptr },
        }
    }

    fn new_iterator_core(
        &self,
        start_key: &str,
        start_field: &str,
        limit_key: &str,
        limit_field: &str,
        method: &str,
    ) -> Result<Box<dyn ResultSet>, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("start_key", start_key);
        ec.add_string("start_field", start_field);
        ec.add_string("limit_key", limit_key);
        ec.add_string("limit_field", limit_field);
        let r = self.common.get_i32_from_chain(ec, method);
        let index = match r {
            Ok(index) => index,
            Err(code) => {
                return Err(code);
            }
        };
        let rs = ResultSetKvImpl::new(self.ctx_ptr, index);
        Ok(Box::new(rs))
    }
}

impl SimContext for SimContextImpl {
    fn ok(&self, value: &[u8]) -> result_code {
        if value.len() == 0 {
            return SUCCESS_CODE;
        }
        let json_str = self.common.get_req_header(CONTRACT_METHOD_SUCCESS_RESULT);
        let (req_header_ptr, req_header_len) = (json_str.as_ptr(), json_str.len());

        let (req_body_ptr, req_body_len) = (value.as_ptr(), value.len());
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        resp_code
    }

    fn error(&self, body: &str) -> result_code {
        let json_str = self.common.get_req_header(CONTRACT_METHOD_ERROR_RESULT);
        let (req_header_ptr, req_header_len) = (json_str.as_ptr(), json_str.len());

        let (req_body_ptr, req_body_len) = (body.as_ptr(), body.len());
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        resp_code
    }

    fn log(&self, msg: &str) {
        let json_str = self.common.get_req_header(CONTRACT_METHOD_LOG_MESSAGE);
        let (req_header_ptr, req_header_len) = (json_str.as_ptr(), json_str.len());
        let (req_body_ptr, req_body_len) = (msg.as_ptr(), msg.len());
        unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
    }

    fn call_contract(
        &self,
        contract_name: &str,
        method: &str,
        param: EasyCodec,
    ) -> Result<Vec<u8>, result_code> {
        // check
        if contract_name.len() == 0 || method.len() == 0 {
            return Err(ERROR_CODE);
        }

        // ## header
        let mut len_data = self.common.allocate(I_32_LENGTH as usize);
        let req_header_str = self
            .common
            .get_req_header(CONTRACT_METHOD_CALL_CONTRACT_LEN);
        let (req_header_ptr, req_header_len) = (req_header_str.as_ptr(), req_header_str.len());

        // ## body
        let mut ec = EasyCodec::new();
        ec.add_string("contract_name", contract_name);
        ec.add_string("method", method);
        ec.add_i32("value_ptr", len_data.as_ptr_i32());
        ec.add_bytes("param", param.marshal());
        let param_sys = ec.marshal();
        let (req_body_ptr, req_body_len) = (param_sys.as_ptr(), param_sys.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };

        // parse response
        if resp_code != SUCCESS_CODE {
            let msg = format!("call_contract_len_from_chain error code {}", resp_code);
            log(msg.as_str());
            len_data.drop();
            return Err(resp_code);
        }
        let data_length = mutu8_ptr_to_i32(len_data.as_mut_ptr());
        if data_length == 0 {
            len_data.drop();
            return Ok(Vec::new());
        }
        len_data.drop();

        // # get data
        // ## header
        let mut value = self.common.allocate(data_length as usize);
        let req_header_str = self.common.get_req_header(CONTRACT_METHOD_CALL_CONTRACT);
        let (req_header_ptr, req_header_len) = (req_header_str.as_ptr(), req_header_str.len());

        // ## body
        ec.remove("value_ptr");
        ec.add_i32("value_ptr", value.as_ptr_i32());
        let param_sys = ec.marshal();
        let (req_body_ptr, req_body_len) = (param_sys.as_ptr(), param_sys.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        if resp_code != SUCCESS_CODE {
            let msg = format!("call_contract_from_chain error code {}", resp_code);
            log(msg.as_str());
            value.drop();
            return Err(resp_code);
        }

        // Read bytes of the value and return
        let vec: Vec<u8> =
            unsafe { slice::from_raw_parts(value.as_mut_ptr(), data_length as usize).to_vec() };
        value.drop();

        return Ok(vec);
    }

    fn arg(&self, key: &str) -> Result<Vec<u8>, String> {
        self.params.get_bytes(key)
    }

    fn arg_as_utf8_str(&self, key: &str) -> String {
        let art_vec = self.arg(key).unwrap_or("".as_bytes().to_vec());
        String::from_utf8(art_vec).unwrap_or("".to_string())
    }

    /// How to use JsonValue: https://github.com/maciejhirsz/json-rust
    fn args(&self) -> &EasyCodec {
        &self.params
    }

    /// get_creator_org_id Get create contract org id
    fn get_creator_org_id(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_CREATOR_ORG_ID)
    }
    /// get_creator_pub_key Get create contract pub key
    fn get_creator_pub_key(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_CREATOR_PK)
    }
    /// get_creator_role Get create contract role
    fn get_creator_role(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_CREATOR_ROLE)
    }
    /// get_sender_org_id Get sender contract org id
    fn get_sender_org_id(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_SENDER_ORG_ID)
    }
    /// get_sender_pub_key Get sender contract pub key
    fn get_sender_pub_key(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_SENDER_PK)
    }
    /// get_sender_role Get sender contract role
    fn get_sender_role(&self) -> String {
        self.arg_as_utf8_str(CONTRACT_PARAM_SENDER_ROLE)
    }
    /// get_block_height Get tx block height
    fn get_block_height(&self) -> u64 {
        let block_height = self.arg_as_utf8_str(CONTRACT_PARAM_BLOCK_HEIGHT);
        block_height.parse::<u64>().unwrap()
    }
    /// get_tx_id Get tx block height
    fn get_tx_id(&self) -> String {
        let tx_id = self.arg_as_utf8_str(CONTRACT_PARAM_TX_ID);
        tx_id
    }

    /// get_state get state from chain
    /// # Example:
    /// ```
    /// fn main() {
    /// }
    /// ```
    fn get_state(&self, key: &str, field: &str) -> Result<Vec<u8>, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("key", key);
        ec.add_string("field", field);
        self.common.get_bytes_from_chain(
            ec,
            CONTRACT_METHOD_GET_STATE_LEN,
            CONTRACT_METHOD_GET_STATE,
        )
    }

    /// put state object to chain
    /// # Example:
    /// ```
    /// fn main() {
    /// }
    /// ```
    fn put_state(&self, key: &str, field: &str, value: &[u8]) -> result_code {
        // ## header
        let header = self.common.get_req_header(CONTRACT_METHOD_PUT_STATE);
        let (req_header_ptr, req_header_len) = (header.as_ptr(), header.len());

        // ## body
        let mut ec = EasyCodec::new();
        ec.add_string("key", key);
        ec.add_string("field", field);
        ec.add_bytes("value", value.to_vec());
        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        resp_code
    }

    // delete state object to chain
    fn delete_state(&self, key: &str, field: &str) -> result_code {
        // ## header
        let json_str = self.common.get_req_header(CONTRACT_METHOD_DELETE_STATE);
        let (req_header_ptr, req_header_len) = (json_str.as_ptr(), json_str.len());

        // ## body
        let mut ec = EasyCodec::new();
        ec.add_string("key", key);
        ec.add_string("field", field);
        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());

        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        resp_code
    }

    fn emit_event(&mut self, topic: &str, data: &Vec<String>) -> result_code {
        // ## header
        let header = self.common.get_req_header(CONTRACT_METHOD_EMIT_EVENT);
        let (req_header_ptr, req_header_len) = (header.as_ptr(), header.len());
        // ## body
        let mut ec = EasyCodec::new();
        ec.add_string("topic", topic);
        for i in data {
            ec.add_string("data", i);
        }
        let param = ec.marshal();
        let (req_body_ptr, req_body_len) = (param.as_ptr(), param.len());
        // ## call
        let resp_code =
            unsafe { sys_call(req_header_ptr, req_header_len, req_body_ptr, req_body_len) };
        resp_code
    }

    fn put_state_from_key(&self, key: &str, value: &[u8]) -> result_code {
        self.put_state(key, "", value)
    }

    fn get_state_from_key(&self, key: &str) -> Result<Vec<u8>, result_code> {
        self.get_state(key, "")
    }

    fn delete_state_from_key(&self, key: &str) -> result_code {
        self.delete_state(key, "")
    }

    fn get_paillier_sim_context(&self) -> Box<dyn PaillierSimContext> {
        Box::new(PaillierSimContextImpl::new(self.ctx_ptr))
    }

    fn get_bulletproofs_sim_context(&self) -> Box<dyn BulletproofsSimContext> {
        Box::new(BulletproofsSimContextImpl::new(self.ctx_ptr))
    }

    fn get_sql_sim_context(&self) -> Box<dyn SqlSimContext> {
        Box::new(SqlSimContextImpl::new(self.ctx_ptr))
    }

    fn new_iterator(
        &self,
        start_key: &str,
        limit_key: &str,
    ) -> Result<Box<dyn ResultSet>, result_code> {
        self.new_iterator_core(start_key, "", limit_key, "", CONTRACT_METHOD_KV_ITERATOR)
    }

    fn new_iterator_with_field(
        &self,
        key: &str,
        start_field: &str,
        limit_field: &str,
    ) -> Result<Box<dyn ResultSet>, result_code> {
        self.new_iterator_core(
            key,
            start_field,
            key,
            limit_field,
            CONTRACT_METHOD_KV_ITERATOR,
        )
    }

    fn new_iterator_prefix_with_key_field(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Box<dyn ResultSet>, result_code> {
        self.new_iterator_core(key, field, "", "", CONTRACT_METHOD_KV_PRE_ITERATOR)
    }

    fn new_iterator_prefix_with_key(&self, key: &str) -> Result<Box<dyn ResultSet>, result_code> {
        self.new_iterator_core(key, "", "", "", CONTRACT_METHOD_KV_PRE_ITERATOR)
    }
}

pub struct ResultSetKvImpl {
    pub common: CommonUtils,
    pub index: i32,
}

impl ResultSetKvImpl {
    pub fn new(ctx_ptr: i32, index: i32) -> ResultSetKvImpl {
        ResultSetKvImpl {
            common: CommonUtils { ctx_ptr },
            index,
        }
    }
}
impl ResultSet for ResultSetKvImpl {
    /// get the next row of data
    fn next_row(&self) -> Result<EasyCodec, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_i32("rs_index", self.index);
        let r = self.common.get_bytes_from_chain(
            ec,
            CONTRACT_METHOD_KV_ITERATOR_NEXT_LEN,
            CONTRACT_METHOD_KV_ITERATOR_NEXT,
        );

        match r {
            Ok(data) => return Ok(EasyCodec::unmarshal(&data)),
            Err(code) => return Err(code),
        }
    }
    /// return whether there is a next line
    fn has_next(&self) -> bool {
        let ec = &mut EasyCodec::new();
        ec.add_i32("rs_index", self.index);
        let r = self
            .common
            .get_i32_from_chain(ec, CONTRACT_METHOD_KV_ITERATOR_HAS_NEXT);

        match r {
            Ok(state) => {
                if state == 0 {
                    return false;
                } else {
                    return true;
                }
            }
            _ => false,
        }
    }
    /// close the query statement
    fn close(&self) -> bool {
        let ec = &mut EasyCodec::new();
        ec.add_i32("rs_index", self.index);

        let r = self
            .common
            .get_i32_from_chain(ec, CONTRACT_METHOD_KV_ITERATOR_CLOSE);
        match r {
            Ok(_) => true,
            _ => false,
        }
    }
}

/// LittleEndian
pub fn mutu8_ptr_to_i32(data_return_length_ptr: *mut u8) -> i32 {
    let return_length_slice =
        unsafe { slice::from_raw_parts(data_return_length_ptr, I_32_LENGTH as usize).to_vec() };
    let return_length_slice = return_length_slice.as_slice();
    let return_data_length: i32 = i32::from_le_bytes([
        *return_length_slice.get(0).unwrap(),
        *return_length_slice.get(1).unwrap(),
        *return_length_slice.get(2).unwrap(),
        *return_length_slice.get(3).unwrap(),
    ]);

    // let msg = format!("return data length {}", return_data_length);
    // log(msg.as_str());

    return_data_length
}

/// LittleEndian
pub fn vecu8_to_i32(return_length_slice: Vec<u8>) -> i32 {
    let return_length_slice = return_length_slice.as_slice();
    let return_data_length: i32 = i32::from_le_bytes([
        *return_length_slice.get(0).unwrap(),
        *return_length_slice.get(1).unwrap(),
        *return_length_slice.get(2).unwrap(),
        *return_length_slice.get(3).unwrap(),
    ]);
    // let msg = format!("return data length {}", return_data_length);
    // log(msg.as_str());

    return_data_length
}

/// Can only be called once globally
pub fn get_sim_context() -> impl SimContext {
    let vb = VecBox::<u8>::get_instance(0);
    let vb_ref = vb.as_ref();
    let mut vb_obj = vb_ref.lock().unwrap();

    let len = vb_obj.len();
    let ptr = vb_obj.data_ptr() as *const u8;

    // parse bytes
    let vec: Vec<u8> = unsafe { slice::from_raw_parts(ptr, len).to_vec() };
    let params = EasyCodec::unmarshal(&vec);
    let ctx_ptr = params.get_bytes_as_utf8_string(CONTRACT_PARAM_CONTEXT_PTR).unwrap();
    let ctx_ptr = ctx_ptr.parse::<i32>().unwrap();
    SimContextImpl::new(ctx_ptr, params)
}
