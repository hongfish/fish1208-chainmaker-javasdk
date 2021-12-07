/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use base64::encode;
use std::{io::BufWriter, io::Write, str, usize};

pub type EasyKeyType = i32;
pub type EasyValueType = i32;

pub const EASY_KEY_TYPE_SYSTEM: EasyKeyType = 0;
pub const EASY_KEY_TYPE_USER: EasyKeyType = 1;
pub const EASY_VALUE_TYPE_INT32: EasyValueType = 0;
pub const EASY_VALUE_TYPE_STRING: EasyValueType = 1;
pub const EASY_VALUE_TYPE_BYTES: EasyValueType = 2;

const EC_MAGIC_NUM: [u8; 4] = [99, 109, 101, 99]; // "cmec"
const EC_VERSION: [u8; 4] = [118, 49, 46, 48]; // "v1.0"
const EC_RESERVED: [u8; 8] = [255, 255, 255, 255, 255, 255, 255, 255];

const PARAMETERS_KEY_MAX_COUNT: i32 = 128;
// const MAX_KEY_COUNT: usize = 128;
// const MAX_KEY_LEN: usize = 64;
// const MAX_VALUE_LEN: usize = 1024 * 1024;
const EC_MAGIC_NUM_LEN: usize = 4;
const EC_VERSION_LEN: usize = 4;
const EC_RESERVED_LEN: usize = 8;
const MIN_LEN: usize = 20;

/// serialize [kv_count, key_type, key_len, key, value_type, val_length, val]
/// EasyCodec serialize data
pub struct EasyCodec {
    items: Vec<EasyCodecItem>,
}

#[allow(dead_code)]
impl EasyCodec {
    ///
    /// # Example:
    /// ```
    /// fn main1(){
    ///     let mut ec = EasyCodec::new();
    ///     ec.add_i32("key1", 123);
    ///     ec.add_string("keyStr", "chainmaker长安链");
    ///     ec.add_bytes("bytes", "2".as_bytes().to_vec());
    ///     let bytes:Vec<u8> = ec.marshal();
    ///     let j: String = ec.to_json();
    ///     let ec = EasyCodec::unmarshal(&bytes);
    /// }
    ///
    /// fn main2() {
    ///     let a: i32 = 123;
    ///     let item1 = EasyCodecItem {
    ///         key_type: EASY_KEY_TYPE_USER,
    ///         key: "key1".to_string(),
    ///         value_type: EASY_VALUE_TYPE_INT32,
    ///         value: a.to_le_bytes().to_vec(),
    ///     };
    ///
    ///     let item2 = EasyCodecItem {
    ///         key_type: EASY_KEY_TYPE_USER,
    ///         key: "keyStr".to_string(),
    ///         value_type: EASY_VALUE_TYPE_STRING,
    ///         value: "chainmaker长安链",
    ///     };
    ///     
    ///     let bytes = "chainmaker长安链".as_bytes().to_vec();
    ///     let item3 = EasyCodecItem {
    ///         key_type: EASY_KEY_TYPE_USER,
    ///         key: "bytes".to_string(),
    ///         value_type: EASY_VALUE_TYPE_BYTES,
    ///         value: bytes.as_bytes().to_vec(),
    ///     };
    ///     let items = vec![item1, item2, item3];
    ///     let ec = EasyCodec::new_with(items);
    /// }
    /// ```
    pub fn new() -> EasyCodec {
        EasyCodec { items: Vec::new() }
    }

    /// new with items, example see new() method
    pub fn new_with(data: Vec<EasyCodecItem>) -> EasyCodec {
        EasyCodec { items: data }
    }

