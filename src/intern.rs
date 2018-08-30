use super::*;
use libc::{c_char, c_int, c_long};

extern {
    /// Creates a new array with no elements
    ///
    /// * Returns an [`rb_cArray`](static.rb_cArray.html)
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L50) |
    ///     [array.c](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L498-L502) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions)
    pub fn rb_ary_new() -> VALUE;

    /// Creates a new array with capacity
    ///
    /// * `capacity` - number of elements to pre-allocate space for
    /// * Returns an [`rb_cArray`](static.rb_cArray.html)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eArgError`](static.rb_eArgError.html)
    ///     * if `capacity` is negative.
    ///     * if `capacity` is greater than [`ARY_MAX_SIZE`](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L32).
    ///
    /// # Miscellaneous
    ///
    /// [`rb_ary_new2`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L90)
    /// is currently an alias for this.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L51) |
    ///     [array.c](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L492-L496) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions)
    pub fn rb_ary_new_capa(capacity: c_long) -> VALUE;

    /// Pushes an item on to the end of an array, returning the array itself.
    ///
    /// * `array`: an instance of [`rb_cArray`](static.rb_cArray.html)
    /// * `item`: any Ruby object
    /// * Returns `array`
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eIndexError`](static.rb_eIndexError.html)
    ///     * if array size would exceed [`ARY_MAX_SIZE`](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L32).
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L67) |
    ///     [array.c](https://github.com/ruby/ruby/blob/v2_5_1/array.c#L924-L934) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Array+Functions)
    pub fn rb_ary_push(array: VALUE, item: VALUE) -> VALUE;

    /// Makes a new string from a char pointer of given length, treating it as UTF-8 encoded.
    ///
    /// # Safety
    ///
    /// No known issues.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L705) |
    ///     [string.c](https://github.com/ruby/ruby/blob/v2_5_1/string.c#L751-L757) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-String+Functions)
    pub fn rb_utf8_str_new(ptr: *const c_char, len: c_long) -> VALUE;

    /// Makes a new instance of a class
    ///
    /// * `argc` - number of arguments passed
    /// * `argv` - pointer list of Ruby objects
    /// * `class` - a [`rb_cClass`](static.rb_cClass.html)
    /// * Returns a new instance of `class`
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eFatal`](https://ruby-doc.org/core-2.5.1/fatal.html)
    ///     * if `class` is not a class
    /// * [`rb_eTypeError`](static.rb_eTypeError.html)
    ///     * if `class` cannot be alloc'ed.
    /// * Other exceptions may be raised by user defined code
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L405) |
    ///     [object.c](https://github.com/ruby/ruby/blob/v2_5_1/object.c#L2169-L2174)
    pub fn rb_class_new_instance(argc: c_int, argv: *const VALUE, class: VALUE) -> VALUE;

    /// Fetches a constant from a module or class
    ///
    /// * `class`: a [`rb_cClass`](static.rb_cClass.html) or [`rb_cModule`](static.rb_cModule.html)
    /// * `name`: the `ID` of the interned name
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `class` is not a module or a class.
    /// * Undefined behavior  if the `ID` is invalid.
    ///
    /// ## Exceptions
    ///
    /// * An undefined constant may cause an exception to be raised,
    /// especially since this path may call a user-defined method.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L930) |
    ///     [variable.c](https://github.com/ruby/ruby/blob/v2_5_1/variable.c#L2300-L2304) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Accessing+the+Variables+and+Constants)
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
    /// * `obj`: any Ruby object
    /// * Returns a [`rb_cString`](static.rb_cString.html)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * May call user defined code that could raise an exception
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L568) |
    ///     [object.c](https://github.com/ruby/ruby/blob/v2_5_1/object.c#L655-L670)
    pub fn rb_inspect(obj: VALUE) -> VALUE;

    /// Looks up the nearest ancestor class of the object, skipping
    /// singleton classes or module inclusions.
    ///
    /// It returns the object itself if it is neither a singleton class or a
    /// module. Otherwise, it returns the ancestor class or a falsey value if
    /// nothing is found.
    ///
    /// * `obj`: any Ruby object
    /// * Returns a [`rb_cClass`](static.rb_cClass.html) or a falsey `VALUE`
    ///
    /// # Safety
    ///
    /// No known issues.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L584) |
    ///     [object.c](https://github.com/ruby/ruby/blob/v2_5_1/object.c#L276-L280)
    pub fn rb_obj_class(obj: VALUE) -> VALUE;

    /// Defines a singleton method on a class
    ///
    /// See [`rb_define_method`](fn.rb_define_method.html) for details on arguments.
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eTypeError`](static.rb_eTypeError.html)
    ///     * if Ruby built-in class does not allow singletons to be defined.
    ///
    /// See also [`rb_define_method`](fn.rb_define_method.html#safety).
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [intern.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/intern.h#L210) |
    ///     [class.c](https://github.com/ruby/ruby/blob/v2_5_1/class.c#L1715-L1719) |
    ///     documentation: [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition),
    ///                     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+Definition)
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

