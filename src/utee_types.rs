// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 KylinSoft Co., Ltd. <https://www.kylinos.cn/>
// See LICENSES for license details.
//


use super::*;

#[repr(C)]
pub enum utee_time_category {
    UTEE_TIME_CAT_SYSTEM,
    UTEE_TIME_CAT_TA_PERSISTENT,
    UTEE_TIME_CAT_REE,
}

#[repr(C)]
pub enum utee_entry_func {
    UTEE_ENTRY_FUNC_OPEN_SESSION,
    UTEE_ENTRY_FUNC_CLOSE_SESSION,
    UTEE_ENTRY_FUNC_INVOKE_COMMAND,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum utee_cache_operation {
    TEE_CACHECLEAN,
    TEE_CACHEFLUSH,
    TEE_CACHEINVALIDATE,
}

#[repr(C)]
pub struct utee_params {
    types: u64,
    vals: [u64; TEE_NUM_PARAMS as usize * 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct utee_attribute {
    pub a: u64,
    pub b: u64,
    pub attribute_id: u32,
}

#[repr(C)]
pub struct utee_object_info {
    pub obj_type: u32,
	pub obj_size: u32,
	pub max_obj_size: u32,
	pub obj_usage: u32,
	pub data_size: u32,
	pub data_pos: u32,
	pub handle_flags: u32,
}

impl Default for utee_object_info {
    fn default() -> Self {
        utee_object_info {
            obj_type: 0,
            obj_size: 0,
            max_obj_size: 0,
            obj_usage: 0,
            data_size: 0,
            data_pos: 0,
            handle_flags: 0,
        }
    }
}