    /// new_with_bytes unmarshal data
    pub fn new_with_bytes(data: &Vec<u8>) -> EasyCodec {
        EasyCodec::unmarshal(data)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_items(&self) -> &Vec<EasyCodecItem> {
        &self.items
    }
    pub fn put_item(&mut self, item: EasyCodecItem) {
        self.items.push(item);
    }
    pub fn add_i32(&mut self, key: &str, val: i32) {
        let v = val.to_le_bytes().to_vec();
        self.add_val(EASY_KEY_TYPE_USER, key, EASY_VALUE_TYPE_INT32, v);
    }
    pub fn add_string(&mut self, key: &str, val: &str) {
        let v = val.as_bytes().to_vec();
        self.add_val(EASY_KEY_TYPE_USER, key, EASY_VALUE_TYPE_STRING, v);
    }
    pub fn add_bytes(&mut self, key: &str, val: Vec<u8>) {
        self.add_val(EASY_KEY_TYPE_USER, key, EASY_VALUE_TYPE_BYTES, val);
    }
    pub fn add_val(
        &mut self,
        key_type: EasyKeyType,
        key: &str,
        value_type: EasyValueType,
        value: Vec<u8>,
    ) {
        let item = EasyCodecItem {
            key_type,
            key: key.to_string(),
            value_type,
            value,
        };
        self.items.push(item);
    }

    pub fn get_i32(&self, key: &str) -> Result<i32, String> {
        let key = key.to_string();
        for i in self.items.iter() {
            if i.key == key {
                return i.get_int();
            }
        }
        Err("not found".to_string())
    }
    pub fn get_string(&self, key: &str) -> Result<String, String> {
        let key = key.to_string();
        for i in self.items.iter() {
            if i.key == key {
                return i.get_string();
            }
        }
        Err("not found".to_string())
    }
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>, String> {
        let key = key.to_string();
        for i in self.items.iter() {
            if i.key == key {
                return i.get_bytes();
            }
        }
        Err("not found".to_string())
    }
    pub fn get_bytes_as_utf8_string(&self, key: &str) -> Result<String, String> {
        let key = key.to_string();
        for i in self.items.iter() {
            if i.key == key {
                let val = i.get_bytes();
                if val.is_err() {
                    return Err("value type is not bytes".to_string());
                }
                let val_str = String::from_utf8(val.unwrap());
                if val_str.is_err() {
                    return Err("not found utf-8 bytes".to_string());
                }
                return Ok(val_str.unwrap());
            }
        }
        Err("not found".to_string())
    }
    /// remove key
    pub fn remove(&mut self, key: &str) {
        let key = key.to_string();
        let mut index = 0 as usize;
        for i in self.items.iter() {
            if key == i.key {
                self.items.remove(index);
                break;
            }
            index += 1;
        }
    }

    /// magic number + 预留8字节
    /// kv count + [ key type + key len + key + value type + value len + value ]
    /// marshal to bytes, example see new() method
    pub fn marshal(&self) -> Vec<u8> {
        let buf = Vec::new();
        let mut stream = BufWriter::new(buf);

        // header
        let _ = stream.write(&EC_MAGIC_NUM);
        let _ = stream.write(&EC_VERSION);
        let _ = stream.write(&EC_RESERVED);

        // kv count
        let size = self.items.len() as i32;
        let size_vec = size.to_le_bytes().to_vec();
        let size_byte = size_vec.as_slice();
        let _ = stream.write(size_byte);

        for i in self.items.iter() {
            // key type
            let key_type_size_vec = i.key_type.to_le_bytes().to_vec();
            let key_type_size_byte = key_type_size_vec.as_slice();
            let _ = stream.write(key_type_size_byte).unwrap();
            // key len
            let key_val_size_vec = (i.key.len() as i32).to_le_bytes().to_vec();
            let key_val_size_byte = key_val_size_vec.as_slice();
            let _ = stream.write(key_val_size_byte).unwrap();
            // key
            let _ = stream.write(i.key.as_bytes()).unwrap();

            // val type
            let value_type_size_vec = i.value_type.to_le_bytes().to_vec();
            let value_type_size_byte = value_type_size_vec.as_slice();
            let _ = stream.write(value_type_size_byte).unwrap();
            // val len
            let val_size_vec = (i.value.len() as i32).to_le_bytes().to_vec();
            let val_size_byte = val_size_vec.as_slice();
            let _ = stream.write(val_size_byte).unwrap();
            // val
            let _ = stream.write(i.value.as_slice()).unwrap();
        }
        stream.flush().unwrap();
        let r = stream.into_inner().unwrap();
        r
    }

    /// unmarshal bytes to EasyCodec, example see new() method
    pub fn unmarshal(data: &Vec<u8>) -> EasyCodec {
        if data.len() <= MIN_LEN {
            return EasyCodec::new();
        }
        let mut items: Vec<EasyCodecItem> = Vec::new();
        let slice = data.as_slice();
        let i32_len = 4;
        let mut index: usize = 0;

        // header
        let magic_num = parse_le_i32(&slice, index);
        let ec_magic_num = parse_le_i32(&EC_MAGIC_NUM, 0);
        if magic_num == ec_magic_num {
            index += EC_MAGIC_NUM_LEN;
            let version = parse_le_i32(&slice, index);
            index += EC_VERSION_LEN;
            let reserved1 = parse_le_i32(&slice, index);
            let reserved2 = parse_le_i32(&slice, index + 4);
            index += EC_RESERVED_LEN;
            let ec_magic_num = parse_le_i32(&EC_MAGIC_NUM, 0);
            let ec_version = parse_le_i32(&EC_VERSION, 0);
            let ec_reserved1 = parse_le_i32(&EC_RESERVED, 0);
            let ec_reserved2 = parse_le_i32(&EC_RESERVED, 4);
            if !(magic_num == ec_magic_num
                && version == ec_version
                && reserved1 == ec_reserved1
                && reserved2 == ec_reserved2)
            {
                return EasyCodec::new();
            }
        }

        // kv count
        let kv_count = parse_le_i32(slice, index);
        if kv_count > PARAMETERS_KEY_MAX_COUNT {
            return EasyCodec::new();
        }
        index += i32_len;
        for _ in 0..kv_count {
            // read key type
            let key_type = parse_le_i32(slice, index);
            index += i32_len;
            if key_type != EASY_KEY_TYPE_SYSTEM && key_type != EASY_KEY_TYPE_USER {
                continue;
            }
            // read key len
            let key_len = parse_le_i32(slice, index) as usize;
            index += i32_len;
            // read key
            let key = String::from_utf8(slice[index..(index + key_len)].to_vec()).unwrap();
            index += key_len;

            // read val type
            let value_type = parse_le_i32(slice, index);
            index += i32_len;

            // read val len
            let val_len = parse_le_i32(slice, index) as usize;
            index += i32_len;

            let value = slice[index..(index + val_len)].to_vec();
            index += val_len;

            let item = EasyCodecItem {
                key_type,
                key,
                value_type,
                value,
            };
            items.push(item);
        }
        EasyCodec::new_with(items)
    }

    /// to simple json, bytes is null
    pub fn to_json(&self) -> String {
        let mut str_json = "{".to_string();
        for i in self.items.iter() {
            str_json.push_str("\"");
            str_json.push_str(i.key.as_str());
            str_json.push_str("\":");
            if i.value_type == EASY_VALUE_TYPE_STRING {
                let val = String::from_utf8(i.value.clone()).unwrap();
                let result = val.replace("\"", "\\\"");
                str_json.push_str("\"");
                str_json.push_str(result.as_str());
                str_json.push_str("\"");
            } else if i.value_type == EASY_VALUE_TYPE_INT32 {
                let val = parse_le_i32(i.value.as_slice(), 0);
                str_json.push_str(val.to_string().as_str());
            } else if i.value_type == EASY_VALUE_TYPE_BYTES {
                let val = encode(i.value.as_slice());
                str_json.push_str("\"");
                str_json.push_str(&val);
                str_json.push_str("\"");
            }
            str_json.push_str(",");
        }
        if str_json.len() > 1 {
            str_json.remove(str_json.len() - 1);
        }
        str_json.push_str("}");
        str_json
    }
}

pub struct EasyCodecItem {
    pub key_type: EasyKeyType,
    pub key: String,
    pub value_type: EasyValueType,
    pub value: Vec<u8>,
}

impl EasyCodecItem {
    pub fn get_int(&self) -> Result<i32, String> {
        if self.value_type == EASY_VALUE_TYPE_INT32 {
            return Ok(i32::from_le_bytes([
                self.value[0],
                self.value[1],
                self.value[2],
                self.value[3],
            ]));
        }
        Err("value type not i32".to_string())
    }
    pub fn get_string(&self) -> Result<String, String> {
        if self.value_type == EASY_VALUE_TYPE_STRING {
            return Ok(std::str::from_utf8(&self.value).unwrap().to_string());
        }
        Err("value type not string".to_string())
    }
    pub fn get_bytes(&self) -> Result<Vec<u8>, String> {
        if self.value_type == EASY_VALUE_TYPE_BYTES {
            return Ok(self.value.clone());
        }
        Err("value type not bytes".to_string())
    }
}

fn parse_le_i32(slice: &[u8], index: usize) -> i32 {
    i32::from_le_bytes([
        slice[index + 0],
        slice[index + 1],
        slice[index + 2],
        slice[index + 3],
    ])
}

/// cargo test --package chainmaker-contract
#[cfg(test)]
mod tests {
    use crate::easycodec::*;

