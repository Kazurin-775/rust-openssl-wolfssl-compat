use super::super::*;
use libc::*;

pub enum SSL_METHOD {}
pub enum SSL_CIPHER {}
cfg_if! {
    if #[cfg(any(ossl110, libressl280))] {
        pub enum SSL_SESSION {}
    } else if #[cfg(libressl251)] {
        #[repr(C)]
        pub struct SSL_SESSION {
            ssl_version: c_int,
            pub master_key_length: c_int,
            pub master_key: [c_uchar; 48],
            session_id_length: c_uint,
            session_id: [c_uchar; SSL_MAX_SSL_SESSION_ID_LENGTH as usize],
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
            peer: *mut X509,
            verify_result: c_long,
            timeout: c_long,
            time: time_t,
            pub references: c_int,
            cipher: *const SSL_CIPHER,
            cipher_id: c_long,
            ciphers: *mut stack_st_SSL_CIPHER,
            tlsext_hostname: *mut c_char,
            tlsext_tick: *mut c_uchar,
            tlsext_ticklen: size_t,
            tlsext_tick_lifetime_int: c_long,
            internal: *mut c_void,
        }
    } else if #[cfg(libressl)] {
        #[repr(C)]
        pub struct SSL_SESSION {
            ssl_version: c_int,
            pub master_key_length: c_int,
            pub master_key: [c_uchar; 48],
            session_id_length: c_uint,
            session_id: [c_uchar; SSL_MAX_SSL_SESSION_ID_LENGTH as usize],
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
            not_resumable: c_int,
            sess_cert: *mut c_void,
            peer: *mut X509,
            verify_result: c_long,
            timeout: c_long,
            time: time_t,
            pub references: c_int,
            cipher: *const c_void,
            cipher_id: c_ulong,
            ciphers: *mut c_void,
            ex_data: CRYPTO_EX_DATA,
            prev: *mut c_void,
            next: *mut c_void,
            tlsext_hostname: *mut c_char,
            tlsext_ecpointformatlist_length: size_t,
            tlsext_ecpointformatlist: *mut u8,
            tlsext_ellipticcurvelist_length: size_t,
            tlsext_ellipticcurvelist: *mut u16,
            tlsext_tick: *mut c_uchar,
            tlsext_ticklen: size_t,
            tlsext_tick_lifetime_hint: c_long,
        }
    } else {
        #[repr(C)]
        pub struct SSL_SESSION {
            ssl_version: c_int,
            key_arg_length: c_uint,
            key_arg: [c_uchar; SSL_MAX_KEY_ARG_LENGTH as usize],
            pub master_key_length: c_int,
            pub master_key: [c_uchar; 48],
            session_id_length: c_uint,
            session_id: [c_uchar; SSL_MAX_SSL_SESSION_ID_LENGTH as usize],
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
            #[cfg(not(osslconf = "OPENSSL_NO_KRB5"))]
            krb5_client_princ_len: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_KRB5"))]
            krb5_client_princ: [c_uchar; SSL_MAX_KRB5_PRINCIPAL_LENGTH as usize],
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_identity_hint: *mut c_char,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_identity: *mut c_char,
            not_resumable: c_int,
            sess_cert: *mut c_void,
            peer: *mut X509,
            verify_result: c_long,
            pub references: c_int,
            timeout: c_long,
            time: c_long,
            compress_meth: c_uint,
            cipher: *const c_void,
            cipher_id: c_ulong,
            ciphers: *mut c_void,
            ex_data: CRYPTO_EX_DATA,
            prev: *mut c_void,
            next: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_hostname: *mut c_char,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist: *mut c_uchar,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ticklen: size_t,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick_lifetime_hint: c_long,
            #[cfg(not(osslconf = "OPENSSL_NO_SRP"))]
            srp_username: *mut c_char,
        }
    }
}

stack!(stack_st_SSL_CIPHER);

#[repr(C)]
pub struct SRTP_PROTECTION_PROFILE {
    pub name: *const c_char,
    pub id: c_ulong,
}

stack!(stack_st_SRTP_PROTECTION_PROFILE);

pub type tls_session_ticket_ext_cb_fn =
    Option<unsafe extern "C" fn(*mut SSL, *const c_uchar, c_int, *mut c_void) -> c_int>;
pub type tls_session_secret_cb_fn = Option<
    unsafe extern "C" fn(
        *mut SSL,
        *mut c_void,
        *mut c_int,
        *mut stack_st_SSL_CIPHER,
        *mut *mut SSL_CIPHER,
        *mut c_void,
    ) -> c_int,
>;

#[cfg(ossl111)]
pub type SSL_custom_ext_add_cb_ex = Option<
    unsafe extern "C" fn(
        ssl: *mut SSL,
        ext_type: c_uint,
        context: c_uint,
        out: *mut *const c_uchar,
        outlen: *mut size_t,
        x: *mut X509,
        chainidx: size_t,
        al: *mut c_int,
        add_arg: *mut c_void,
    ) -> c_int,
>;

#[cfg(ossl111)]
pub type SSL_custom_ext_free_cb_ex = Option<
    unsafe extern "C" fn(
        ssl: *mut SSL,
        ext_type: c_uint,
        context: c_uint,
        out: *const c_uchar,
        add_arg: *mut c_void,
    ),
>;

#[cfg(ossl111)]
pub type SSL_custom_ext_parse_cb_ex = Option<
    unsafe extern "C" fn(
        ssl: *mut SSL,
        ext_type: c_uint,
        context: c_uint,
        input: *const c_uchar,
        inlen: size_t,
        x: *mut X509,
        chainidx: size_t,
        al: *mut c_int,
        parse_arg: *mut c_void,
    ) -> c_int,
