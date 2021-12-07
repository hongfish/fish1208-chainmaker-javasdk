/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context::*;

const CONTRACT_METHOD_EXECUTE_QUERY_SQL: &str = "ExecuteQuery"; // 执行sql
const CONTRACT_METHOD_EXECUTE_QUERY_ONE: &str = "ExecuteQueryOne"; // 执行sql
const CONTRACT_METHOD_EXECUTE_QUERY_ONE_LEN: &str = "ExecuteQueryOneLen"; // 执行sql
const CONTRACT_METHOD_ITERATOR_NEXT: &str = "RSNext"; // 迭代获取下一个数据
const CONTRACT_METHOD_ITERATOR_NEXT_LEN: &str = "RSNextLen"; // 迭代获取下一个数据的长度
const CONTRACT_METHOD_ITERATOR_HAS_NEXT: &str = "RSHasNext"; // 迭代获取是否有下一个数据
const CONTRACT_METHOD_ITERATOR_CLOSE: &str = "RSClose"; // 迭代结束
const CONTRACT_METHOD_EXECUTE_UPDATE_SQL: &str = "ExecuteUpdate"; // 执行sql更新语句
const CONTRACT_METHOD_EXECUTE_DDL_SQL: &str = "ExecuteDDL"; // 执行sql DDL语句

pub trait ResultSet {
    fn next_row(&self) -> Result<EasyCodec, result_code>;
    fn has_next(&self) -> bool;
    fn close(&self) -> bool;
}

// 查询迭代
pub struct ResultSetSqlImpl {
    pub common: CommonUtils,
    pub index: i32,
}

pub struct SqlSimContextImpl {
    pub common: CommonUtils,
}

pub trait SqlSimContext {
    fn execute_query_one(&self, sql: &str) -> Result<EasyCodec, result_code>;
    fn execute_query(&self, sql: &str) -> Result<Box<dyn ResultSet>, result_code>;

    /// #### ExecuteUpdateSql execute update/insert/delete sql
    /// ##### It is best to update with primary key
    ///
    /// as:
    ///
    /// - update table set name = 'Tom' where uniqueKey='xxx'
    /// - delete from table where uniqueKey='xxx'
    /// - insert into table(id, xxx,xxx) values(xxx,xxx,xxx)
    ///
    /// ### not allow:
    /// - random methods: NOW() RAND() and so on
    fn execute_update(&self, sql: &str) -> Result<i32, result_code>;

    /// ExecuteDDLSql execute DDL sql, for init_contract or upgrade method. allow table create/alter/drop/truncate
    ///
    /// ## You must have a primary key to create a table
    /// ### allow:     
    /// - CREATE TABLE tableName
    /// - ALTER TABLE tableName
    /// - DROP TABLE tableName   
    /// - TRUNCATE TABLE tableName
    ///
    /// ### not allow:
    /// - CREATE DATABASE dbName
    /// - CREATE TABLE dbName.tableName
    /// - ALTER TABLE dbName.tableName
    /// - DROP DATABASE dbName   
    /// - DROP TABLE dbName.tableName   
    /// - TRUNCATE TABLE dbName.tableName
    /// not allow:
    /// - random methods: NOW() RAND() and so on
    ///
    fn execute_ddl(&self, sql: &str) -> Result<i32, result_code>;
}

impl SqlSimContext for SqlSimContextImpl {
    /// 查询多行数据
    fn execute_query(&self, sql: &str) -> Result<Box<dyn ResultSet>, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("sql", sql);
        let r = self
            .common
            .get_i32_from_chain(ec, CONTRACT_METHOD_EXECUTE_QUERY_SQL);
        let index = match r {
            Ok(index) => index,
            Err(code) => {
                return Err(code);
            }
        };
        let rs = ResultSetSqlImpl::new(self.common.ctx_ptr, index);
        Ok(Box::new(rs))
    }
    /// 查询单行数据
    fn execute_query_one(&self, sql: &str) -> Result<EasyCodec, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("sql", sql);
        let r = self.common.get_bytes_from_chain(
            ec,
            CONTRACT_METHOD_EXECUTE_QUERY_ONE_LEN,
            CONTRACT_METHOD_EXECUTE_QUERY_ONE,
        );
        match r {
            Ok(bytes) => return Ok(EasyCodec::new_with_bytes(&bytes)),
            Err(code) => return Err(code),
        }
    }
    /// 执行更新语句，返回被影响的条数
    fn execute_update(&self, sql: &str) -> Result<i32, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("sql", sql);
        self.common
            .get_i32_from_chain(ec, CONTRACT_METHOD_EXECUTE_UPDATE_SQL)
    }

    /// 执行一条DDL语句，返回被影响的条数。只有安装合约和更新合约允许执行
    fn execute_ddl(&self, sql: &str) -> Result<i32, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_string("sql", sql);
        self.common
            .get_i32_from_chain(ec, CONTRACT_METHOD_EXECUTE_DDL_SQL)
    }
}

impl SqlSimContextImpl {
    pub fn new(ctx_ptr: i32) -> SqlSimContextImpl {
        SqlSimContextImpl {
            common: CommonUtils { ctx_ptr },
        }
    }
}

// 结构体生命周期不能超过属性的生命周期

impl ResultSetSqlImpl {
    pub fn new(ctx_ptr: i32, index: i32) -> ResultSetSqlImpl {
        ResultSetSqlImpl {
            common: CommonUtils { ctx_ptr },
            index,
        }
    }
}

impl ResultSet for ResultSetSqlImpl {
    /// get the next row of data
    fn next_row(&self) -> Result<EasyCodec, result_code> {
        let ec = &mut EasyCodec::new();
        ec.add_i32("rs_index", self.index);
        let r = self.common.get_bytes_from_chain(
            ec,
            CONTRACT_METHOD_ITERATOR_NEXT_LEN,
            CONTRACT_METHOD_ITERATOR_NEXT,
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
            .get_i32_from_chain(ec, CONTRACT_METHOD_ITERATOR_HAS_NEXT);

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
            .get_i32_from_chain(ec, CONTRACT_METHOD_ITERATOR_CLOSE);

        match r {
            Ok(_) => true,
            _ => false,
        }
    }
}
