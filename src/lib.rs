#![allow(non_camel_case_types)]

use std::os::raw::*;

pub type sasl_malloc_t = extern "C" fn(usize) -> *mut c_void;
pub type sasl_calloc_t = extern "C" fn(usize, usize) -> *mut c_void;
pub type sasl_realloc_t = extern "C" fn(*mut c_void, usize) -> *mut c_void;
pub type sasl_free_t = extern "C" fn(*mut c_void);

extern "C" {
    pub fn sasl_set_alloc(malloc: *mut sasl_malloc_t,
                          calloc: *mut sasl_calloc_t,
                          realloc: *mut sasl_realloc_t,
                          free: *mut sasl_free_t);
}

pub type sasl_mutex_alloc_t = extern "C" fn() -> *mut c_void;
pub type sasl_mutex_lock_t = extern "C" fn(*mut c_void) -> c_int;
pub type sasl_mutex_unlock_t = extern "C" fn(*mut c_void) -> c_int;
pub type sasl_mutex_free_t = extern "C" fn(*mut c_void);

extern "C" {
    pub fn sasl_set_mutex(_1: *mut sasl_mutex_alloc_t,
                          _2: *mut sasl_mutex_lock_t,
                          _3: *mut sasl_mutex_unlock_t,
                          _4: *mut sasl_mutex_free_t);
}

pub type sasl_getopt_t = extern "C" fn(*mut c_void, *const c_char, *const c_char, *const *mut c_char, *mut c_uint) -> c_int;


pub const SASL_CB_GETOPT: c_int = 1;

pub const SASL_LOG_NONE: c_int  = 0;
pub const SASL_LOG_ERR: c_int   = 1;
pub const SASL_LOG_FAIL: c_int  = 2;
pub const SASL_LOG_WARN: c_int  = 3;
pub const SASL_LOG_NOTE: c_int  = 4;
pub const SASL_LOG_DEBUG: c_int = 5;
pub const SASL_LOG_TRACE: c_int = 6;
pub const SASL_LOG_PASS: c_int  = 7;

pub type sasl_log_t = extern "C" fn(*mut c_void, c_int, *const c_char) -> c_int;

pub const SASL_CB_LOG: c_int = 2;

pub type sasl_getpath_t = extern "C" fn(*mut c_void, *const *mut c_char) -> c_int;

pub const SASL_CB_GETPTAH: c_int = 3;

#[repr(C)]
pub enum sasl_verify_type_t {
    SASL_VRFY_PLUGIN = 0,
    SASL_VRFY_CONF   = 1,
    SASL_VRFY_PASSWD = 2,
    SASL_VRFY_OTHER  = 3
}

pub type sasl_verifyfile_t = extern "C" fn(*mut c_void, *const c_char, sasl_verify_type_t) -> c_int;

pub const SASL_CB_VERIFYFILE: c_int = 4;

pub type sasl_getconfpath_t = extern "C" fn(*mut c_void, *mut *mut c_char) -> c_int;
pub const SASL_CB_GETCONFPATH: c_int =  5;

pub type sasl_getsimple_t = extern "C" fn(*mut c_void, c_int, *const *mut c_char, *mut c_uint) -> c_int;

pub const SASL_CB_USER: c_int = 0x4001;
pub const SASL_CB_AUTHNAME: c_int = 0x4002;
pub const SASL_CB_LANGUAGE: c_int = 0x4003;
pub const SASL_CB_CNONCE: c_int = 0x4007;

pub type sasl_getsecret_t = extern "C" fn(*mut sasl_conn_t, *mut c_void, c_int, *mut *mut sasl_secret_t) -> c_int;