>;

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_get_options"]
            pub fn SSL_CTX_get_options(ctx: *const SSL_CTX) -> u64;
            #[link_name = "wolfSSL_CTX_set_options"]
            pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: u64) -> u64;
            #[link_name = "wolfSSL_CTX_clear_options"]
            pub fn SSL_CTX_clear_options(ctx: *mut SSL_CTX, op: u64) -> u64;
        }
    } else if #[cfg(ossl110)] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_get_options"]
            pub fn SSL_CTX_get_options(ctx: *const SSL_CTX) -> c_ulong;
            #[link_name = "wolfSSL_CTX_set_options"]
            pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: c_ulong) -> c_ulong;
            #[link_name = "wolfSSL_CTX_clear_options"]
            pub fn SSL_CTX_clear_options(ctx: *mut SSL_CTX, op: c_ulong) -> c_ulong;
        }
    }
}

pub type GEN_SESSION_CB =
    Option<unsafe extern "C" fn(*const SSL, *mut c_uchar, *mut c_uint) -> c_int>;

extern "C" {
    #[link_name = "wolfSSL_CTX_sess_set_new_cb"]
    pub fn SSL_CTX_sess_set_new_cb(
        ctx: *mut SSL_CTX,
        new_session_cb: Option<unsafe extern "C" fn(*mut SSL, *mut SSL_SESSION) -> c_int>,
    );
    #[link_name = "wolfSSL_CTX_sess_set_remove_cb"]
    pub fn SSL_CTX_sess_set_remove_cb(
        ctx: *mut SSL_CTX,
        remove_session_cb: Option<unsafe extern "C" fn(*mut SSL_CTX, *mut SSL_SESSION)>,
    );
}
cfg_if! {
    // const change in passed function pointer signature
    if #[cfg(any(ossl110, libressl280))] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_sess_set_get_cb"]
            pub fn SSL_CTX_sess_set_get_cb(
                ctx: *mut SSL_CTX,
                get_session_cb: Option<
                    unsafe extern "C" fn(*mut SSL, *const c_uchar, c_int, *mut c_int) -> *mut SSL_SESSION,
                >,
            );
        }
    } else {
        extern "C" {
            #[link_name = "wolfSSL_CTX_sess_set_get_cb"]
            pub fn SSL_CTX_sess_set_get_cb(
                ctx: *mut SSL_CTX,
                get_session_cb: Option<
                    unsafe extern "C" fn(*mut SSL, *mut c_uchar, c_int, *mut c_int) -> *mut SSL_SESSION,
                >,
            );
        }
    }
}
extern "C" {
    // FIXME change to unsafe extern "C" fn
    #[link_name = "wolfSSL_CTX_set_cookie_generate_cb"]
    pub fn SSL_CTX_set_cookie_generate_cb(
        s: *mut SSL_CTX,
        cb: Option<
            extern "C" fn(ssl: *mut SSL, cookie: *mut c_uchar, cookie_len: *mut c_uint) -> c_int,
        >,
    );
}

cfg_if! {
    // const change in passed function pointer signature
    if #[cfg(any(ossl110, libressl280))] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_set_cookie_verify_cb"]
            pub fn SSL_CTX_set_cookie_verify_cb(
                s: *mut SSL_CTX,
                cb: Option<
                    extern "C" fn(ssl: *mut SSL, cookie: *const c_uchar, cookie_len: c_uint) -> c_int,
                >,
            );
        }
    } else {
        extern "C" {
            #[link_name = "wolfSSL_CTX_set_cookie_verify_cb"]
            pub fn SSL_CTX_set_cookie_verify_cb(
                s: *mut SSL_CTX,
                cb: Option<extern "C" fn(ssl: *mut SSL, cookie: *mut c_uchar, cookie_len: c_uint) -> c_int>,
            );
        }
    }
}

extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_set_stateless_cookie_generate_cb"]
    pub fn SSL_CTX_set_stateless_cookie_generate_cb(
        s: *mut SSL_CTX,
        cb: Option<
            unsafe extern "C" fn(
                ssl: *mut SSL,
                cookie: *mut c_uchar,
                cookie_len: *mut size_t,
            ) -> c_int,
        >,
    );
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_set_stateless_cookie_verify_cb"]
    pub fn SSL_CTX_set_stateless_cookie_verify_cb(
        s: *mut SSL_CTX,
        cb: Option<
            unsafe extern "C" fn(
                ssl: *mut SSL,
                cookie: *const c_uchar,
                cookie_len: size_t,
            ) -> c_int,
        >,
    );

    #[link_name = "wolfSSL_CTX_set_next_protos_advertised_cb"]
    pub fn SSL_CTX_set_next_protos_advertised_cb(
        ssl: *mut SSL_CTX,
        cb: extern "C" fn(
            ssl: *mut SSL,
            out: *mut *const c_uchar,
            outlen: *mut c_uint,
            arg: *mut c_void,
        ) -> c_int,
        arg: *mut c_void,
    );
    #[link_name = "wolfSSL_CTX_set_next_proto_select_cb"]
    pub fn SSL_CTX_set_next_proto_select_cb(
        ssl: *mut SSL_CTX,
        cb: extern "C" fn(
            ssl: *mut SSL,
            out: *mut *mut c_uchar,
            outlen: *mut c_uchar,
            inbuf: *const c_uchar,
            inlen: c_uint,
            arg: *mut c_void,
        ) -> c_int,
        arg: *mut c_void,
    );
    #[link_name = "wolfSSL_get0_next_proto_negotiated"]
    pub fn SSL_get0_next_proto_negotiated(
        s: *const SSL,
        data: *mut *const c_uchar,
        len: *mut c_uint,
    );

    #[link_name = "wolfSSL_select_next_proto"]
    pub fn SSL_select_next_proto(
        out: *mut *mut c_uchar,
        outlen: *mut c_uchar,
        inbuf: *const c_uchar,
        inlen: c_uint,
        client: *const c_uchar,
        client_len: c_uint,
    ) -> c_int;
}

