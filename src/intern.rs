use super::*;
use libc::{c_char, c_int, c_long};

/// Generally used as return values from traversing callbacks
///
/// _Descriptions below are the usual behavior and may vary depending on the specific case._
///
/// * [`ST_CONTINUE`] - continue traversing
/// * [`ST_STOP`] - stop traversing
/// * [`ST_DELETE`] - delete item and continue traversing
/// * [`ST_CHECK`]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct st_retval(c_int);

extern {
    /// Generally used as a return value from traversing callbacks to indicate
    /// that traversing should continue.
    #[link_name = "RS_ST_CONTINUE"]
    pub static ST_CONTINUE: st_retval;

    /// Generally used as a return value from traversing callbacks to indicate
    /// that traversing should stop.
    #[link_name = "RS_ST_STOP"]
    pub static ST_STOP: st_retval;

    /// Generally used as a return value from traversing callbacks to indicate
    /// that traversing should stop.
    #[link_name = "RS_ST_DELETE"]
    pub static ST_DELETE: st_retval;

    /// Generally used as a return value from traversing callbacks. It's unclear
    /// when this should be used in user-defined code.
    #[link_name = "RS_ST_CHECK"]
    pub static ST_CHECK: st_retval;

    /// Constructs a new, empty array.
    ///
    /// * Returns an [`Array`](rb_cArray)
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions)
    ///
    //+ c-func: array.c `VALUE rb_ary_new(void)`
    pub fn rb_ary_new() -> VALUE;

    /// Constructs a new, empty array with the specified capacity.
    ///
    /// * `capacity` - number of elements to pre-allocate space for
    /// * Returns an [`Array`](rb_cArray)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`ArgumentError`](rb_eArgError)
    ///     * if `capacity` is negative.
    ///     * if `capacity` is greater than [`ARY_MAX_SIZE`](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L32).
    ///
    /// # Miscellaneous
    ///
    /// [`rb_ary_new2`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L90)
    /// is currently an alias for this.
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions)
    ///
    //+ c-func: array.c `VALUE rb_ary_new_capa(long capa)`
    pub fn rb_ary_new_capa(capacity: c_long) -> VALUE;

    /// Pushes an item on to the end of an array, returning the array itself.
    ///
    /// * `array` - an instance of [`Array`](rb_cArray)
    /// * `item` - any Ruby object
    /// * Returns `array`
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `array` is not an `Array`
    ///
    /// ## Exceptions
    ///
    /// * [`IndexError`](rb_eIndexError)
    ///     * if array size would exceed [`ARY_MAX_SIZE`](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L32).
    /// * [`FrozenError`](rb_eFrozenError)
    ///     * if `hash` is frozen
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [C API](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions) |
    ///     [`Array#<<`](https://ruby-doc.org/core-2.5.1/Array.html#method-i-3C-3C)
    ///
    //+ c-func: array.c `VALUE rb_ary_push(VALUE, VALUE)`
    pub fn rb_ary_push(array: VALUE, item: VALUE) -> VALUE;

    /// Returns the element at the given index or [`nil`](Qnil) if the index is
    /// out-of-bounds for the array.
    ///
    /// * `array` - an [`Array`](rb_cArray)
    /// * `idx` - the offset to retrieve. A negative value will count from the end of the array.
    ///
    /// # Safety
    ///
    /// * This function performs bounds checks which should be safer than some options.
    /// * Undefined behavior if `array` is not an `Array`
    ///
    /// # Miscellaneous
    ///
    /// * [`RARRAY_AREF`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1035) has
    /// a similar function but does not do bounds checking.
    ///
    //+ c-func: array.c `VALUE rb_ary_entry(VALUE, long)`
    pub fn rb_ary_entry(array: VALUE, idx: c_long) -> VALUE;

    /// # Constructs a new, empty hash.
    ///
    /// * Returns a [`Hash`](rb_cHash)
    ///
    /// # Safety
    ///
    /// * No known issues
    ///
    //+ c-func: hash.c `VALUE rb_hash_new(void)`
    pub fn rb_hash_new() -> VALUE;

    /// Inserts a key-value pair into the hash.
    ///
    /// If the key already exists in the hash, the value will be replaced
    ///
    /// * `hash` - a `[Hash]`(rb_cHash)
    /// * `key` - any Ruby object
    /// * `value` - any Ruby object
    /// * Returns `value`
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `hash` is not a `Hash`
    /// * Undefined behavior if `key`'s value is changed while it is in use as a key
    /// (an unfrozen `String` passed as a key will be duplicated and frozen)
    ///
    /// ## Exceptions
    ///
    /// * [`FrozenError`](rb_eFrozenError)
    ///     * if `hash` is frozen
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [`Hash#[]=`](https://ruby-doc.org/core-2.5.1/Hash.html#method-i-5B-5D-3D) |
    ///     [`Hash#store`](https://ruby-doc.org/core-2.5.1/Hash.html#method-i-store)
    ///
    //+ c-func: hash.c `VALUE rb_hash_aset(VALUE, VALUE, VALUE)`
    pub fn rb_hash_aset(hash: VALUE, key: VALUE, val: VALUE) -> VALUE;

    /// Executes a function on each key-value pair in a hash.
    ///
    /// * `hash` - a [`Hash`](rb_cHash)
    /// * `func` - a function that will be called for each key-value pair of the hash
    ///     * Returns `st_retval`:
    ///         * [`ST_CONTINUE`]: iteration will continue
    ///         * [`ST_CHECK`]: iteration will continue, it appears that `ST_CONTINUE` is preferred
    ///         * [`ST_DELETE`]: entry will be deleted and iteration will continue
    ///         * [`ST_STOP`]: iteration will stop
    /// * `farg` - a Ruby object to be passed through to the `func`
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `hash` is not a `Hash`
    ///
    /// ## Exceptions
    ///
    /// * [`RuntimeError`](rb_eRuntimeError)
    ///     * if `hash` is modified by an iteration
    ///     * if a rehash occurred during an interation
    /// * User-defined iterator method could also raise an exception.
    ///
    //+ c-func: hash.c `void rb_hash_foreach(VALUE, int (*)(ANYARGS), VALUE)`
    pub fn rb_hash_foreach(hash: VALUE, func: extern "C" fn(key: VALUE, val: VALUE, farg: VALUE) -> st_retval, farg: VALUE);

    /// Constructs a new Ruby string from a UTF-8 encoded C string of a given length.
    ///
    /// # Safety
    ///
    /// * Undefined behavior if the `ptr` does not point to a valid UTF-8 C string
    /// of length greater than or equal to `len`.
    /// * Undefined behavior if nul-bytes are included withing the C string.
    ///
    /// No known issues.
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-String+Functions)
    ///
    //+ c-func: string.c `VALUE rb_utf8_str_new(const char*, long)`
    pub fn rb_utf8_str_new(ptr: *const c_char, len: c_long) -> VALUE;

    /// Constructs a new instance of a class by calling its allocator and constructor
    /// (`alloc` and `initialize`) as `::new` normally would.
    ///
    /// * `argc` - number of arguments passed
    /// * `argv` - pointer to the arguments, passed as a C array
    /// * `class` - a [`Class`](rb_cClass)
    /// * Returns a new instance of `class`
    ///
    /// # Safety
    ///
    /// * `argv` must point to a location in memory containing at least `argc` number
    /// of Ruby objects, (i.e. a valid C `VALUE` array of at least size `argc`)
    ///
    /// ## Exceptions
    ///
    /// * [`fatal`](https://ruby-doc.org/core-2.5.1/fatal.html)
    ///     * if `class` is not a class
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `class` cannot be alloc'ed.
    /// * Other exceptions may be raised by user defined code
    ///
    //+ c-func: object.c `VALUE rb_class_new_instance(int, const VALUE*, VALUE)`
    pub fn rb_class_new_instance(argc: c_int, argv: *const VALUE, class: VALUE) -> VALUE;

    /// Fetches a constant from a module or class.
    ///
    /// * `class` - a [`Class`](rb_cClass) or [`Module`](rb_cModule)
    /// * `name` - the `ID` of the interned name
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `class` is not a module or a class.
    /// * Undefined behavior if the `ID` is invalid.
    ///
    /// ## Exceptions
    ///
    /// * An undefined constant may cause an exception to be raised,
    /// especially since this path may invoke user-defined code (via
    /// `const_missing` and friends).
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Accessing+the+Variables+and+Constants)
    ///
    //+ c-func: variable.c `VALUE rb_const_get(VALUE, ID)`
    pub fn rb_const_get(class: VALUE, name: ID) -> VALUE;

    /// Returns a human-readable Ruby string representation of an object,
    /// similarly to Ruby's `Object#inspect`.
    ///
    /// Unlike `Object#inspect`, it escapes characters to keep the result
    /// compatible to the default internal or external encoding.
    /// If the default internal or external encoding is ASCII compatible,
    /// the encoding of the inspected result must be compatible with it.
    /// If the default internal or external encoding is ASCII incompatible,
    /// the result must be ASCII only.
    ///
    /// * `obj` - any Ruby object
    /// * Returns a [`String`](rb_cString)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * May call user-defined code that could raise an exception
    ///
    //+ c-func: object.c `VALUE rb_inspect(VALUE)`
    pub fn rb_inspect(obj: VALUE) -> VALUE;

    /// Looks up the nearest ancestor class of the object, skipping
    /// singleton classes or module inclusions.
    ///
    /// It returns the object itself if it is neither a singleton class or a
    /// module. Otherwise, it returns the ancestor class or a falsey value if
    /// nothing is found.
    ///
    /// * `obj` - any Ruby object
    /// * Returns a [`Class`](rb_cClass) or a falsey `VALUE`
    ///
    /// Similar to [`CLASS_OF`] except that it will not return a singleton class.
    ///
    /// # Safety
    ///
    /// No known issues.
    ///
    //+ c-func: object.c `VALUE rb_obj_class(VALUE)`
    pub fn rb_obj_class(obj: VALUE) -> VALUE;

    /// Defines a singleton method on a class.
    ///
    /// See [`rb_define_method`](rb_define_method) for details on arguments.
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if Ruby built-in class does not allow singletons to be defined.
    ///
    /// See also [`rb_define_method`](fn.rb_define_method.html#safety).
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition),
    ///     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+Definition)
    ///
    //+ c-func: class.c `void rb_define_singleton_method(VALUE, const char*, VALUE(*)(ANYARGS), int)`
    pub fn rb_define_singleton_method(class: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);
}

