#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod napi_sys {
    pub type __uint16_t = ::std::os::raw::c_ushort;
    pub type __int32_t = ::std::os::raw::c_int;
    pub type __uint32_t = ::std::os::raw::c_uint;
    pub type __int64_t = ::std::os::raw::c_long;
    pub type char16_t = u16;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct napi_env__ {
        _unused: [u8; 0],
    }
    pub type napi_env = *mut napi_env__;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct napi_value__ {
        _unused: [u8; 0],
    }
    pub type napi_value = *mut napi_value__;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct napi_callback_info__ {
        _unused: [u8; 0],
    }
    pub type napi_callback_info = *mut napi_callback_info__;
    pub type napi_callback = ::std::option::Option<
        unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    >;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum napi_status {
        napi_ok = 0,
        napi_invalid_arg = 1,
        napi_object_expected = 2,
        napi_string_expected = 3,
        napi_name_expected = 4,
        napi_function_expected = 5,
        napi_number_expected = 6,
        napi_boolean_expected = 7,
        napi_array_expected = 8,
        napi_generic_failure = 9,
        napi_pending_exception = 10,
        napi_cancelled = 11,
        napi_escape_called_twice = 12,
        napi_handle_scope_mismatch = 13,
        napi_callback_scope_mismatch = 14,
        napi_queue_full = 15,
        napi_closing = 16,
        napi_bigint_expected = 17,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct napi_extended_error_info {
        pub error_message: *const ::std::os::raw::c_char,
        pub engine_reserved: *mut ::std::os::raw::c_void,
        pub engine_error_code: u32,
        pub error_code: napi_status,
    }
    #[link(wasm_import_module = "napi")]
    extern "C" {
        pub fn napi_get_last_error_info(
            env: napi_env,
            result: *mut *const napi_extended_error_info,
        ) -> napi_status;
        pub fn napi_is_exception_pending(env: napi_env, result: *mut bool) -> napi_status;
        pub fn napi_create_object(env: napi_env, result: *mut napi_value) -> napi_status;
        pub fn napi_set_named_property(
            env: napi_env,
            object: napi_value,
            utf8name: *const ::std::os::raw::c_char,
            value: napi_value,
        ) -> napi_status;
        pub fn napi_create_int32(env: napi_env, value: i32, result: *mut napi_value)
            -> napi_status;
        pub fn napi_create_function(
            env: napi_env,
            utf8name: *const ::std::os::raw::c_char,
            length: usize,
            cb: napi_callback,
            data: *mut ::std::os::raw::c_void,
            result: *mut napi_value,
        ) -> napi_status;
        pub fn napi_create_string_utf8(
            env: napi_env,
            str: *const ::std::os::raw::c_char,
            length: usize,
            result: *mut napi_value,
        ) -> napi_status;
        pub fn napi_get_cb_info(
            env: napi_env,
            cbinfo: napi_callback_info,
            argc: *mut usize,
            argv: *mut napi_value,
            this_arg: *mut napi_value,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> napi_status;
        pub fn napi_get_new_target(
            env: napi_env,
            cbinfo: napi_callback_info,
            result: *mut napi_value,
        ) -> napi_status;
        pub fn napi_get_undefined(env: napi_env, result: *mut napi_value) -> napi_status;
        pub fn napi_get_null(env: napi_env, result: *mut napi_value) -> napi_status;
        pub fn napi_throw(env: napi_env, error: napi_value) -> napi_status;
    }
}

mod napi {
    use napi_sys::*;
    use std::ffi::{CStr, CString};
    use std::mem::MaybeUninit;

    pub type Value = napi_value;

    macro_rules! NAPI {
        ($env:expr, $f:expr, $r:expr) => {
            #[allow(unused_unsafe)]
            unsafe {
                match $f {
                    napi_status::napi_ok => Ok($r),
                    _ => {
                        let info = MaybeUninit::uninit();
                        assert!(
                            napi_get_last_error_info($env.env, &mut info.as_ptr())
                                == napi_status::napi_ok
                        );
                        let info = info.assume_init();
                        let message = CStr::from_ptr(info.error_message).to_str().unwrap();
                        Err(Error {
                            message: message.to_owned(),
                        })
                    }
                }
            }
        };
    }

    #[derive(Clone, Copy)]
    pub struct Env {
        env: napi_env,
    }
    impl Env {
        pub fn is_exception_pending(&self) -> bool {
            let mut result = MaybeUninit::uninit();
            NAPI!(
                self,
                napi_is_exception_pending(self.env, result.as_mut_ptr()),
                result.assume_init()
            )
            .unwrap()
        }

        pub fn undefined(&self) -> Value {
            let mut result = MaybeUninit::uninit();
            NAPI!(
                self,
                napi_get_undefined(self.env, result.as_mut_ptr()),
                result.assume_init()
            )
            .unwrap()
        }

        pub fn null(&self) -> Value {
            let mut result = MaybeUninit::uninit();
            NAPI!(
                self,
                napi_get_null(self.env, result.as_mut_ptr()),
                result.assume_init()
            )
            .unwrap()
        }
    }
    impl From<napi_env> for Env {
        fn from(env: napi_env) -> Self {
            Env { env }
        }
    }

    pub struct CallbackInfo {
        env: Env,
        info: napi_callback_info,
        this: Value,
        args: Vec<Value>,
        undefined: Value,
    }
    impl CallbackInfo {
        pub fn env(&self) -> Env {
            self.env
        }

        pub fn len(&self) -> usize {
            self.args.len()
        }

        pub fn this(&self) -> Value {
            self.this
        }

        pub fn new_target(&self) -> Value {
            let mut nt = MaybeUninit::uninit();
            unsafe {
                NAPI!(
                    self.env,
                    napi_get_new_target(self.env.env, self.info, nt.as_mut_ptr()),
                    ()
                )
                .unwrap();
                nt.assume_init()
            }
        }
    }
    impl std::ops::Index<usize> for CallbackInfo {
        type Output = Value;
        fn index(&self, index: usize) -> &Self::Output {
            match self.args.get(index) {
                Some(v) => v,
                None => &self.undefined,
            }
        }
    }

    pub type Callback = fn(info: &CallbackInfo) -> Result<Value, Value>;

    #[derive(Debug)]
    pub struct Error {
        message: String,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(self, f)
        }
    }

    impl std::error::Error for Error {}

    pub fn create_string_utf8(env: Env, string: &str) -> Result<Value, Error> {
        let mut value = MaybeUninit::uninit();
        let len = string.bytes().len();
        let cstr = CString::new(string).unwrap();
        NAPI!(
            env,
            napi_create_string_utf8(env.env, cstr.as_ptr(), len, value.as_mut_ptr()),
            value.assume_init()
        )
    }

    unsafe extern "C" fn callback_dispatch(env: napi_env, info: napi_callback_info) -> napi_value {
        let env = Env::from(env);
        let mut argc = 0;
        let mut data = MaybeUninit::uninit();
        let mut this = MaybeUninit::uninit();
        NAPI!(
            env,
            napi_get_cb_info(
                env.env,
                info,
                &mut argc,
                std::ptr::null_mut(),
                this.as_mut_ptr(),
                data.as_mut_ptr(),
            ),
            ()
        )
        .unwrap();

        let mut args = Vec::with_capacity(argc);
        NAPI!(
            env,
            napi_get_cb_info(
                env.env,
                info,
                &mut argc,
                args.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            ()
        )
        .unwrap();
        args.set_len(argc);

        let cbinfo = CallbackInfo {
            env,
            info,
            this: this.assume_init(),
            args,
            undefined: env.undefined(),
        };
        match std::mem::transmute::<_, Callback>(data.assume_init())(&cbinfo) {
            Ok(v) => v,
            Err(e) => {
                NAPI!(env, napi_throw(env.env, e), ()).unwrap();
                std::ptr::null_mut()
            }
        }
    }

    pub fn create_function(env: Env, name: &str, cb: Callback) -> Result<Value, Error> {
        let mut value = MaybeUninit::uninit();
        let cstr = CString::new(name).unwrap();
        NAPI!(
            env,
            napi_create_function(
                env.env,
                cstr.as_ptr(),
                name.len(),
                Some(callback_dispatch),
                cb as *mut std::ffi::c_void,
                value.as_mut_ptr(),
            ),
            value.assume_init()
        )
    }

    pub fn set_named_property(env: Env, obj: Value, name: &str, value: Value) -> Result<(), Error> {
        let cstr = CString::new(name).unwrap();
        NAPI!(
            env,
            napi_set_named_property(env.env, obj, cstr.as_ptr(), value),
            ()
        )
    }

    #[macro_export]
    macro_rules! napi_init {
        ($init:ident) => {
            use napi_sys::*;
            #[no_mangle]
            extern "C" fn napi_register_module_v1(
                env: napi_env,
                exports: napi_value,
            ) -> napi_value {
                $init(Env::from(env), exports);
                exports
            }
        };
    }
}

pub use napi::{CallbackInfo, Env, Value};

fn read_file(info: &CallbackInfo) -> Result<Value, Value> {
    let string = std::fs::read_to_string("/sandbox/hello.txt").unwrap();
    Ok(napi::create_string_utf8(info.env(), &string).unwrap())
}

fn throw(info: &CallbackInfo) -> Result<Value, Value> {
    Err(info[0])
}

fn init(env: Env, exports: Value) {
    napi::set_named_property(
        env,
        exports,
        "readFile",
        napi::create_function(env, "readFile", read_file).unwrap(),
    )
    .unwrap();
    napi::set_named_property(
        env,
        exports,
        "throw",
        napi::create_function(env, "throw", throw).unwrap(),
    )
    .unwrap();
}
napi_init!(init);