extern "C" {
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_CTX_set_alpn_protos"]
    pub fn SSL_CTX_set_alpn_protos(s: *mut SSL_CTX, data: *const c_uchar, len: c_uint) -> c_int;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_set_alpn_protos"]
    pub fn SSL_set_alpn_protos(s: *mut SSL, data: *const c_uchar, len: c_uint) -> c_int;
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_CTX_set_alpn_select_cb"]
    pub fn SSL_CTX_set_alpn_select_cb__fixed_rust(
        ssl: *mut SSL_CTX,
        cb: Option<
            unsafe extern "C" fn(
                ssl: *mut SSL,
                out: *mut *const c_uchar,
                outlen: *mut c_uchar,
                inbuf: *const c_uchar,
                inlen: c_uint,
                arg: *mut c_void,
            ) -> c_int,
        >,
        arg: *mut c_void,
    );
    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_get0_alpn_selected"]
    pub fn SSL_get0_alpn_selected(s: *const SSL, data: *mut *const c_uchar, len: *mut c_uint);
}

#[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
extern "C" {
    #[link_name = "wolfSSL_CTX_set_psk_client_callback"]
    pub fn SSL_CTX_set_psk_client_callback(
        ssl: *mut SSL_CTX,
        psk_client_cb: Option<
            extern "C" fn(
                *mut SSL,
                *const c_char,
                *mut c_char,
                c_uint,
                *mut c_uchar,
                c_uint,
            ) -> c_uint,
        >,
    );
    #[link_name = "wolfSSL_CTX_set_psk_server_callback"]
    pub fn SSL_CTX_set_psk_server_callback(
        ssl: *mut SSL_CTX,
        psk_server_cb: Option<
            extern "C" fn(*mut SSL, *const c_char, *mut c_uchar, c_uint) -> c_uint,
        >,
    );
    #[link_name = "wolfSSL_get_psk_identity_hint"]
    pub fn SSL_get_psk_identity_hint(ssl: *const SSL) -> *const c_char;
    #[link_name = "wolfSSL_get_psk_identity"]
    pub fn SSL_get_psk_identity(ssl: *const SSL) -> *const c_char;
}

extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_add_custom_ext"]
    pub fn SSL_CTX_add_custom_ext(
        ctx: *mut SSL_CTX,
        ext_type: c_uint,
        context: c_uint,
        add_cb: SSL_custom_ext_add_cb_ex,
        free_cb: SSL_custom_ext_free_cb_ex,
        add_arg: *mut c_void,
        parse_cb: SSL_custom_ext_parse_cb_ex,
        parse_arg: *mut c_void,
    ) -> c_int;

    #[cfg(ossl102)]
    #[link_name = "wolfSSL_extension_supported"]
    pub fn SSL_extension_supported(ext_type: c_uint) -> c_int;
}

#[cfg(ossl111)]
pub type SSL_CTX_keylog_cb_func =
    Option<unsafe extern "C" fn(ssl: *const SSL, line: *const c_char)>;

extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_set_keylog_callback"]
    pub fn SSL_CTX_set_keylog_callback(ctx: *mut SSL_CTX, cb: SSL_CTX_keylog_cb_func);

    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_CTX_set_max_early_data"]
    pub fn SSL_CTX_set_max_early_data(ctx: *mut SSL_CTX, max_early_data: u32) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_CTX_get_max_early_data"]
    pub fn SSL_CTX_get_max_early_data(ctx: *const SSL_CTX) -> u32;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_set_max_early_data"]
    pub fn SSL_set_max_early_data(ctx: *mut SSL, max_early_data: u32) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_get_max_early_data"]
    pub fn SSL_get_max_early_data(ctx: *const SSL) -> u32;

    #[link_name = "wolfSSL_get_finished"]
    pub fn SSL_get_finished(s: *const SSL, buf: *mut c_void, count: size_t) -> size_t;
    #[link_name = "wolfSSL_get_peer_finished"]
    pub fn SSL_get_peer_finished(s: *const SSL, buf: *mut c_void, count: size_t) -> size_t;

    #[link_name = "wolfSSL_CTX_get_verify_mode"]
    pub fn SSL_CTX_get_verify_mode(ctx: *const SSL_CTX) -> c_int;
    #[link_name = "wolfSSL_get_verify_mode"]
    pub fn SSL_get_verify_mode(s: *const SSL) -> c_int;
}

const_ptr_api! {
    extern "C" {
        #[cfg(ossl110)]
        #[link_name = "wolfSSL_is_init_finished"]
        pub fn SSL_is_init_finished(s: #[const_ptr_if(ossl111)] SSL) -> c_int;
    }
}

cfg_if! {
    if #[cfg(libressl261)] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_set_min_proto_version"]
            pub fn SSL_CTX_set_min_proto_version(ctx: *mut SSL_CTX, version: u16) -> c_int;
            #[link_name = "wolfSSL_CTX_set_max_proto_version"]
            pub fn SSL_CTX_set_max_proto_version(ctx: *mut SSL_CTX, version: u16) -> c_int;
            #[link_name = "wolfSSL_set_min_proto_version"]
            pub fn SSL_set_min_proto_version(s: *mut SSL, version: u16) -> c_int;
            #[link_name = "wolfSSL_set_max_proto_version"]
            pub fn SSL_set_max_proto_version(s: *mut SSL, version: u16) -> c_int;
        }
    }
}

cfg_if! {
    if #[cfg(libressl270)] {
        extern "C" {
            #[link_name = "wolfSSL_CTX_get_min_proto_version"]
            pub fn SSL_CTX_get_min_proto_version(ctx: *mut SSL_CTX) -> c_int;
            #[link_name = "wolfSSL_CTX_get_max_proto_version"]
            pub fn SSL_CTX_get_max_proto_version(ctx: *mut SSL_CTX) -> c_int;
            #[link_name = "wolfSSL_get_min_proto_version"]
            pub fn SSL_get_min_proto_version(s: *mut SSL) -> c_int;
            #[link_name = "wolfSSL_get_max_proto_version"]
            pub fn SSL_get_max_proto_version(s: *mut SSL) -> c_int;
        }
    }
}

