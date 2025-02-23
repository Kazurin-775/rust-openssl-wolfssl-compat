use super::super::*;
use libc::*;

#[cfg(any(libressl, all(ossl102, not(ossl110))))]
pub enum X509_VERIFY_PARAM_ID {}

extern "C" {
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_X509_LOOKUP_meth_free"]
    pub fn X509_LOOKUP_meth_free(method: *mut X509_LOOKUP_METHOD);
}

extern "C" {
    #[link_name = "wolfSSL_X509_LOOKUP_free"]
    pub fn X509_LOOKUP_free(ctx: *mut X509_LOOKUP);
    #[link_name = "wolfSSL_X509_LOOKUP_hash_dir"]
    pub fn X509_LOOKUP_hash_dir() -> *mut X509_LOOKUP_METHOD;
    #[link_name = "wolfSSL_X509_LOOKUP_file"]
    pub fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    #[link_name = "wolfSSL_X509_LOOKUP_ctrl"]
    pub fn X509_LOOKUP_ctrl(
        ctx: *mut X509_LOOKUP,
        cmd: c_int,
        argc: *const c_char,
        argl: c_long,
        ret: *mut *mut c_char,
    ) -> c_int;
    #[link_name = "wolfSSL_X509_load_cert_file"]
    pub fn X509_load_cert_file(ctx: *mut X509_LOOKUP, file: *const c_char, _type: c_int) -> c_int;
    #[link_name = "wolfSSL_X509_load_crl_file"]
    pub fn X509_load_crl_file(ctx: *mut X509_LOOKUP, file: *const c_char, _type: c_int) -> c_int;
}

extern "C" {
    #[link_name = "wolfSSL_X509_STORE_new"]
    pub fn X509_STORE_new() -> *mut X509_STORE;
    #[link_name = "wolfSSL_X509_STORE_free"]
    pub fn X509_STORE_free(store: *mut X509_STORE);

    #[link_name = "wolfSSL_X509_STORE_CTX_new"]
    pub fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;

    #[link_name = "wolfSSL_X509_STORE_CTX_free"]
    pub fn X509_STORE_CTX_free(ctx: *mut X509_STORE_CTX);
    #[link_name = "wolfSSL_X509_STORE_CTX_init"]
    pub fn X509_STORE_CTX_init(
        ctx: *mut X509_STORE_CTX,
        store: *mut X509_STORE,
        x509: *mut X509,
        chain: *mut stack_st_X509,
    ) -> c_int;
    #[link_name = "wolfSSL_X509_STORE_CTX_cleanup"]
    pub fn X509_STORE_CTX_cleanup(ctx: *mut X509_STORE_CTX);

    #[link_name = "wolfSSL_X509_STORE_add_cert"]
    pub fn X509_STORE_add_cert(store: *mut X509_STORE, x: *mut X509) -> c_int;

    #[link_name = "wolfSSL_X509_STORE_add_lookup"]
    pub fn X509_STORE_add_lookup(
        store: *mut X509_STORE,
        meth: *mut X509_LOOKUP_METHOD,
    ) -> *mut X509_LOOKUP;

    #[link_name = "wolfSSL_X509_STORE_set_default_paths"]
    pub fn X509_STORE_set_default_paths(store: *mut X509_STORE) -> c_int;
    #[link_name = "wolfSSL_X509_STORE_set_flags"]
    pub fn X509_STORE_set_flags(store: *mut X509_STORE, flags: c_ulong) -> c_int;
    #[link_name = "wolfSSL_X509_STORE_set_purpose"]
    pub fn X509_STORE_set_purpose(ctx: *mut X509_STORE, purpose: c_int) -> c_int;
    #[link_name = "wolfSSL_X509_STORE_set_trust"]
    pub fn X509_STORE_set_trust(ctx: *mut X509_STORE, trust: c_int) -> c_int;

}

const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_X509_STORE_set1_param"]
        pub fn X509_STORE_set1_param(store: *mut X509_STORE, pm: #[const_ptr_if(ossl300)] X509_VERIFY_PARAM) -> c_int;
    }
}

