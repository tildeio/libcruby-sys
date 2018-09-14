use libc::c_int;

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
}

tests! {
    // See intern::tests::test_hash_foreach
}