extern "C" {
    #[link_name = "wolfSSL_CTX_set_cipher_list"]
    pub fn SSL_CTX_set_cipher_list(ssl: *mut SSL_CTX, s: *const c_char) -> c_int;
    #[link_name = "wolfSSL_CTX_new"]
    pub fn SSL_CTX_new(method: *const SSL_METHOD) -> *mut SSL_CTX;
    #[link_name = "wolfSSL_CTX_free"]
    pub fn SSL_CTX_free(ctx: *mut SSL_CTX);
    #[cfg(any(ossl110, libressl273))]
    #[link_name = "wolfSSL_CTX_up_ref"]
    pub fn SSL_CTX_up_ref(x: *mut SSL_CTX) -> c_int;
    #[link_name = "wolfSSL_CTX_get_cert_store"]
    pub fn SSL_CTX_get_cert_store(ctx: *const SSL_CTX) -> *mut X509_STORE;
    #[link_name = "wolfSSL_CTX_set_cert_store"]
    pub fn SSL_CTX_set_cert_store(ctx: *mut SSL_CTX, store: *mut X509_STORE);

    #[link_name = "wolfSSL_get_current_cipher"]
    pub fn SSL_get_current_cipher(ssl: *const SSL) -> *const SSL_CIPHER;
    #[link_name = "wolfSSL_CIPHER_get_bits"]
    pub fn SSL_CIPHER_get_bits(cipher: *const SSL_CIPHER, alg_bits: *mut c_int) -> c_int;
}
const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_CIPHER_get_version"]
        pub fn SSL_CIPHER_get_version(cipher: *const SSL_CIPHER) -> #[const_ptr_if(any(ossl110, libressl280))] c_char;
    }
}
extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CIPHER_get_handshake_digest"]
    pub fn SSL_CIPHER_get_handshake_digest(cipher: *const SSL_CIPHER) -> *const EVP_MD;
    #[link_name = "wolfSSL_CIPHER_get_name"]
    pub fn SSL_CIPHER_get_name(cipher: *const SSL_CIPHER) -> *const c_char;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CIPHER_standard_name"]
    pub fn SSL_CIPHER_standard_name(cipher: *const SSL_CIPHER) -> *const c_char;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_OPENSSL_cipher_name"]
    pub fn OPENSSL_cipher_name(rfc_name: *const c_char) -> *const c_char;

    #[link_name = "wolfSSL_pending"]
    pub fn SSL_pending(ssl: *const SSL) -> c_int;
    #[link_name = "wolfSSL_set_bio"]
    pub fn SSL_set_bio(ssl: *mut SSL, rbio: *mut BIO, wbio: *mut BIO);
    #[link_name = "wolfSSL_SSL_get_rbio"]
    pub fn SSL_get_rbio(ssl: *const SSL) -> *mut BIO;
    #[link_name = "wolfSSL_get_wbio"]
    pub fn SSL_get_wbio(ssl: *const SSL) -> *mut BIO;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_CTX_set_ciphersuites"]
    pub fn SSL_CTX_set_ciphersuites(ctx: *mut SSL_CTX, str: *const c_char) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_set_ciphersuites"]
    pub fn SSL_set_ciphersuites(ssl: *mut SSL, str: *const c_char) -> c_int;
    #[link_name = "wolfSSL_set_cipher_list"]
    pub fn SSL_set_cipher_list(ssl: *mut SSL, s: *const c_char) -> c_int;
    #[link_name = "wolfSSL_set_ssl_method"]
    pub fn SSL_set_ssl_method(s: *mut SSL, method: *const SSL_METHOD) -> c_int;
    #[link_name = "wolfSSL_set_verify"]
    pub fn SSL_set_verify(
        ssl: *mut SSL,
        mode: c_int,
        // FIXME should be unsafe
        verify_callback: Option<extern "C" fn(c_int, *mut X509_STORE_CTX) -> c_int>,
    );
    #[link_name = "wolfSSL_CTX_use_PrivateKey"]
    pub fn SSL_CTX_use_PrivateKey(ctx: *mut SSL_CTX, key: *mut EVP_PKEY) -> c_int;
    #[link_name = "wolfSSL_CTX_use_certificate"]
    pub fn SSL_CTX_use_certificate(ctx: *mut SSL_CTX, cert: *mut X509) -> c_int;

    #[link_name = "wolfSSL_CTX_use_PrivateKey_file"]
    pub fn SSL_CTX_use_PrivateKey_file(
        ctx: *mut SSL_CTX,
        key_file: *const c_char,
        file_type: c_int,
    ) -> c_int;
    #[link_name = "wolfSSL_CTX_use_certificate_file"]
    pub fn SSL_CTX_use_certificate_file(
        ctx: *mut SSL_CTX,
        cert_file: *const c_char,
        file_type: c_int,
    ) -> c_int;
    #[link_name = "wolfSSL_CTX_use_certificate_chain_file"]
    pub fn SSL_CTX_use_certificate_chain_file(
        ctx: *mut SSL_CTX,
        cert_chain_file: *const c_char,
    ) -> c_int;
    #[link_name = "wolfSSL_use_PrivateKey_file"]
    pub fn SSL_use_PrivateKey_file(ssl: *mut SSL, file: *const c_char, type_: c_int) -> c_int;
    #[link_name = "wolfSSL_use_PrivateKey"]
    pub fn SSL_use_PrivateKey(ssl: *mut SSL, pkey: *mut EVP_PKEY) -> c_int;
    #[link_name = "wolfSSL_use_certificate"]
    pub fn SSL_use_certificate(ssl: *mut SSL, x: *mut X509) -> c_int;
    #[cfg(any(ossl110, libressl332))]
    #[link_name = "wolfSSL_use_certificate_chain_file"]
    pub fn SSL_use_certificate_chain_file(ssl: *mut SSL, file: *const c_char) -> c_int;
    #[link_name = "wolfSSL_set_client_CA_list"]
    pub fn SSL_set_client_CA_list(s: *mut SSL, name_list: *mut stack_st_X509_NAME);
    #[link_name = "wolfSSL_add_client_CA"]
    pub fn SSL_add_client_CA(ssl: *mut SSL, x: *mut X509) -> c_int;
    #[link_name = "wolfSSL_load_client_CA_file"]
    pub fn SSL_load_client_CA_file(file: *const c_char) -> *mut stack_st_X509_NAME;

    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_load_error_strings"]
    pub fn SSL_load_error_strings();
    #[link_name = "wolfSSL_state_string"]
    pub fn SSL_state_string(ssl: *const SSL) -> *const c_char;
    #[link_name = "wolfSSL_state_string_long"]
    pub fn SSL_state_string_long(ssl: *const SSL) -> *const c_char;

    #[link_name = "wolfSSL_SESSION_get_time"]
    pub fn SSL_SESSION_get_time(s: *const SSL_SESSION) -> c_long;
    #[link_name = "wolfSSL_SESSION_get_timeout"]
    pub fn SSL_SESSION_get_timeout(s: *const SSL_SESSION) -> c_long;
    #[cfg(any(ossl110, libressl270))]
    #[link_name = "wolfSSL_SESSION_get_protocol_version"]
    pub fn SSL_SESSION_get_protocol_version(s: *const SSL_SESSION) -> c_int;

    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_SESSION_set_max_early_data"]
    pub fn SSL_SESSION_set_max_early_data(ctx: *mut SSL_SESSION, max_early_data: u32) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_SESSION_get_max_early_data"]
    pub fn SSL_SESSION_get_max_early_data(ctx: *const SSL_SESSION) -> u32;

    #[link_name = "wolfSSL_SESSION_get_id"]
    pub fn SSL_SESSION_get_id(s: *const SSL_SESSION, len: *mut c_uint) -> *const c_uchar;
    #[cfg(any(ossl110, libressl273))]
    #[link_name = "wolfSSL_SESSION_up_ref"]
    pub fn SSL_SESSION_up_ref(ses: *mut SSL_SESSION) -> c_int;
    #[link_name = "wolfSSL_SESSION_free"]
    pub fn SSL_SESSION_free(s: *mut SSL_SESSION);
}
const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_i2d_SSL_SESSION"]
        pub fn i2d_SSL_SESSION(s: #[const_ptr_if(ossl300)] SSL_SESSION, pp: *mut *mut c_uchar) -> c_int;
    }
}
extern "C" {
    #[link_name = "wolfSSL_set_session"]
    pub fn SSL_set_session(ssl: *mut SSL, session: *mut SSL_SESSION) -> c_int;
    #[link_name = "wolfSSL_CTX_add_session"]
    pub fn SSL_CTX_add_session(ctx: *mut SSL_CTX, session: *mut SSL_SESSION) -> c_int;
    #[link_name = "wolfSSL_CTX_remove_session"]
    pub fn SSL_CTX_remove_session(ctx: *mut SSL_CTX, session: *mut SSL_SESSION) -> c_int;
    #[link_name = "wolfSSL_d2i_SSL_SESSION"]
    pub fn d2i_SSL_SESSION(
        a: *mut *mut SSL_SESSION,
        pp: *mut *const c_uchar,
        len: c_long,
    ) -> *mut SSL_SESSION;

    #[cfg(not(ossl300))]
    #[link_name = "wolfSSL_get_peer_certificate"]
    pub fn SSL_get_peer_certificate(ssl: *const SSL) -> *mut X509;
    #[cfg(ossl300)]
    #[link_name = "wolfSSL_get1_peer_certificate"]
    pub fn SSL_get1_peer_certificate(ssl: *const SSL) -> *mut X509;

    #[link_name = "wolfSSL_get_peer_cert_chain"]
    pub fn SSL_get_peer_cert_chain(ssl: *const SSL) -> *mut stack_st_X509;

    #[link_name = "wolfSSL_CTX_set_verify"]
    pub fn SSL_CTX_set_verify(
        ctx: *mut SSL_CTX,
        mode: c_int,
        verify_callback: Option<extern "C" fn(c_int, *mut X509_STORE_CTX) -> c_int>,
    );
    #[link_name = "wolfSSL_CTX_set_verify_depth"]
    pub fn SSL_CTX_set_verify_depth(ctx: *mut SSL_CTX, depth: c_int);

    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_CTX_set_post_handshake_auth"]
    pub fn SSL_CTX_set_post_handshake_auth(ctx: *mut SSL_CTX, val: c_int);

    #[link_name = "wolfSSL_CTX_check_private_key"]
    pub fn SSL_CTX_check_private_key(ctx: *const SSL_CTX) -> c_int;

    #[link_name = "wolfSSL_CTX_set_session_id_context"]
    pub fn SSL_CTX_set_session_id_context(
        ssl: *mut SSL_CTX,
        sid_ctx: *const c_uchar,
        sid_ctx_len: c_uint,
    ) -> c_int;

    #[link_name = "wolfSSL_new"]
    pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;

    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_CTX_get0_param"]
    pub fn SSL_CTX_get0_param(ctx: *mut SSL_CTX) -> *mut X509_VERIFY_PARAM;

    #[cfg(any(ossl102, libressl261))]
    #[link_name = "wolfSSL_get0_param"]
    pub fn SSL_get0_param(ssl: *mut SSL) -> *mut X509_VERIFY_PARAM;
}

