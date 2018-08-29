#![recursion_limit="1024"]

extern crate libc;

#[cfg(test)]
#[macro_use]
extern crate cstr_macro;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
pub mod testing;

#[cfg(test)]
macro_rules! tests {
    ( $( $item:tt )* ) => { rb_tests! { $($item)* } }
}

#[cfg(not(test))]
macro_rules! tests {
    ( $( $item:tt )* ) => {}
}

#[cfg(test)]
macro_rules! init {
    ( $( $item:tt )* ) => { rb_init! { $($item)* } }
}

#[cfg(not(test))]
macro_rules! init {
    ( $( $item:tt )* ) => { $($item)* }
}

init! {
    mod ruby;
    mod intern;
    mod encoding;

    pub use ruby::*;
    pub use intern::*;
    pub use encoding::*;
}
