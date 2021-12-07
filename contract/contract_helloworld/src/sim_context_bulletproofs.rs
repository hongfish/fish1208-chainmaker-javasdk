/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context::*;

const CONTRACT_METHOD_GET_BULLETPROOFS_RESULT: &str = "GetBulletproofsResult";
const CONTRACT_METHOD_GET_BULLETPROOFS_RESULT_LEN: &str = "GetBulletproofsResultLen";
const BULLETPROOFS_OPERATION_TYPE_PEDERSEN_ADD_NUM: &str = "PedersenAddNum";
const BULLETPROOFS_OPERATION_TYPE_PEDERSEN_ADD_COMMITMENT: &str = "PedersenAddCommitment";
const BULLETPROOFS_OPERATION_TYPE_PEDERSEN_SUB_NUM: &str = "PedersenSubNum";
const BULLETPROOFS_OPERATION_TYPE_PEDERSEN_SUB_COMMITMENT: &str = "PedersenSubCommitment";
const BULLETPROOFS_OPERATION_TYPE_PEDERSEN_MUL_NUM: &str = "PedersenMulNum";
const BULLETPROOFS_VERIFY: &str = "BulletproofsVerify";

/// BulletproofsSimContext is the trait that wrap the bulletproofs method
pub trait BulletproofsSimContext {
    /// Compute a commitment to x + y from a commitment to x without revealing the value x, where y is a scalar
    ///
    /// # Arguments
    ///
    /// * `commitment` - C = xB + rB'
    /// * `num` - the value y
    ///
    /// # return
    ///
    /// * `return1` - the new commitment to x + y: C' = (x + y)B + rB'
    ///
    fn pedersen_add_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code>;

    /// Compute a commitment to x + y from commitments to x and y, without revealing the value x and y
    ///
    /// # Arguments
    ///
    /// * `commitment1` - commitment to x: Cx = xB + rB'
    /// * `commitment2` - commitment to y: Cy = yB + sB'
    ///
    /// # return
    ///
    /// * `return1` - commitment to x + y: C = (x + y)B + (r + s)B'
    ///
    fn pedersen_add_commitment(
        &self,
        commitment1: Vec<u8>,
        commitment2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code>;

    /// Compute a commitment to x - y from a commitment to x without revealing the value x, where y is a scalar
    ///
    /// # Arguments
    ///
    /// * `commitment1` - C = xB + rB'
    /// * `num` - the value y
    ///
    /// # return
    ///
    /// * `return1` - the new commitment to x - y: C' = (x - y)B + rB'
    fn pedersen_sub_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code>;

    /// Compute a commitment to x - y from commitments to x and y, without revealing the value x and y
    ///
    /// # Arguments
    ///
    /// * `commitment1` - commitment to x: Cx = xB + rB'
    /// * `commitment2` - commitment to y: Cy = yB + sB'
    ///
    /// # return
    ///
    /// * `return1` - commitment to x - y: C = (x - y)B + (r - s)B'
    fn pedersen_sub_commitment(
        &self,
        commitment1: Vec<u8>,
        commitment2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code>;

    /// Compute a commitment to x * y from a commitment to x and an integer y, without revealing the value x and y
    ///
    /// # Arguments
    ///
    /// * `commitment1` - commitment to x: Cx = xB + rB'
    /// * `num` - integer value y
    ///
    /// # return
    ///
    /// * `return1` - commitment to x * y: C = (x * y)B + (r * y)B'
    fn pedersen_mul_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code>;

    /// Verify the validity of a proof
    ///
    /// # Arguments
    ///
    /// * `proof` - the zero-knowledge proof proving the number committed in commitment is in the range [0, 2^64)
    /// * `commitment` - commitment bindingly hiding the number x
    ///
    /// # return
    ///
    /// * `return1` - true on valid proof, false otherwise
    fn verify(&self, proof: Vec<u8>, commitment: Vec<u8>) -> Result<Vec<u8>, result_code>;
}

pub struct BulletproofsSimContextImpl {
    pub common: CommonUtils,
}

impl BulletproofsSimContextImpl {
    pub fn new(ctx_ptr: result_code) -> BulletproofsSimContextImpl {
        BulletproofsSimContextImpl {
            common: CommonUtils { ctx_ptr },
        }
    }
}

impl BulletproofsSimContext for BulletproofsSimContextImpl {
    fn pedersen_add_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(
            self,
            commitment,
            num.to_string().into_bytes(),
            BULLETPROOFS_OPERATION_TYPE_PEDERSEN_ADD_NUM,
        )
    }

    fn pedersen_add_commitment(
        &self,
        commitment1: Vec<u8>,
        commitment2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(
            self,
            commitment1,
            commitment2,
            BULLETPROOFS_OPERATION_TYPE_PEDERSEN_ADD_COMMITMENT,
        )
    }

    fn pedersen_sub_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(
            self,
            commitment,
            num.to_string().into_bytes(),
            BULLETPROOFS_OPERATION_TYPE_PEDERSEN_SUB_NUM,
        )
    }

    fn pedersen_sub_commitment(
        &self,
        commitment1: Vec<u8>,
        commitment2: Vec<u8>,
    ) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(
            self,
            commitment1,
            commitment2,
            BULLETPROOFS_OPERATION_TYPE_PEDERSEN_SUB_COMMITMENT,
        )
    }

    fn pedersen_mul_num(&self, commitment: Vec<u8>, num: &str) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(
            self,
            commitment,
            num.to_string().into_bytes(),
            BULLETPROOFS_OPERATION_TYPE_PEDERSEN_MUL_NUM,
        )
    }

    fn verify(&self, proof: Vec<u8>, commitment: Vec<u8>) -> Result<Vec<u8>, result_code> {
        bulletproofs_operation(self, proof, commitment, BULLETPROOFS_VERIFY)
    }
}

fn bulletproofs_operation(
    bulletproofs_sim_context: &BulletproofsSimContextImpl,
    param1: Vec<u8>,
    param2: Vec<u8>,
    bulletproofs_func_name: &str,
) -> Result<Vec<u8>, result_code> {
    let ec = &mut EasyCodec::new();
    ec.add_bytes("param1", param1);
    ec.add_bytes("param2", param2);
    ec.add_string("bulletproofsFuncName", bulletproofs_func_name);
    bulletproofs_sim_context.common.get_bytes_from_chain(
        ec,
        CONTRACT_METHOD_GET_BULLETPROOFS_RESULT_LEN,
        CONTRACT_METHOD_GET_BULLETPROOFS_RESULT,
    )
}