#[cfg(ossl111)]
pub type SSL_client_hello_cb_fn =
    Option<unsafe extern "C" fn(s: *mut SSL, al: *mut c_int, arg: *mut c_void) -> c_int>;
extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_set_client_hello_cb"]
    pub fn SSL_CTX_set_client_hello_cb(
        c: *mut SSL_CTX,
        cb: SSL_client_hello_cb_fn,
        arg: *mut c_void,
    );
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_isv2"]
    pub fn SSL_client_hello_isv2(s: *mut SSL) -> c_int;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_legacy_version"]
    pub fn SSL_client_hello_get0_legacy_version(s: *mut SSL) -> c_uint;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_random"]
    pub fn SSL_client_hello_get0_random(s: *mut SSL, out: *mut *const c_uchar) -> size_t;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_session_id"]
    pub fn SSL_client_hello_get0_session_id(s: *mut SSL, out: *mut *const c_uchar) -> size_t;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_ciphers"]
    pub fn SSL_client_hello_get0_ciphers(s: *mut SSL, out: *mut *const c_uchar) -> size_t;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_compression_methods"]
    pub fn SSL_client_hello_get0_compression_methods(
        s: *mut SSL,
        out: *mut *const c_uchar,
    ) -> size_t;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get1_extensions_present"]
    pub fn SSL_client_hello_get1_extensions_present(
        s: *mut SSL,
        out: *mut *mut c_int,
        outlen: *mut size_t,
    ) -> c_int;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_client_hello_get0_ext"]
    pub fn SSL_client_hello_get0_ext(
        s: *mut SSL,
        type_: c_uint,
        out: *mut *const c_uchar,
        outlen: *mut size_t,
    ) -> c_int;

    #[link_name = "wolfSSL_free"]
    pub fn SSL_free(ssl: *mut SSL);
    #[link_name = "wolfSSL_accept"]
    pub fn SSL_accept(ssl: *mut SSL) -> c_int;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_stateless"]
    pub fn SSL_stateless(s: *mut SSL) -> c_int;
    #[link_name = "wolfSSL_connect"]
    pub fn SSL_connect(ssl: *mut SSL) -> c_int;
    #[link_name = "wolfSSL_read"]
    pub fn SSL_read(ssl: *mut SSL, buf: *mut c_void, num: c_int) -> c_int;
    #[link_name = "wolfSSL_peek"]
    pub fn SSL_peek(ssl: *mut SSL, buf: *mut c_void, num: c_int) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_read_early_data"]
    pub fn SSL_read_early_data(
        s: *mut SSL,
        buf: *mut c_void,
        num: size_t,
        readbytes: *mut size_t,
    ) -> c_int;
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_bytes_to_cipher_list"]
    pub fn SSL_bytes_to_cipher_list(
        s: *mut SSL,
        bytes: *const c_uchar,
        len: size_t,
        isv2format: c_int,
        sk: *mut *mut stack_st_SSL_CIPHER,
        scsvs: *mut *mut stack_st_SSL_CIPHER,
    ) -> c_int;
}

