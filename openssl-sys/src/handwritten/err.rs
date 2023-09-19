use super::super::*;
use libc::*;

#[repr(C)]
pub struct ERR_STRING_DATA {
    pub error: c_ulong,
    pub string: *const c_char,
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            #[link_name = "wolfSSL_ERR_new"]
            pub fn ERR_new();
            #[link_name = "wolfSSL_ERR_set_debug"]
            pub fn ERR_set_debug(file: *const c_char, line: c_int, func: *const c_char);
            #[link_name = "wolfSSL_ERR_set_error"]
            pub fn ERR_set_error(lib: c_int, reason: c_int, fmt: *const c_char, ...);
        }
    } else {
        extern "C" {
            #[link_name = "wolfSSL_ERR_put_error"]
            pub fn ERR_put_error(lib: c_int, func: c_int, reason: c_int, file: *const c_char, line: c_int);
        }
    }
}

extern "C" {
    #[link_name = "wolfSSL_ERR_set_error_data"]
    pub fn ERR_set_error_data(data: *mut c_char, flags: c_int);

    #[link_name = "wolfSSL_ERR_get_error"]
    pub fn ERR_get_error() -> c_ulong;
    #[cfg(ossl300)]
    #[link_name = "wolfSSL_ERR_get_error_all"]
    pub fn ERR_get_error_all(
        file: *mut *const c_char,
        line: *mut c_int,
        func: *mut *const c_char,
        data: *mut *const c_char,
        flags: *mut c_int,
    ) -> c_ulong;
    #[link_name = "wolfSSL_ERR_get_error_line_data"]
    pub fn ERR_get_error_line_data(
        file: *mut *const c_char,
        line: *mut c_int,
        data: *mut *const c_char,
        flags: *mut c_int,
    ) -> c_ulong;
    #[link_name = "wolfSSL_ERR_peek_last_error"]
    pub fn ERR_peek_last_error() -> c_ulong;
    #[link_name = "wolfSSL_ERR_clear_error"]
    pub fn ERR_clear_error();
    #[link_name = "wolfSSL_ERR_lib_error_string"]
    pub fn ERR_lib_error_string(err: c_ulong) -> *const c_char;
    #[link_name = "wolfSSL_ERR_func_error_string"]
    pub fn ERR_func_error_string(err: c_ulong) -> *const c_char;
    #[link_name = "wolfSSL_ERR_reason_error_string"]
    pub fn ERR_reason_error_string(err: c_ulong) -> *const c_char;
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_ERR_load_strings"]
    pub fn ERR_load_strings(lib: c_int, str: *mut ERR_STRING_DATA) -> c_int;
    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_ERR_load_strings"]
    pub fn ERR_load_strings(lib: c_int, str: *mut ERR_STRING_DATA);
    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_ERR_load_crypto_strings"]
    pub fn ERR_load_crypto_strings();

    #[link_name = "wolfSSL_ERR_get_next_error_library"]
    pub fn ERR_get_next_error_library() -> c_int;
}
