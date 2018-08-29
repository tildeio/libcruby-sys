use libc::{c_char, c_int, c_long, uintptr_t};
use std::mem::transmute;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VALUE(uintptr_t);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ID(uintptr_t);

// TODO: explain why this is ok
unsafe impl Sync for ID {}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ANYARGS<T>(extern "C" fn() -> T);

macro_rules! impl_from_arity {
    ($name:ident $(, $arg:ty)*) => {
        impl<T> ANYARGS<T> {
            #[inline(always)]
            pub fn $name(func: extern "C" fn($($arg),*) -> T) -> Self {
                unsafe { ANYARGS(transmute(func)) }
            }
        }
    }
}

impl_from_arity!(from_arity_0);
impl_from_arity!(from_arity_1, VALUE);
impl_from_arity!(from_arity_2, VALUE, VALUE);
impl_from_arity!(from_arity_3, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_4, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_5, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_6, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_7, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_8, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_9, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_10, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_11, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_12, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);

extern {
    #[link_name = "RS_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "RS_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "RS_Qnil"]
    pub static Qnil: VALUE;

    pub static rb_mKernel: VALUE;
    pub static rb_mComparable: VALUE;
    pub static rb_mEnumerable: VALUE;
    pub static rb_mErrno: VALUE;
    pub static rb_mFileTest: VALUE;
    pub static rb_mGC: VALUE;
    pub static rb_mMath: VALUE;
    pub static rb_mProcess: VALUE;
    pub static rb_mWaitReadable: VALUE;
    pub static rb_mWaitWritable: VALUE;

    pub static rb_cBasicObject: VALUE;
    pub static rb_cObject: VALUE;
    pub static rb_cArray: VALUE;
    pub static rb_cBinding: VALUE;
    pub static rb_cClass: VALUE;
    pub static rb_cData: VALUE;
    pub static rb_cDir: VALUE;
    pub static rb_cEncoding: VALUE;
    pub static rb_cEnumerator: VALUE;
    pub static rb_cFalseClass: VALUE;
    pub static rb_cFile: VALUE;
    pub static rb_cComplex: VALUE;
    pub static rb_cFloat: VALUE;
    pub static rb_cHash: VALUE;
    pub static rb_cIO: VALUE;
    pub static rb_cInteger: VALUE;
    pub static rb_cMatch: VALUE;
    pub static rb_cMethod: VALUE;
    pub static rb_cModule: VALUE;
    pub static rb_cNilClass: VALUE;
    pub static rb_cNumeric: VALUE;
    pub static rb_cProc: VALUE;
    pub static rb_cRandom: VALUE;
    pub static rb_cRange: VALUE;
    pub static rb_cRational: VALUE;
    pub static rb_cRegexp: VALUE;
    pub static rb_cStat: VALUE;
    pub static rb_cString: VALUE;
    pub static rb_cStruct: VALUE;
    pub static rb_cSymbol: VALUE;
    pub static rb_cThread: VALUE;
    pub static rb_cTime: VALUE;
    pub static rb_cTrueClass: VALUE;
    pub static rb_cUnboundMethod: VALUE;

    pub static rb_eException: VALUE;
    pub static rb_eStandardError: VALUE;
    pub static rb_eSystemExit: VALUE;
    pub static rb_eInterrupt: VALUE;
    pub static rb_eSignal: VALUE;
    pub static rb_eArgError: VALUE;
    pub static rb_eEOFError: VALUE;
    pub static rb_eIndexError: VALUE;
    pub static rb_eStopIteration: VALUE;
    pub static rb_eKeyError: VALUE;
    pub static rb_eRangeError: VALUE;
    pub static rb_eIOError: VALUE;
    pub static rb_eRuntimeError: VALUE;
    pub static rb_eSecurityError: VALUE;
    pub static rb_eSystemCallError: VALUE;
    pub static rb_eThreadError: VALUE;
    pub static rb_eTypeError: VALUE;
    pub static rb_eZeroDivError: VALUE;
    pub static rb_eNotImpError: VALUE;
    pub static rb_eNoMemError: VALUE;
    pub static rb_eNoMethodError: VALUE;
    pub static rb_eFloatDomainError: VALUE;
    pub static rb_eLocalJumpError: VALUE;
    pub static rb_eSysStackError: VALUE;
    pub static rb_eRegexpError: VALUE;
    pub static rb_eEncodingError: VALUE;
    pub static rb_eEncCompatError: VALUE;

    pub static rb_eScriptError: VALUE;
    pub static rb_eNameError: VALUE;
    pub static rb_eSyntaxError: VALUE;
    pub static rb_eLoadError: VALUE;

    pub static rb_eMathDomainError: VALUE;

    pub fn rb_intern(cstr: *const c_char) -> ID;
    pub fn rb_intern2(ptr: *const c_char, len: c_long) -> ID;

    pub fn rb_sym2id(symbol: VALUE) -> ID;
    pub fn rb_id2sym(id: ID) -> VALUE;

    pub fn rb_define_class(name: *const c_char, superclass: VALUE) -> VALUE;

    pub fn rb_define_method(class: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);
    pub fn rb_define_module_function(module: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);
    pub fn rb_undef_method(class: VALUE, name: *const c_char);
}