extern "C" {
    #[link_name = "wolfSSL_write"]
    pub fn SSL_write(ssl: *mut SSL, buf: *const c_void, num: c_int) -> c_int;
    #[cfg(any(ossl111, libressl340))]
    #[link_name = "wolfSSL_write_early_data"]
    pub fn SSL_write_early_data(
        s: *mut SSL,
        buf: *const c_void,
        num: size_t,
        written: *mut size_t,
    ) -> c_int;
    #[link_name = "wolfSSL_ctrl"]
    pub fn SSL_ctrl(ssl: *mut SSL, cmd: c_int, larg: c_long, parg: *mut c_void) -> c_long;
    #[link_name = "wolfSSL_CTX_ctrl"]
    pub fn SSL_CTX_ctrl(ctx: *mut SSL_CTX, cmd: c_int, larg: c_long, parg: *mut c_void) -> c_long;
    #[link_name = "wolfSSL_CTX_callback_ctrl"]
    pub fn SSL_CTX_callback_ctrl__fixed_rust(
        ctx: *mut SSL_CTX,
        cmd: c_int,
        fp: Option<unsafe extern "C" fn()>,
    ) -> c_long;
}

cfg_if! {
    if #[cfg(any(ossl110, libressl291))] {
        extern "C" {
            #[link_name = "wolfSSLv23_method"]
            pub fn TLS_method() -> *const SSL_METHOD;

            #[link_name = "wolfDTLS_method"]
            pub fn DTLS_method() -> *const SSL_METHOD;

            #[link_name = "wolfTLS_server_method"]
            pub fn TLS_server_method() -> *const SSL_METHOD;

            #[link_name = "wolfTLS_client_method"]
            pub fn TLS_client_method() -> *const SSL_METHOD;
        }
    } else {
        extern "C" {
            #[cfg(not(osslconf = "OPENSSL_NO_SSL3_METHOD"))]
            #[link_name = "wolfSSLv3_method"]
            pub fn SSLv3_method() -> *const SSL_METHOD;

            #[link_name = "wolfSSLv23_method"]
            pub fn SSLv23_method() -> *const SSL_METHOD;

            #[link_name = "wolfSSLv23_client_method"]
            pub fn SSLv23_client_method() -> *const SSL_METHOD;

            #[link_name = "wolfSSLv23_server_method"]
            pub fn SSLv23_server_method() -> *const SSL_METHOD;

            #[link_name = "wolfTLSv1_method"]
            pub fn TLSv1_method() -> *const SSL_METHOD;

            #[link_name = "wolfTLSv1_1_method"]
            pub fn TLSv1_1_method() -> *const SSL_METHOD;

            #[link_name = "wolfTLSv1_2_method"]
            pub fn TLSv1_2_method() -> *const SSL_METHOD;

            #[link_name = "wolfDTLSv1_method"]
            pub fn DTLSv1_method() -> *const SSL_METHOD;

            #[cfg(ossl102)]
            #[link_name = "wolfDTLSv1_2_method"]
            pub fn DTLSv1_2_method() -> *const SSL_METHOD;
        }
    }
}

