use super::*;
use libc::{c_char, c_int, c_long};
use std::cmp::PartialEq;
use std::fmt::Debug;

lazy_static! {
    static ref TESTING: VALUE = {
        unsafe {
            rb_const_get(rb_cObject, rb_intern(cstr!("Testing")))
        }
    };

    static ref ASSERTIONS: VALUE = {
        unsafe {
            rb_const_get(*TESTING, rb_intern(cstr!("Assertions")))
        }
    };

    static ref ASSERT_EQ: VALUE = {
        unsafe {
            rb_const_get(*ASSERTIONS, rb_intern(cstr!("Equal")))
        }
    };

    static ref ASSERT_NE: VALUE = {
        unsafe {
            rb_const_get(*ASSERTIONS, rb_intern(cstr!("NotEqual")))
        }
    };

    static ref ASSERT_OK: VALUE = {
        unsafe {
            rb_const_get(*ASSERTIONS, rb_intern(cstr!("Ok")))
        }
    };

    static ref ASSERT_NIL: VALUE = {
        unsafe {
            rb_const_get(*ASSERTIONS, rb_intern(cstr!("Nil")))
        }
    };

    static ref LAZY_VALUE: VALUE = {
        unsafe {
            rb_const_get(*TESTING, rb_intern(cstr!("LazyValue")))
        }
    };

    pub static ref TESTS: VALUE = {
        unsafe {
            rb_const_get(*TESTING, rb_intern(cstr!("Tests")))
        }
    };
}

pub trait ToRuby {
    fn to_ruby(&self) -> VALUE;
}

impl ToRuby for str {
    fn to_ruby(&self) -> VALUE {
        let ptr = self.as_ptr() as *const c_char;
        let len = self.len() as c_long;
        unsafe { rb_utf8_str_new(ptr, len) }
    }
}

impl ToRuby for String {
    fn to_ruby(&self) -> VALUE {
        str::to_ruby(&self)
    }
}

macro_rules! count {
    () => { 0 };
    ( $item:tt $($rest:tt)* ) => { 1 + count!($($rest)*) };
}

macro_rules! new {
    ( $class:expr, $($arg:expr),* ) => {
        {
            let args: Vec<VALUE> = vec![$($arg),*];
            let argc = count!($(($arg))*) as c_int;
            let argv = args.as_ptr();

            unsafe { rb_class_new_instance(argc, argv, $class) }
        }
    };
}

#[derive(Debug)]
pub struct Assertions {
    assertions: Vec<VALUE>,
}

impl Assertions {
    pub fn new() -> Self {
        Assertions { assertions: vec![] }
    }

    pub fn rb_eq(&mut self, expected: VALUE, actual: VALUE) {
        self.assertions.push(new!(*ASSERT_EQ, expected, actual));
    }

    pub fn rs_eq<T: Debug + PartialEq<U>, U: Debug + PartialEq<T>>(&mut self, lhs: T, rhs: U) {
        let predicate = {
            if lhs == rhs {
                unsafe { Qtrue }
            } else {
                unsafe { Qfalse }
            }
        };

        let message = format!("{:?} == {:?}", lhs, rhs).to_ruby();

        self.assertions.push(new!(*ASSERT_OK, predicate, message));
    }

    pub fn rb_ne(&mut self, expected: VALUE, actual: VALUE) {
        self.assertions.push(new!(*ASSERT_NE, expected, actual));
    }

    pub fn rs_ne<T: Debug + PartialEq<U>, U: Debug + PartialEq<T>>(&mut self, lhs: T, rhs: U) {
        let predicate = {
            if lhs != rhs {
                unsafe { Qtrue }
            } else {
                unsafe { Qfalse }
            }
        };

        let message = format!("{:?} != {:?}", lhs, rhs).to_ruby();

        self.assertions.push(new!(*ASSERT_OK, predicate, message));
    }

    pub fn rb_nil(&mut self, value: VALUE) {
        self.assertions.push(new!(*ASSERT_NIL, value));
    }
}

impl ToRuby for Assertions {
    fn to_ruby(&self) -> VALUE {
        let len = self.assertions.len() * 2;
        let array = unsafe { rb_ary_new_capa(len as c_long) };

        for assertion in &self.assertions {
            unsafe { rb_ary_push(array, *assertion) };
        }

        array
    }
}

#[macro_export]
macro_rules! rb_tests {
    ( $( $item:tt )* ) => {
        #[cfg(test)]
        pub mod tests {
            #[allow(unused_attributes)]
            #[allow(unused_imports)]
            #[macro_use]
            use $crate::testing;

            rb_test_items! {
                items: {},
                init: {},
                rest: { $($item)* }
            }
        }
    };
}

#[macro_export]
macro_rules! rb_test_items {
    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: {}
    ) => {
        $($item)*

        pub fn init() { $($init)* }
    };

    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: { #[test] fn $name:ident($arg:ident : $type:ty) { $($body:tt)* } $($rest:tt)* }
    ) => {
        rb_test_items! {
            items: {
                $($item)*

                #[cfg(test)]
                #[no_mangle]
                pub extern "C" fn $name(_: $crate::VALUE) -> VALUE {
                    fn test_case($arg: $type) {
                        $($body)*
                    }

                    let mut assertions = $crate::testing::Assertions::new();

                    test_case(&mut assertions);

                    $crate::testing::ToRuby::to_ruby(&assertions)
                }
            },
            init: {
                $($init)*

                {
                    let name = cstr!(stringify!($name));
                    let func = $crate::ANYARGS::from_arity_1($name);
                    let arity = 0;

                    unsafe { rb_define_module_function(*$crate::testing::TESTS, name, func, arity) };
                }
            },
            rest: { $($rest)* }
        }
    };

    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: { $next:tt $($rest:tt)* }
    ) => {
        rb_test_items! {
            items: { $($item)* $next },
            init: { $($init)* },
            rest: { $($rest)* }
        }
    };
}

#[macro_export]
macro_rules! rb_init {
    ( $( $item:tt )* ) => {
        rb_init_items! {
            items: {},
            init: {},
            rest: { $($item)* }
        }
    };
}

#[macro_export]
macro_rules! rb_init_items {
    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: {}
    ) => {
        $($item)*

        #[cfg(test)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn Init_tests() {
            $($init)*
        }
    };

    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: { mod $name:ident; $($rest:tt)* }
    ) => {
        rb_init_items! {
            items: { $($item)* pub mod $name; },
            init: { $($init)* $name::tests::init(); },
            rest: { $($rest)* }
        }
    };

    (
        items: { $($item:tt)* },
        init: { $($init:tt)* },
        rest: { $next:tt $($rest:tt)* }
    ) => {
        rb_init_items! {
            items: { $($item)* $next },
            init: { $($init)* },
            rest: { $($rest)* }
        }
    };
}

pub fn lazy_eval(code: &str) -> VALUE {
    new!(*LAZY_VALUE, code.to_ruby())
}