    #[test]
    fn base_test() {
        let val: i32 = 12345678;
        let b = val.to_le_bytes();
        let v = b.to_vec();
        let slice = v.as_slice();
        println!("slice {:?}", slice);
        let kv_count = parse_le_i32(slice, 0);
        let index = 4;
        let slice = &slice[index..slice.len()];
        println!("slice {:?}", slice);
        println!("kv_count {:?}", kv_count);

        let iiii = parse_le_i32("1cmec2312".as_bytes(), 1);
        println!("iiii {:?}", iiii);
    }

    /// how to test, execute the following command
    /// cargo test -- --nocapture easy_codec_test
    #[test]
    fn easy_codec_test() {
        let a: i32 = 123;
        let item1 = EasyCodecItem {
            key_type: EASY_KEY_TYPE_USER,
            key: "key1".to_string(),
            value_type: EASY_VALUE_TYPE_INT32,
            value: a.to_le_bytes().to_vec(),
        };

        let bytes = "chainmaker长安链".as_bytes().to_vec();
        let item2 = EasyCodecItem {
            key_type: EASY_KEY_TYPE_USER,
            key: "keyStr".to_string(),
            value_type: EASY_VALUE_TYPE_STRING,
            value: bytes,
        };

        let item3 = EasyCodecItem {
            key_type: EASY_KEY_TYPE_USER,
            key: "bytes".to_string(),
            value_type: EASY_VALUE_TYPE_BYTES,
            value: "222222222222".as_bytes().to_vec(),
        };

        println!("===================marshal start==================");
        let items = vec![item1, item2, item3];
        let ec = EasyCodec::new_with(items);
        let bytes = ec.marshal();
        println!("bytes {:?}", bytes);

        let j = ec.to_json();
        println!("to_json {:?}", j);
        println!("===================marshal end==================");

        println!("===================unmarshal start==================");
        let ec_new = EasyCodec::unmarshal(&bytes);
        let j = ec_new.to_json();
        println!("to_json {:?}", j);
        let bytes = ec_new.marshal();
        println!("bytes {:?}", bytes);
        println!("===================unmarshal end==================");

        println!("===================other use start==================");
        let mut ec = EasyCodec::new();
        ec.add_i32("key1", 123);
        ec.add_string("keyStr", "chainmaker长安链");
        ec.add_bytes("bytes", "2".as_bytes().to_vec());
        let bytes = ec.marshal();
        println!("bytes {:?}", bytes);

        let mut ec = EasyCodec::unmarshal(&bytes);
        let bytes = ec.get_bytes("bytes").unwrap();
        let key1 = ec.get_i32("key1").unwrap();
        let key_str = ec.get_string("keyStr").unwrap();

        println!("bytes {:?}", bytes);
        println!("key1 {:?}", key1);
        println!("key_str {:?}", key_str);
        println!("before 【remove】 len {:?}", ec.len());

        println!("remove key keyStr");
        ec.remove("keyStr");
        let key_str = ec.get_string("keyStr");
        println!("key_str {:?}", key_str);
        println!("after  【remove】 len {:?}", ec.len());

        println!("===================other use end==================");
        tojson_test();
    }