extern "C" {
    #[link_name = "wolfSSL_get_error"]
    pub fn SSL_get_error(ssl: *const SSL, ret: c_int) -> c_int;
    #[link_name = "wolfSSL_get_version"]
    pub fn SSL_get_version(ssl: *const SSL) -> *const c_char;

    #[link_name = "wolfSSL_do_handshake"]
    pub fn SSL_do_handshake(ssl: *mut SSL) -> c_int;
    #[link_name = "wolfSSL_shutdown"]
    pub fn SSL_shutdown(ssl: *mut SSL) -> c_int;

    #[link_name = "wolfSSL_CTX_set_client_CA_list"]
    pub fn SSL_CTX_set_client_CA_list(ctx: *mut SSL_CTX, list: *mut stack_st_X509_NAME);

    #[link_name = "wolfSSL_CTX_add_client_CA"]
    pub fn SSL_CTX_add_client_CA(ctx: *mut SSL_CTX, cacert: *mut X509) -> c_int;

    #[link_name = "wolfSSL_CTX_set_default_verify_paths"]
    pub fn SSL_CTX_set_default_verify_paths(ctx: *mut SSL_CTX) -> c_int;
    #[link_name = "wolfSSL_CTX_load_verify_locations"]
    pub fn SSL_CTX_load_verify_locations(
        ctx: *mut SSL_CTX,
        CAfile: *const c_char,
        CApath: *const c_char,
    ) -> c_int;
}

const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_get_ssl_method"]
        pub fn SSL_get_ssl_method(ssl: #[const_ptr_if(ossl111b)] SSL) -> *const SSL_METHOD;
    }
}

extern "C" {
    #[link_name = "wolfSSL_set_connect_state"]
    pub fn SSL_set_connect_state(s: *mut SSL);
    #[link_name = "wolfSSL_set_accept_state"]
    pub fn SSL_set_accept_state(s: *mut SSL);

    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_library_init"]
    pub fn SSL_library_init() -> c_int;

    #[link_name = "wolfSSL_CIPHER_description"]
    pub fn SSL_CIPHER_description(
        cipher: *const SSL_CIPHER,
        buf: *mut c_char,
        size: c_int,
    ) -> *mut c_char;

    #[link_name = "wolfSSL_get_certificate"]
    pub fn SSL_get_certificate(ssl: *const SSL) -> *mut X509;
}
const_ptr_api! {
    extern "C" {
        #[link_name = "wolfSSL_get_privatekey"]
        pub fn SSL_get_privatekey(ssl: #[const_ptr_if(any(ossl102, libressl280))] SSL) -> *mut EVP_PKEY;
    }
}

extern "C" {
    #[cfg(any(ossl102, libressl270))]
    #[link_name = "wolfSSL_CTX_get0_certificate"]
    pub fn SSL_CTX_get0_certificate(ctx: *const SSL_CTX) -> *mut X509;
    #[cfg(any(ossl102, libressl340))]
    #[link_name = "wolfSSL_CTX_get0_privatekey"]
    pub fn SSL_CTX_get0_privatekey(ctx: *const SSL_CTX) -> *mut EVP_PKEY;

    #[link_name = "wolfSSL_set_shutdown"]
    pub fn SSL_set_shutdown(ss: *mut SSL, mode: c_int);
    #[link_name = "wolfSSL_get_shutdown"]
    pub fn SSL_get_shutdown(ssl: *const SSL) -> c_int;
    #[link_name = "wolfSSL_version"]
    pub fn SSL_version(ssl: *const SSL) -> c_int;
    #[link_name = "wolfSSL_get_session"]
    pub fn SSL_get_session(s: *const SSL) -> *mut SSL_SESSION;
    #[link_name = "wolfSSL_get_SSL_CTX"]
    pub fn SSL_get_SSL_CTX(ssl: *const SSL) -> *mut SSL_CTX;
    #[link_name = "wolfSSL_set_SSL_CTX"]
    pub fn SSL_set_SSL_CTX(ssl: *mut SSL, ctx: *mut SSL_CTX) -> *mut SSL_CTX;

    #[link_name = "wolfSSL_get_verify_result"]
    pub fn SSL_get_verify_result(ssl: *const SSL) -> c_long;
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_get0_verified_chain"]
    pub fn SSL_get0_verified_chain(ssl: *const SSL) -> *mut stack_st_X509;

    #[cfg(any(ossl110, libressl270))]
    #[link_name = "wolfSSL_get_client_random"]
    pub fn SSL_get_client_random(ssl: *const SSL, out: *mut c_uchar, len: size_t) -> size_t;
    #[cfg(any(ossl110, libressl270))]
    #[link_name = "wolfSSL_get_server_random"]
    pub fn SSL_get_server_random(ssl: *const SSL, out: *mut c_uchar, len: size_t) -> size_t;
    #[cfg(any(ossl110, libressl273))]
    #[link_name = "wolfSSL_SESSION_get_master_key"]
    pub fn SSL_SESSION_get_master_key(
        session: *const SSL_SESSION,
        out: *mut c_uchar,
        outlen: size_t,
    ) -> size_t;
}

extern "C" {
    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_get_ex_new_index"]
    pub fn SSL_get_ex_new_index(
        argl: c_long,
        argp: *mut c_void,
        new_func: Option<CRYPTO_EX_new>,
        dup_func: Option<CRYPTO_EX_dup>,
        free_func: Option<CRYPTO_EX_free>,
    ) -> c_int;

    #[link_name = "wolfSSL_set_ex_data"]
    pub fn SSL_set_ex_data(ssl: *mut SSL, idx: c_int, data: *mut c_void) -> c_int;
    #[link_name = "wolfSSL_get_ex_data"]
    pub fn SSL_get_ex_data(ssl: *const SSL, idx: c_int) -> *mut c_void;

    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_CTX_get_ex_new_index"]
    pub fn SSL_CTX_get_ex_new_index(
        argl: c_long,
        argp: *mut c_void,
        new_func: Option<CRYPTO_EX_new>,
        dup_func: Option<CRYPTO_EX_dup>,
        free_func: Option<CRYPTO_EX_free>,
    ) -> c_int;

    #[link_name = "wolfSSL_CTX_set_ex_data"]
    pub fn SSL_CTX_set_ex_data(ctx: *mut SSL_CTX, idx: c_int, data: *mut c_void) -> c_int;
    #[link_name = "wolfSSL_CTX_get_ex_data"]
    pub fn SSL_CTX_get_ex_data(ctx: *const SSL_CTX, idx: c_int) -> *mut c_void;

    #[link_name = "wolfSSL_get_ex_data_X509_STORE_CTX_idx"]
    pub fn SSL_get_ex_data_X509_STORE_CTX_idx() -> c_int;
}

extern "C" {
    #[link_name = "wolfSSL_CTX_set_tmp_dh_callback"]
    pub fn SSL_CTX_set_tmp_dh_callback__fixed_rust(
        ctx: *mut SSL_CTX,
        dh: Option<
            unsafe extern "C" fn(ssl: *mut SSL, is_export: c_int, keylength: c_int) -> *mut DH,
        >,
    );
    #[link_name = "wolfSSL_set_tmp_dh_callback"]
    pub fn SSL_set_tmp_dh_callback__fixed_rust(
        ctx: *mut SSL,
        dh: Option<
            unsafe extern "C" fn(ssl: *mut SSL, is_export: c_int, keylength: c_int) -> *mut DH,
        >,
    );
    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_CTX_set_tmp_ecdh_callback"]
    pub fn SSL_CTX_set_tmp_ecdh_callback__fixed_rust(
        ctx: *mut SSL_CTX,
        ecdh: Option<
            unsafe extern "C" fn(ssl: *mut SSL, is_export: c_int, keylength: c_int) -> *mut EC_KEY,
        >,
    );
    #[cfg(not(ossl110))]
    #[link_name = "wolfSSL_set_tmp_ecdh_callback"]
    pub fn SSL_set_tmp_ecdh_callback__fixed_rust(
        ssl: *mut SSL,
        ecdh: Option<
            unsafe extern "C" fn(ssl: *mut SSL, is_export: c_int, keylength: c_int) -> *mut EC_KEY,
        >,
    );
}

cfg_if! {
    if #[cfg(libressl)] {
        extern "C" {
            #[link_name = "wolfSSL_get_current_compression"]
            pub fn SSL_get_current_compression(ssl: *mut SSL) -> *const libc::c_void;
        }
    } else if #[cfg(not(osslconf = "OPENSSL_NO_COMP"))] {
        const_ptr_api! {
            extern "C" {
                #[link_name = "wolfSSL_get_current_compression"]
                pub fn SSL_get_current_compression(ssl: #[const_ptr_if(ossl111b)] SSL) -> *const COMP_METHOD;
            }
        }
    }
}
cfg_if! {
    if #[cfg(libressl)] {
        extern "C" {
            #[link_name = "wolfSSL_COMP_get_name"]
            pub fn SSL_COMP_get_name(comp: *const libc::c_void) -> *const c_char;
        }
    } else if #[cfg(not(osslconf = "OPENSSL_NO_COMP"))] {
        extern "C" {
            #[link_name = "wolfSSL_COMP_get_name"]
            pub fn SSL_COMP_get_name(comp: *const COMP_METHOD) -> *const c_char;
        }
    }
}

