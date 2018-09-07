use super::*;
use libc::c_int;

extern {
    /// Returns the encoding index of the provided Ruby object
    ///
    /// * `obj` - a variety of different Ruby object types are accepted
    /// * Returns an integer value corresponding to the encoding index or `-1`
    /// if encoding cannot be determined.
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    /// # Defined In
    ///
    /// * **2.3:** [encoding.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/encoding.h#L118)
    /// * **2.4:** [encoding.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/encoding.h#L119)
    /// * **2.5:**
    ///     [encoding.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/encoding.h#L119)
    ///     [encoding.c](https://github.com/ruby/ruby/blob/v2_5_1/encoding.c#L783-L815)
    /// * **2.6:** [encoding.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/encoding.h#L123)
    pub fn rb_enc_get_index(obj: VALUE) -> c_int;

    /// Returns the encoding index for UTF-8
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    /// # Defined In
    ///
    /// * **2.3:** [encoding.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/encoding.h#L261)
    /// * **2.4:** [encoding.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/encoding.h#L265)
    /// * **2.5:**
    ///     [encoding.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/encoding.h#L265)
    ///     [encoding.c](https://github.com/ruby/ruby/blob/v2_5_1/encoding.c#L1339-L1343)
    /// * **2.6:** [encoding.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/encoding.h#L269)
    pub fn rb_utf8_encindex() -> c_int;
}

tests! {
    use super::*;
    use super::super::testing::Assertions;
    use libc::{c_long, c_char};

    #[test]
    fn test_rb_utf8_encindex(assert: &mut Assertions) {
        let string = "foo";
        let ptr = string.as_ptr() as *const c_char;
        let len = string.len() as c_long;
        let ruby_string = unsafe { rb_utf8_str_new(ptr, len) };
        assert.rs_eq(unsafe { rb_enc_get_index(ruby_string) }, unsafe { rb_utf8_encindex() });
    }
}
