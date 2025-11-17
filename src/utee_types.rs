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
pub struct utee_attribute {
    a: u64,
    b: u64,
    attribute_id: u32,
}

#[repr(C)]
pub struct utee_object_info {
    obj_type: u32,
	obj_size: u32,
	max_obj_size: u32,
	obj_usage: u32,
	data_size: u32,
	data_pos: u32,
	handle_flags: u32,
}