#[cfg(not(osslconf = "OPENSSL_NO_COMP"))]
extern "C" {
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_COMP_get_type"]
    pub fn COMP_get_type(meth: *const COMP_METHOD) -> i32;
}

extern "C" {
    #[cfg(any(ossl110, libressl270))]
    #[link_name = "wolfSSL_CIPHER_get_cipher_nid"]
    pub fn SSL_CIPHER_get_cipher_nid(c: *const SSL_CIPHER) -> c_int;
    #[cfg(any(ossl110, libressl270))]
    #[link_name = "wolfSSL_CIPHER_get_digest_nid"]
    pub fn SSL_CIPHER_get_digest_nid(c: *const SSL_CIPHER) -> c_int;
}

const_ptr_api! {
    extern "C" {
        #[cfg(ossl110)]
        #[link_name = "wolfSSL_session_reused"]
        pub fn SSL_session_reused(ssl: #[const_ptr_if(ossl111c)] SSL) -> c_int;
    }
}

const_ptr_api! {
    extern "C" {
        #[cfg(any(ossl102, libressl273))]
        #[link_name = "wolfSSL_is_server"]
        pub fn SSL_is_server(s: #[const_ptr_if(any(ossl110f, libressl273))] SSL) -> c_int;
    }
}

extern "C" {
    #[cfg(ossl110)]
    #[link_name = "wolfSSL_OPENSSL_init_ssl"]
    pub fn OPENSSL_init_ssl(opts: u64, settings: *const OPENSSL_INIT_SETTINGS) -> c_int;
}

extern "C" {
    #[cfg(ossl111)]
    #[link_name = "wolfSSL_CTX_set_num_tickets"]
    pub fn SSL_CTX_set_num_tickets(ctx: *mut SSL_CTX, num_tickets: size_t) -> c_int;

    #[cfg(ossl111)]
    #[link_name = "wolfSSL_set_num_tickets"]
    pub fn SSL_set_num_tickets(s: *mut SSL, num_tickets: size_t) -> c_int;

    #[cfg(ossl111b)]
    #[link_name = "wolfSSL_CTX_get_num_tickets"]
    pub fn SSL_CTX_get_num_tickets(ctx: *const SSL_CTX) -> size_t;
    #[cfg(all(ossl111, not(ossl111b)))]
    #[link_name = "wolfSSL_CTX_get_num_tickets"]
    pub fn SSL_CTX_get_num_tickets(ctx: *mut SSL_CTX) -> size_t;

    #[cfg(ossl111b)]
    #[link_name = "wolfSSL_get_num_tickets"]
    pub fn SSL_get_num_tickets(s: *const SSL) -> size_t;
    #[cfg(all(ossl111, not(ossl111b)))]
    #[link_name = "wolfSSL_get_num_tickets"]
    pub fn SSL_get_num_tickets(s: *mut SSL) -> size_t;
}