tests! {
    use super::*;
    use super::super::testing::{Assertions, ToRuby, lazy_eval};

    use std::ffi::CString;

    #[test]
    fn test_false(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("false"), unsafe { Qfalse });
    }

    #[test]
    fn test_true(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("true"), unsafe{ Qtrue });
    }

    #[test]
    fn test_nil(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("nil"), unsafe { Qnil });
    }

    #[test]
    fn test_m_kernel(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Kernel"), unsafe { rb_mKernel });
    }

    #[test]
    fn test_m_comparable(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Comparable"), unsafe { rb_mComparable });
    }

    #[test]
    fn test_m_enumerable(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Enumerable"), unsafe { rb_mEnumerable });
    }

    #[test]
    fn test_m_errno(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Errno"), unsafe { rb_mErrno });
    }

    #[test]
    fn test_m_file_test(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::FileTest"), unsafe { rb_mFileTest });
    }

    #[test]
    fn test_m_gc(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::GC"), unsafe { rb_mGC });
    }

    #[test]
    fn test_m_math(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Math"), unsafe { rb_mMath });
    }

    #[test]
    fn test_m_process(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Process"), unsafe { rb_mProcess });
    }

    #[test]
    fn test_m_wait_readable(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::IO::WaitReadable"), unsafe { rb_mWaitReadable });
    }

    #[test]
    fn test_m_wait_writable(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::IO::WaitWritable"), unsafe { rb_mWaitWritable });
    }

    #[test]
    fn test_c_basic_object(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::BasicObject"), unsafe { rb_cBasicObject });
    }

    #[test]
    fn test_c_object(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Object"), unsafe { rb_cObject });
    }

    #[test]
    fn test_c_array(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Array"), unsafe { rb_cArray });
    }

    #[test]
    fn test_c_binding(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Binding"), unsafe { rb_cBinding });
    }

    #[test]
    fn test_c_class(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Class"), unsafe { rb_cClass });
    }

    #[test]
    fn test_c_data(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Data"), unsafe { rb_cData });
    }

    #[test]
    fn test_c_dir(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Dir"), unsafe { rb_cDir });
    }

    #[test]
    fn test_c_encoding(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Encoding"), unsafe { rb_cEncoding });
    }

    #[test]
    fn test_c_enumerator(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Enumerator"), unsafe { rb_cEnumerator });
    }

    #[test]
    fn test_c_false_class(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::FalseClass"), unsafe { rb_cFalseClass });
    }

    #[test]
    fn test_c_file(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::File"), unsafe { rb_cFile });
    }

    #[test]
    fn test_c_complex(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Complex"), unsafe { rb_cComplex });
    }

    #[test]
    fn test_c_float(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Float"), unsafe { rb_cFloat });
    }

    #[test]
    fn test_c_hash(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Hash"), unsafe { rb_cHash });
    }

    #[test]
    fn test_c_io(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::IO"), unsafe { rb_cIO });
    }

    #[test]
    fn test_c_integer(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Integer"), unsafe { rb_cInteger });
    }

    #[test]
    fn test_c_match(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::MatchData"), unsafe { rb_cMatch });
    }

    #[test]
    fn test_c_method(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Method"), unsafe { rb_cMethod });
    }

    #[test]
    fn test_c_module(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Module"), unsafe { rb_cModule });
    }

    #[test]
    fn test_c_nil_class(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::NilClass"), unsafe { rb_cNilClass });
    }

    #[test]
    fn test_c_numeric(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Numeric"), unsafe { rb_cNumeric });
    }

    #[test]
    fn test_c_proc(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Proc"), unsafe { rb_cProc });
    }

    #[test]
    fn test_c_random(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Random"), unsafe { rb_cRandom });
    }

    #[test]
    fn test_c_range(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Range"), unsafe { rb_cRange });
    }

    #[test]
    fn test_c_rational(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Rational"), unsafe { rb_cRational });
    }

    #[test]
    fn test_c_regexp(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Regexp"), unsafe { rb_cRegexp });
    }

    #[test]
    fn test_c_stat(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::File::Stat"), unsafe { rb_cStat });
    }

    #[test]
    fn test_c_string(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::String"), unsafe { rb_cString });
    }

    #[test]
    fn test_c_struct(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Struct"), unsafe { rb_cStruct });
    }

    #[test]
    fn test_c_symbol(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Symbol"), unsafe { rb_cSymbol });
    }

    #[test]
    fn test_c_thread(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Thread"), unsafe { rb_cThread });
    }

    #[test]
    fn test_c_time(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Time"), unsafe { rb_cTime });
    }

    #[test]
    fn test_c_true_class(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::TrueClass"), unsafe { rb_cTrueClass });
    }

    #[test]
    fn test_c_unbound_method(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::UnboundMethod"), unsafe { rb_cUnboundMethod });
    }

    #[test]
    fn test_e_exception(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Exception"), unsafe { rb_eException });
    }

    #[test]
    fn test_e_standard_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::StandardError"), unsafe { rb_eStandardError });
    }

    #[test]
    fn test_e_system_exit(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SystemExit"), unsafe { rb_eSystemExit });
    }

    #[test]
    fn test_e_interrupt(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Interrupt"), unsafe { rb_eInterrupt });
    }

    #[test]
    fn test_e_signal(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SignalException"), unsafe { rb_eSignal });
    }

    #[test]
    fn test_e_arg_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::ArgumentError"), unsafe { rb_eArgError });
    }

    #[test]
    fn test_e_eof_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::EOFError"), unsafe { rb_eEOFError });
    }

    #[test]
    fn test_e_index_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::IndexError"), unsafe { rb_eIndexError });
    }

    #[test]
    fn test_e_stop_iteration(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::StopIteration"), unsafe { rb_eStopIteration });
    }

    #[test]
    fn test_e_key_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::KeyError"), unsafe { rb_eKeyError });
    }

    #[test]
    fn test_e_range_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::RangeError"), unsafe { rb_eRangeError });
    }

    #[test]
    fn test_e_io_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::IOError"), unsafe { rb_eIOError });
    }

    #[test]
    fn test_e_runtime_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::RuntimeError"), unsafe { rb_eRuntimeError });
    }

    #[test]
    fn test_e_security_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SecurityError"), unsafe { rb_eSecurityError });
    }

    #[test]
    fn test_e_system_call_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SystemCallError"), unsafe { rb_eSystemCallError });
    }

    #[test]
    fn test_e_thread_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::ThreadError"), unsafe { rb_eThreadError });
    }

    #[test]
    fn test_e_type_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::TypeError"), unsafe { rb_eTypeError });
    }

    #[test]
    fn test_e_zero_div_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::ZeroDivisionError"), unsafe { rb_eZeroDivError });
    }

    #[test]
    fn test_e_not_imp_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::NotImplementedError"), unsafe { rb_eNotImpError });
    }

    #[test]
    fn test_e_no_mem_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::NoMemoryError"), unsafe { rb_eNoMemError });
    }

    #[test]
    fn test_e_no_method_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::NoMethodError"), unsafe { rb_eNoMethodError });
    }

    #[test]
    fn test_e_float_domain_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::FloatDomainError"), unsafe { rb_eFloatDomainError });
    }

    #[test]
    fn test_e_local_jump_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::LocalJumpError"), unsafe { rb_eLocalJumpError });
    }

    #[test]
    fn test_e_system_stack_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SystemStackError"), unsafe { rb_eSysStackError });
    }

    #[test]
    fn test_e_regexp_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::RegexpError"), unsafe { rb_eRegexpError });
    }

    #[test]
    fn test_e_encoding_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::EncodingError"), unsafe { rb_eEncodingError });
    }

    #[test]
    fn test_e_enc_compat_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Encoding::CompatibilityError"), unsafe { rb_eEncCompatError });
    }

    #[test]
    fn test_e_script_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::ScriptError"), unsafe { rb_eScriptError });
    }

    #[test]
    fn test_e_name_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::NameError"), unsafe { rb_eNameError });
    }

    #[test]
    fn test_e_syntax_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::SyntaxError"), unsafe { rb_eSyntaxError });
    }

    #[test]
    fn test_e_load_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::LoadError"), unsafe { rb_eLoadError });
    }

    #[test]
    fn test_e_math_domain_error(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("::Math::DomainError"), unsafe { rb_eMathDomainError });
    }

    #[test]
    fn test_id(assert: &mut Assertions) {
        let foo1 = unsafe { rb_intern(cstr!("foo")) };
        let foo2 = unsafe { rb_intern(CString::new("foo").unwrap().into_raw()) };
        let foo3 = unsafe { rb_intern2("foobar".as_ptr() as *const c_char, 3 as c_long) };

        let bar = unsafe { rb_intern(cstr!("bar")) };

        assert.rs_eq(foo1, foo1);
        assert.rs_eq(foo1, foo2);
        assert.rs_eq(foo1, foo3);

        assert.rs_ne(foo1, bar);
        assert.rs_ne(foo2, bar);
        assert.rs_ne(foo3, bar);

        assert.rs_eq(foo1, unsafe { rb_sym2id(rb_id2sym(foo1)) });
        assert.rs_eq(foo2, unsafe { rb_sym2id(rb_id2sym(foo2)) });
        assert.rs_eq(foo3, unsafe { rb_sym2id(rb_id2sym(foo3)) });

        assert.rs_ne(foo1, unsafe { rb_sym2id(rb_id2sym(bar)) });
        assert.rs_ne(foo2, unsafe { rb_sym2id(rb_id2sym(bar)) });
        assert.rs_ne(foo3, unsafe { rb_sym2id(rb_id2sym(bar)) });

        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo1) });
        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo2) });
        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo3) });
        assert.rb_eq(lazy_eval(":bar"), unsafe { rb_id2sym(bar) });
    }

    #[test]
    fn test_define_class(assert: &mut Assertions) {
        let foo = unsafe { rb_define_class(cstr!("TestDefineClass__Foo"), rb_cObject) };
        let bar = unsafe { rb_define_class(cstr!("TestDefineClass__Bar"), foo) };

        assert.rb_eq(lazy_eval("::TestDefineClass__Foo"), foo);
        assert.rb_eq(lazy_eval("::TestDefineClass__Bar"), bar);

        assert.rb_eq(lazy_eval("::TestDefineClass__Foo.class"), unsafe { rb_cClass });
        assert.rb_eq(lazy_eval("::TestDefineClass__Bar.class"), unsafe { rb_cClass });

        assert.rb_eq(lazy_eval("::TestDefineClass__Foo.superclass"), unsafe { rb_cObject });
        assert.rb_eq(lazy_eval("::TestDefineClass__Bar.superclass"), foo);
    }

    #[test]
    fn test_define_method(assert: &mut Assertions) {
        extern "C" fn __test_define_method_arity_0__(_self: VALUE) -> VALUE {
            "__test_define_method_arity_0__ works!".to_ruby()
        }

        unsafe {
            rb_define_method(
                rb_cObject,
                cstr!("__test_define_method_arity_0__"),
                ANYARGS::from_arity_1(__test_define_method_arity_0__),
                0
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.new.__test_define_method_arity_0__"),
            "__test_define_method_arity_0__ works!".to_ruby()
        );

        extern "C" fn __test_define_method_arity_3__(_self: VALUE, foo_sym: VALUE, bar_sym: VALUE, baz_sym: VALUE) -> VALUE {
            if unsafe { rb_sym2id(foo_sym) != rb_intern(cstr!("foo")) } {
                "__test_define_method_arity_3__ failed (expected :foo for first argument)".to_ruby()
            } else if unsafe { rb_sym2id(bar_sym) != rb_intern(cstr!("bar")) } {
                "__test_define_method_arity_3__ failed (expected :bar for second argument)".to_ruby()
            } else if unsafe { rb_sym2id(baz_sym) != rb_intern(cstr!("baz")) } {
                "__test_define_method_arity_3__ failed (expected :baz for third argument)".to_ruby()
            } else {
                "__test_define_method_arity_3__ works!".to_ruby()
            }
        }

        unsafe {
            rb_define_method(
                rb_cObject,
                cstr!("__test_define_method_arity_3__"),
                ANYARGS::from_arity_4(__test_define_method_arity_3__),
                3
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.new.__test_define_method_arity_3__(:foo, :bar, :baz)"),
            "__test_define_method_arity_3__ works!".to_ruby()
        );
    }

    #[test]
    fn test_define_module_function(assert: &mut Assertions) {
        extern "C" fn __test_define_module_function_arity_0__(_self: VALUE) -> VALUE {
            "__test_define_module_function_arity_0__ works!".to_ruby()
        }

        unsafe {
            rb_define_module_function(
                rb_mKernel,
                cstr!("__test_define_module_function_arity_0__"),
                ANYARGS::from_arity_1(__test_define_module_function_arity_0__),
                0
            );
        }

        assert.rb_eq(
            lazy_eval("::Kernel.__test_define_module_function_arity_0__"),
            "__test_define_module_function_arity_0__ works!".to_ruby()
        );

        assert.rb_eq(
            lazy_eval("__test_define_module_function_arity_0__"),
            "__test_define_module_function_arity_0__ works!".to_ruby()
        );

        extern "C" fn __test_define_module_function_arity_3__(_self: VALUE, foo_sym: VALUE, bar_sym: VALUE, baz_sym: VALUE) -> VALUE {
            if unsafe { rb_sym2id(foo_sym) != rb_intern(cstr!("foo")) } {
                "__test_define_module_function_arity_3__ failed (expected :foo for first argument)".to_ruby()
            } else if unsafe { rb_sym2id(bar_sym) != rb_intern(cstr!("bar")) } {
                "__test_define_module_function_arity_3__ failed (expected :bar for second argument)".to_ruby()
            } else if unsafe { rb_sym2id(baz_sym) != rb_intern(cstr!("baz")) } {
                "__test_define_module_function_arity_3__ failed (expected :baz for third argument)".to_ruby()
            } else {
                "__test_define_module_function_arity_3__ works!".to_ruby()
            }
        }

        unsafe {
            rb_define_method(
                rb_mKernel,
                cstr!("__test_define_module_function_arity_3__"),
                ANYARGS::from_arity_4(__test_define_module_function_arity_3__),
                3
            );
        }

        assert.rb_eq(
            lazy_eval("::Kernel.__test_define_module_function_arity_3__(:foo, :bar, :baz)"),
            "__test_define_module_function_arity_3__ works!".to_ruby()
        );

        assert.rb_eq(
            lazy_eval("__test_define_module_function_arity_3__(:foo, :bar, :baz)"),
            "__test_define_module_function_arity_3__ works!".to_ruby()
        );
    }

    #[test]
    fn test_undef_method(assert: &mut Assertions) {
        let class = unsafe {
            rb_define_class(
                cstr!("TestUndefClass"),
                rb_cObject
            )
        };

        unsafe { rb_undef_method(class, cstr!("to_s")) };

        assert.rb_eq(
            lazy_eval("::TestUndefClass.new.respond_to?(:to_s)"),
            unsafe { Qfalse }
        );

        assert.rb_eq(
            lazy_eval(r#"
                exception = begin
                TestUndefClass.new.to_s
                rescue => e
                e
                end

                exception.class
            "#),
            unsafe { rb_eNoMethodError }
        );
    }

    #[test]
    fn test_undef_module_method(assert: &mut Assertions) {
        extern "C" fn __test_undef_module_method__(_self: VALUE) -> VALUE {
            "__test_undef_module_method__ works!".to_ruby()
        }

        unsafe {
            rb_define_method(
                rb_mKernel,
                cstr!("__test_undef_module_method__"),
                ANYARGS::from_arity_1(__test_undef_module_method__),
                0
            );
        }

        // NOTE: Ideally we would test this just to make sure things are set up correctly,
        //   but due to the lazy_eval this won't pass.
        //
        // assert.rb_eq(
        //     lazy_eval("::Kernel.__test_undef_module_method__"),
        //     "__test_undef_module_method__ works!".to_ruby()
        // );

        unsafe {
            rb_undef_method(
                rb_mKernel,
                cstr!("__test_undef_module_method__")
            );
        }

        assert.rb_eq(
            lazy_eval("::Kernel.respond_to?(:__test_undef_module_method__)"),
            unsafe { Qfalse }
        );
    }
}