    fn tojson_test() {
        println!("========json test start=========");
        let mut ec = EasyCodec::new();
        ec.add_i32("key1", 123);
        ec.add_string("keyStr", "chainmaker长安链{\"name\":\"abcd\"}");
        ec.add_bytes("bytes", "2".as_bytes().to_vec());
        let j = ec.to_json();
        println!("json: \n{:?}", j);
        println!("========json test end==========");
    }

    /// how to test, execute the following command
    /// cargo test -- --nocapture easy_codec_field_test    
    #[test]
    fn easy_codec_field_test() {
        let origin1: [u8; 174] = [
            99, 109, 101, 99, 118, 49, 46, 48, 255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 0, 0,
            1, 0, 0, 0, 8, 0, 0, 0, 107, 101, 121, 66, 121, 116, 101, 115, 2, 0, 0, 0, 40, 0, 0, 0,
            99, 104, 97, 105, 110, 109, 97, 107, 101, 114, 233, 149, 191, 229, 174, 137, 233, 147,
            190, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43, 45, 61, 123, 125, 124, 58, 63, 62,
            60, 1, 0, 0, 0, 8, 0, 0, 0, 107, 101, 121, 73, 110, 116, 51, 50, 0, 0, 0, 0, 4, 0, 0,
            0, 21, 205, 91, 7, 1, 0, 0, 0, 6, 0, 0, 0, 107, 101, 121, 83, 116, 114, 1, 0, 0, 0, 40,
            0, 0, 0, 99, 104, 97, 105, 110, 109, 97, 107, 101, 114, 233, 149, 191, 229, 174, 137,
            233, 147, 190, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43, 45, 61, 123, 125, 124,
            58, 63, 62, 60,
        ];

        let origin2: [u8; 158] = [
            3, 0, 0, 0, 1, 0, 0, 0, 8, 0, 0, 0, 107, 101, 121, 66, 121, 116, 101, 115, 2, 0, 0, 0,
            40, 0, 0, 0, 99, 104, 97, 105, 110, 109, 97, 107, 101, 114, 233, 149, 191, 229, 174,
            137, 233, 147, 190, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43, 45, 61, 123, 125,
            124, 58, 63, 62, 60, 1, 0, 0, 0, 8, 0, 0, 0, 107, 101, 121, 73, 110, 116, 51, 50, 0, 0,
            0, 0, 4, 0, 0, 0, 21, 205, 91, 7, 1, 0, 0, 0, 6, 0, 0, 0, 107, 101, 121, 83, 116, 114,
            1, 0, 0, 0, 40, 0, 0, 0, 99, 104, 97, 105, 110, 109, 97, 107, 101, 114, 233, 149, 191,
            229, 174, 137, 233, 147, 190, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43, 45, 61,
            123, 125, 124, 58, 63, 62, 60,
        ];

        let str_val = "chainmaker长安链!@#$%^&*()_+-={}|:?><";
        let mut ec = EasyCodec::new();
        ec.add_bytes("keyBytes", str_val.as_bytes().to_vec());
        ec.add_i32("keyInt32", 123456789);
        ec.add_string("keyStr", str_val);
        println!("{:?}", ec.to_json());
        let data = ec.marshal();

        let ec1 = EasyCodec::new_with_bytes(&origin1.to_vec());
        assert_equal_byte(data.as_slice(), ec1.marshal().as_slice());

        let ec2 = EasyCodec::new_with_bytes(&origin2.to_vec());
        assert_equal_byte(data.as_slice(), ec2.marshal().as_slice());
    }

    fn assert_equal_byte(a: &[u8], b: &[u8]) {
        if a.len() != b.len() {
            println!("a \t {:?}", a);
            println!("b \t {:?}", b);
            panic!("not equal")
        }
        for i in 0..a.len() {
            if a[i] != b[i] {
                println!("a \t {:?}", a);
                println!("b \t {:?}", b);
                println!("i {:?} a val={:?} b val={:?}", i, a[i], b[i]);
                panic!("not equal")
            }
        }
    }
}
