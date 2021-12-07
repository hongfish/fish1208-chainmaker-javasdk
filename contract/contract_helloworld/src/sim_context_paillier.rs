/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context::*;

const CONTRACT_METHOD_GET_PAILLIER_OPERATION_RESULT: &str = "GetPaillierOperationResult";
const CONTRACT_METHOD_GET_PAILLIER_OPERATION_RESULT_LEN: &str = "GetPaillierOperationResultLen";
const PAILLIER_OPERATION_TYPE_ADD_CIPHERTEXT: &str = "AddCiphertext";
const PAILLIER_OPERATION_TYPE_ADD_PLAINTEXT: &str = "AddPlaintext";
const PAILLIER_OPERATION_TYPE_SUB_CIPHERTEXT: &str = "SubCiphertext";
const PAILLIER_OPERATION_TYPE_SUB_PLAINTEXT: &str = "SubPlaintext";
const PAILLIER_OPERATION_TYPE_NUM_MUL: &str = "NumMul";

pub trait PaillierSimContext {
    // Paillier method
    fn add_ciphertext(
        &self,
        pubkey: Vec<u8>,
        ciphertext1: Vec<u8>,
        ciphertext2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code>;
    fn add_plaintext(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code>;
    fn sub_ciphertext(
        &self,
        pubkey: Vec<u8>,
        ciphertext1: Vec<u8>,
        ciphertext2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code>;
    fn sub_plaintext(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code>;
    fn num_mul(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code>;
}

pub struct PaillierSimContextImpl {
    pub common: CommonUtils,
}

impl PaillierSimContextImpl {
    pub fn new(ctx_ptr: i32) -> PaillierSimContextImpl {
        PaillierSimContextImpl {
            common: CommonUtils { ctx_ptr },
        }
    }
}

impl PaillierSimContext for PaillierSimContextImpl {
    fn add_ciphertext(
        &self,
        pubkey: Vec<u8>,
        ciphertext1: Vec<u8>,
        ciphertext2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code> {
        paillier_operation(
            self,
            pubkey,
            ciphertext1,
            ciphertext2,
            PAILLIER_OPERATION_TYPE_ADD_CIPHERTEXT,
        )
    }

    fn add_plaintext(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code> {
        paillier_operation(
            self,
            pubkey,
            ciphertext,
            plaintext.to_string().into_bytes(),
            PAILLIER_OPERATION_TYPE_ADD_PLAINTEXT,
        )
    }

    fn sub_ciphertext(
        &self,
        pubkey: Vec<u8>,
        ciphertext1: Vec<u8>,
        ciphertext2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code> {
        paillier_operation(
            self,
            pubkey,
            ciphertext1,
            ciphertext2,
            PAILLIER_OPERATION_TYPE_SUB_CIPHERTEXT,
        )
    }

    fn sub_plaintext(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code> {
        paillier_operation(
            self,
            pubkey,
            ciphertext,
            plaintext.to_string().into_bytes(),
            PAILLIER_OPERATION_TYPE_SUB_PLAINTEXT,
        )
    }

    fn num_mul(
        &self,
        pubkey: Vec<u8>,
        ciphertext: Vec<u8>,
        plaintext: &str,
    ) -> Result<Vec<u8>, result_code> {
        paillier_operation(
            self,
            pubkey,
            ciphertext,
            plaintext.to_string().into_bytes(),
            PAILLIER_OPERATION_TYPE_NUM_MUL,
        )
    }
}

fn paillier_operation(
    crypto_sim_context: &PaillierSimContextImpl,
    pubkey: Vec<u8>,
    operate_one: Vec<u8>,
    operate_two: Vec<u8>,
    operation_type: &str,
) -> Result<Vec<u8>, result_code> {
    let ec = &mut EasyCodec::new();
    ec.add_bytes("pubKey", pubkey);
    ec.add_bytes("operandOne", operate_one);
    ec.add_bytes("operandTwo", operate_two);
    ec.add_string("opType", operation_type);
    crypto_sim_context.common.get_bytes_from_chain(
        ec,
        CONTRACT_METHOD_GET_PAILLIER_OPERATION_RESULT_LEN,
        CONTRACT_METHOD_GET_PAILLIER_OPERATION_RESULT,
    )
}