tests! {
    use super::*;
    use super::super::testing::{Assertions, ToRuby, lazy_eval};
    use std::ptr::null;

    #[test]
    fn test_ary_new(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new() });

        let arr1 = unsafe { rb_ary_new() };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };

        assert.rb_eq(lazy_eval("[true, false, nil]"), arr1);

        let arr2 = unsafe { rb_ary_new() };

        unsafe { rb_ary_push(arr2, "hello".to_ruby()) };
        unsafe { rb_ary_push(arr2, "world!".to_ruby()) };

        assert.rb_eq(lazy_eval("['hello', 'world!']"), arr2);
    }

    #[test]
    fn test_ary_new_capa(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new_capa(0) });
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new_capa(100) });

        let arr1 = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };

        assert.rb_eq(lazy_eval("[true, false, nil]"), arr1);

        let arr2 = unsafe { rb_ary_new_capa(0) };

        unsafe { rb_ary_push(arr2, "hello".to_ruby()) };
        unsafe { rb_ary_push(arr2, "world!".to_ruby()) };

        assert.rb_eq(lazy_eval("['hello', 'world!']"), arr2);
    }

    #[test]
    fn test_ary_entry(assert: &mut Assertions) {
        let arr1 = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };
        unsafe { rb_ary_push(arr1, "hello".to_ruby()) };

        assert.rb_eq(unsafe { Qtrue }, unsafe { rb_ary_entry(arr1, 0) });
        assert.rb_eq(unsafe { Qfalse }, unsafe { rb_ary_entry(arr1, 1) });
        assert.rb_nil(unsafe { rb_ary_entry(arr1, 2) });
        assert.rb_eq("hello".to_ruby(), unsafe { rb_ary_entry(arr1, 3) });
        assert.rb_nil(unsafe { rb_ary_entry(arr1, 4) });
        assert.rb_eq("hello".to_ruby(), unsafe { rb_ary_entry(arr1, -1) });
    }

    #[test]
    fn test_utf8_str_new(assert: &mut Assertions) {
        let static_str = "static str";
        let static_str_ptr = static_str.as_ptr() as *const c_char;

        let heap_string = format!("heap string");
        let heap_string_ptr = heap_string.as_ptr() as *const c_char;

        let unicode = "❤️💛💚💙💜";
        let unicode_ptr = unicode.as_ptr() as *const c_char;

        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(null(), 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(static_str_ptr, 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(heap_string_ptr, 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(unicode_ptr, 0) });

        assert.rb_eq(lazy_eval("'static'"), unsafe { rb_utf8_str_new(static_str_ptr, 6) });
        assert.rb_eq(lazy_eval("'heap'"), unsafe { rb_utf8_str_new(heap_string_ptr, 4) });
        assert.rb_eq(lazy_eval("'❤️'"), unsafe { rb_utf8_str_new(unicode_ptr, 6) });

        assert.rb_eq(lazy_eval("'static str'"), unsafe { rb_utf8_str_new(static_str_ptr, 10) });
        assert.rb_eq(lazy_eval("'heap string'"), unsafe { rb_utf8_str_new(heap_string_ptr, 11) });
        assert.rb_eq(lazy_eval("'❤️💛💚💙💜'"), unsafe { rb_utf8_str_new(unicode_ptr, 22) });
    }

    #[test]
    fn test_hash_create_and_set(assert: &mut Assertions) {
        let hash = unsafe { rb_hash_new() };

        unsafe {
            rb_hash_aset(hash, "foo".to_ruby(), "bar".to_ruby());
            rb_hash_aset(hash, "baz".to_ruby(), "qux".to_ruby());
        }

        assert.rb_eq(unsafe { rb_inspect(hash) }, r#"{"foo"=>"bar", "baz"=>"qux"}"#.to_ruby());
    }

    #[test]
    fn test_hash_foreach(assert: &mut Assertions) {
        extern "C" fn __test_hash_foreach__(key: VALUE, val: VALUE, arg: VALUE) -> st_retval {
            unsafe {
                rb_ary_push(arg, key);
                rb_ary_push(arg, val);

                let id = rb_intern_str(key);

                if      id == rb_intern(cstr!("foo"))  { ST_CONTINUE }
                else if id == rb_intern(cstr!("baz"))  { ST_CHECK }
                else if id == rb_intern(cstr!("hoge")) { ST_DELETE }
                else                                   { ST_STOP }
            }
        }

        let hash = unsafe { rb_hash_new() };
        let ary = unsafe { rb_ary_new() };

        unsafe {
            rb_hash_aset(hash, "foo".to_ruby(), "bar".to_ruby());
            rb_hash_aset(hash, "baz".to_ruby(), "qux".to_ruby());
            rb_hash_aset(hash, "hoge".to_ruby(), "piyo".to_ruby());
            rb_hash_aset(hash, "wibble".to_ruby(), "wobble".to_ruby());
            rb_hash_aset(hash, "eggs".to_ruby(), "spam".to_ruby());
            rb_hash_foreach(hash, __test_hash_foreach__, ary);
        }

        assert.rb_eq(lazy_eval("['foo', 'bar', 'baz', 'qux', 'hoge', 'piyo', 'wibble', 'wobble']"), ary);
        assert.rb_eq(lazy_eval(r#"{"foo"=>"bar", "baz"=>"qux", "wibble"=>"wobble", "eggs"=>"spam"}"#), hash);
    }

    #[test]
    fn test_class_new_instance(assert: &mut Assertions) {
        assert.rb_ne(lazy_eval("Object.new"), unsafe { rb_class_new_instance(0, null(), rb_cObject) });

        assert.rb_eq(lazy_eval("[]"), unsafe { rb_class_new_instance(0, null(), rb_cArray) });

        let ary_source = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(ary_source, Qtrue) };
        unsafe { rb_ary_push(ary_source, Qfalse) };
        unsafe { rb_ary_push(ary_source, Qnil) };

        let ary_cloned = unsafe { rb_class_new_instance(1, &ary_source, rb_cArray) };

        assert.rs_ne(ary_source, ary_cloned);
        assert.rb_eq(lazy_eval("[true, false, nil]"), ary_cloned);

        assert.rb_eq(
            lazy_eval("('a'..'z')"),
            unsafe {
                rb_class_new_instance(
                    2,
                    vec!["a".to_ruby(), "z".to_ruby()].as_ptr(),
                    rb_cRange
                )
            }
        );

        assert.rb_eq(
            lazy_eval("('a'...'z')"),
            unsafe {
                rb_class_new_instance(
                    3,
                    vec!["a".to_ruby(), "z".to_ruby(), Qtrue].as_ptr(),
                    rb_cRange
                )
            }
        );
    }

    #[test]
    fn test_const_get(assert: &mut Assertions) {
        assert.rs_eq(
            unsafe { rb_mKernel },
            unsafe { rb_const_get(rb_cObject, rb_intern(cstr!("Kernel"))) }
        );

        assert.rs_eq(
            unsafe { rb_cObject },
            unsafe { rb_const_get(rb_cObject, rb_intern(cstr!("Object"))) }
        );

        assert.rs_eq(
            unsafe { rb_eEncCompatError },
            unsafe { rb_const_get(rb_cEncoding, rb_intern(cstr!("CompatibilityError"))) }
        );
    }

    #[test]
    fn test_inspect(assert: &mut Assertions) {
        assert.rb_eq(
            lazy_eval("Kernel.inspect"),
            unsafe { rb_inspect(rb_mKernel) }
        );

        assert.rb_eq(
            lazy_eval("Object.inspect"),
            unsafe { rb_inspect(rb_cObject) }
        );

        extern "C" fn __test_inspect__(_self: VALUE) -> VALUE {
            "__test_inspect__ works!".to_ruby()
        }

        let class = unsafe { rb_define_class(cstr!("TestInspect"), rb_cObject) };

        unsafe {
            rb_define_method(
                class,
                cstr!("inspect"),
                ANYARGS::from_arity_1(__test_inspect__),
                0
            );
        }

        assert.rb_eq(
            unsafe { rb_inspect(rb_class_new_instance(0, null(), class)) },
            "__test_inspect__ works!".to_ruby()
        );
    }


    #[test]
    fn test_obj_class(assert: &mut Assertions) {
        let class = unsafe { rb_obj_class(rb_cObject) };
        let module = unsafe { rb_obj_class(rb_mKernel) };
        let instance = unsafe { rb_obj_class(rb_class_new_instance(0, null(), rb_cObject)) };

        assert.rb_eq(lazy_eval("::Class"), class);
        assert.rb_eq(lazy_eval("::Module"), module);
        assert.rb_eq(lazy_eval("::Object"), instance);
    }

    #[test]
    fn test_define_singleton_method(assert: &mut Assertions) {
        extern "C" fn __test_define_singleton_method_arity_0__(_self: VALUE) -> VALUE {
            "__test_define_singleton_method_arity_0__ works!".to_ruby()
        }

        unsafe {
            rb_define_singleton_method(
                rb_cObject,
                cstr!("__test_define_singleton_method_arity_0__"),
                ANYARGS::from_arity_1(__test_define_singleton_method_arity_0__),
                0
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.__test_define_singleton_method_arity_0__"),
            "__test_define_singleton_method_arity_0__ works!".to_ruby()
        );

        extern "C" fn __test_define_singleton_method_arity_3__(_self: VALUE, foo_sym: VALUE, bar_sym: VALUE, baz_sym: VALUE) -> VALUE {
            if unsafe { rb_sym2id(foo_sym) != rb_intern(cstr!("foo")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :foo for first argument)".to_ruby()
            } else if unsafe { rb_sym2id(bar_sym) != rb_intern(cstr!("bar")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :bar for second argument)".to_ruby()
            } else if unsafe { rb_sym2id(baz_sym) != rb_intern(cstr!("baz")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :baz for third argument)".to_ruby()
            } else {
                "__test_define_singleton_method_arity_3__ works!".to_ruby()
            }
        }

        unsafe {
            rb_define_singleton_method(
                rb_cObject,
                cstr!("__test_define_singleton_method_arity_3__"),
                ANYARGS::from_arity_4(__test_define_singleton_method_arity_3__),
                3
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.__test_define_singleton_method_arity_3__(:foo, :bar, :baz)"),
            "__test_define_singleton_method_arity_3__ works!".to_ruby()
        );
    }
}