const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_X509_STORE_CTX_get_ex_data"]
        pub fn X509_STORE_CTX_get_ex_data(ctx: #[const_ptr_if(ossl300)] X509_STORE_CTX, idx: c_int) -> *mut c_void;
        #[link_name = "wolfSSL_X509_STORE_CTX_get_error"]
        pub fn X509_STORE_CTX_get_error(ctx: #[const_ptr_if(ossl300)] X509_STORE_CTX) -> c_int;
        #[link_name = "wolfSSL_X509_STORE_CTX_get_error_depth"]
        pub fn X509_STORE_CTX_get_error_depth(ctx: #[const_ptr_if(ossl300)] X509_STORE_CTX) -> c_int;
        #[link_name = "wolfSSL_X509_STORE_CTX_get_current_cert"]
        pub fn X509_STORE_CTX_get_current_cert(ctx: #[const_ptr_if(ossl300)] X509_STORE_CTX) -> *mut X509;
    }
}
extern "C" {
    #[link_name = "wolfSSL_X509_STORE_CTX_set_error"]
    pub fn X509_STORE_CTX_set_error(ctx: *mut X509_STORE_CTX, error: c_int);
}
cfg_if! {
    if #[cfg(any(ossl110, libressl350))] {
        const_ptr_api! {
            extern "C" {
                #[link_name = "wolfSSL_X509_STORE_CTX_get0_chain"]
                pub fn X509_STORE_CTX_get0_chain(ctx: #[const_ptr_if(ossl300)] X509_STORE_CTX) -> *mut stack_st_X509;
            }
        }
    } else {
        extern "C" {
            #[link_name = "wolfSSL_X509_STORE_CTX_get_chain"]
            pub fn X509_STORE_CTX_get_chain(ctx: *mut X509_STORE_CTX) -> *mut stack_st_X509;
        }
    }
}

extern "C" {
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_new"]
    pub fn X509_VERIFY_PARAM_new() -> *mut X509_VERIFY_PARAM;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_free"]
    pub fn X509_VERIFY_PARAM_free(param: *mut X509_VERIFY_PARAM);

    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_flags"]
    pub fn X509_VERIFY_PARAM_set_flags(param: *mut X509_VERIFY_PARAM, flags: c_ulong) -> c_int;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_clear_flags"]
    pub fn X509_VERIFY_PARAM_clear_flags(param: *mut X509_VERIFY_PARAM, flags: c_ulong) -> c_int;

    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_time"]
    pub fn X509_VERIFY_PARAM_set_time(param: *mut X509_VERIFY_PARAM, t: time_t);

    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_depth"]
    pub fn X509_VERIFY_PARAM_set_depth(param: *mut X509_VERIFY_PARAM, depth: c_int);
}
const_ptr_api! {
    extern "C" {
        #[cfg(any(ossl102, libressl261))]
        #[link_name = "wolfSSL_X509_VERIFY_PARAM_get_flags"]
        pub fn X509_VERIFY_PARAM_get_flags(param: #[const_ptr_if(ossl300)] X509_VERIFY_PARAM) -> c_ulong;
    }
}

extern "C" {
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set1_host"]
    pub fn X509_VERIFY_PARAM_set1_host(
        param: *mut X509_VERIFY_PARAM,
        name: *const c_char,
        namelen: size_t,
    ) -> c_int;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_hostflags"]
    pub fn X509_VERIFY_PARAM_set_hostflags(param: *mut X509_VERIFY_PARAM, flags: c_uint);
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set1_email"]
    pub fn X509_VERIFY_PARAM_set1_email(
        param: *mut X509_VERIFY_PARAM,
        email: *const c_char,
        emaillen: size_t,
    ) -> c_int;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set1_ip"]
    pub fn X509_VERIFY_PARAM_set1_ip(
        param: *mut X509_VERIFY_PARAM,
        ip: *const c_uchar,
        iplen: size_t,
    ) -> c_int;
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_auth_level"]
    pub fn X509_VERIFY_PARAM_set_auth_level(param: *mut X509_VERIFY_PARAM, lvl: c_int);
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_get_auth_level"]
    pub fn X509_VERIFY_PARAM_get_auth_level(param: *const X509_VERIFY_PARAM) -> c_int;
    #[cfg(ossl102)]
    #[link_name = "wolfSSL_X509_VERIFY_PARAM_set_purpose"]
    pub fn X509_VERIFY_PARAM_set_purpose(param: *mut X509_VERIFY_PARAM, purpose: c_int) -> c_int;
}
