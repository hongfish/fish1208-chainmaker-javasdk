/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

// use crate::sim_context;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct VecBox<T> {
    data: ManuallyDrop<Box<Vec<T>>>,
}

impl<T: Copy> VecBox<T> {
    /// get_instance 全局静态实例，每个instance实例一次。用于chan执行智能合约方法时，往智能合约传输数据
    pub fn get_instance(capacity: usize) -> Arc<Mutex<VecBox<u8>>> {
        static mut VECBOX: Option<Arc<Mutex<VecBox<u8>>>> = None;
        unsafe {
            // Rust中使用可变静态变量都是unsafe的
            let v = VECBOX
                .get_or_insert_with(|| {
                    // 初始化单例对象的代码
                    // sim_context::log(&format!("new instance size={}", capacity));
                    let instance: VecBox<u8> = VecBox::new(capacity);
                    Arc::new(Mutex::new(instance))
                })
                .clone();
            v
        }
    }
    /// new 根据size 申请对应大小的数据内存
    pub fn new(capacity: usize) -> VecBox<T> {
        let v: Vec<T> = Vec::with_capacity(capacity);
        let b = Box::new(v);
        Self {
            data: ManuallyDrop::new(b),
        }
    }

    pub fn reset_data_size(&mut self, capacity: usize) {
        self.deallocated();

        // sim_context::log(&format!("new data allocate size={}", capacity));
        let v: Vec<T> = Vec::with_capacity(capacity);
        let b = Box::new(v);
        self.data = ManuallyDrop::new(b);
    }

    pub fn deallocated(&mut self) {
        unsafe {
            let _ = ManuallyDrop::drop(&mut self.data);
        }
    }

    pub fn drop(&mut self) {
        unsafe {
            let _ = ManuallyDrop::drop(&mut self.data);
        }
    }

    pub fn data_as_mut(&mut self) -> &Vec<T> {
        self.data.as_mut()
    }

    pub fn copy<'a>(&'a mut self, vec: &Vec<T>) -> Result<usize, &str> {
        let size = vec.len() as usize;
        // Write get_state parameters into memory, then vm host could obtain the parameters.
        let buffer = self.data.as_mut();
        if buffer.capacity() >= size {
            for b in vec {
                buffer.push(*b);
            }
            Ok(size)
        } else {
            Err("Vec size overflow")
        }
    }

    pub fn len(&mut self) -> usize {
        return self.data.capacity();
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        return self.data.as_mut_ptr();
    }

    pub fn as_ptr(&mut self) -> *const T {
        return self.data.as_ptr();
    }

    pub fn as_ptr_i32(&mut self) -> i32 {
        return self.data.as_ptr() as i32;
    }

    pub fn as_ptr_str(&mut self) -> String {
        (self.data.as_ptr() as i32).to_string()
    }

    pub fn data_ptr(&mut self) -> *const T {
        let s = &mut self.data.as_ref();
        (*s).as_ptr()
    }

    pub fn value(&mut self) -> &Vec<T> {
        let s = self.data.as_ref();
        s
    }
}
