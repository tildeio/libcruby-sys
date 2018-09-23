use libc::{c_char, c_int, c_uint, c_long, c_ulong, c_longlong, c_ulonglong, c_double, size_t, uintptr_t};
use std::mem::transmute;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VALUE(uintptr_t);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ID(uintptr_t);

// `ID` is just an index into a Ruby array representing all symbols
// and interned strings. Since this type is just an index into that
// array, not an actual pointer to a Ruby object, it is safe to share
// between threads.
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
impl_from_arity!(from_arity_13, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_14, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_15, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);

/// Data types used within the Ruby C interpreter
///
/// * [`T_OBJECT`] - ordinary Ruby object
/// * [`T_CLASS`] - [`Class`](rb_cClass)
/// * [`T_MODULE`] - [`Module`](rb_cModule)
/// * [`T_FLOAT`] - [`Float`](rb_cFloat)
/// * [`T_STRING`] - [`String`](rb_cString)
/// * [`T_REGEXP`] - regular expression ([`Regexp`](rb_cRegexp))
/// * [`T_ARRAY`] - [`Array`](rb_cArray)
/// * [`T_HASH`] - [`Hash`](rb_cHash)
/// * [`T_STRUCT`] - [`Struct`](rb_cStruct)
/// * [`T_BIGNUM`] - `Bignum`
/// * [`T_FILE`] - [`IO`](rb_cIO)
/// * [`T_DATA`]
/// * [`T_MATCH`] - [`MatchData`](rb_cMatch) object
/// * [`T_COMPLEX`] - complex number ([`Complex`](rb_cComplex))
/// * [`T_RATIONAL`] - rational number ([`Rational`](rb_cRational))
/// * [`T_NIL`] - `nil`
/// * [`T_TRUE`] - `true`
/// * [`T_FALSE`] - `false`
/// * [`T_SYMBOL`] - [`Symbol`](rb_cSymbol)
/// * [`T_FIXNUM`] - `Fixnum` (31-bit or 63-bit integer)
///
/// In addition, there are several other types used internally:
///
/// * [`T_NONE`]
/// * [`T_IMEMO`]
/// * [`T_NODE`] - syntax tree node
/// * [`T_ICLASS`] - included module
/// * [`T_UNDEF`] - undefined
/// * [`T_ZOMBIE`] - object awaiting finalization
///
/// # Ruby Documentation
///
/// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Data+Types)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct ruby_value_type(c_int);

extern {
    /// This type denotes an ordinary Ruby object.
    #[link_name = "RS_T_OBJECT"]
    pub static T_OBJECT: ruby_value_type;

    /// This type denotes a Ruby [`Class`](rb_cClass).
    #[link_name = "RS_T_CLASS"]
    pub static T_CLASS: ruby_value_type;

    /// This type denotes a Ruby [`Module`](rb_cModule).
    #[link_name = "RS_T_MODULE"]
    pub static T_MODULE: ruby_value_type;

    /// This type denotes a Ruby [`Float`](rb_cFloat).
    #[link_name = "RS_T_FLOAT"]
    pub static T_FLOAT: ruby_value_type;

    /// This type denotes a Ruby [`String`](rb_cString).
    #[link_name = "RS_T_STRING"]
    pub static T_STRING: ruby_value_type;

    /// This type denotes a Ruby regular expression ([`Regexp`](rb_cRegexp)).
    #[link_name = "RS_T_REGEXP"]
    pub static T_REGEXP: ruby_value_type;

    /// This type denotes a Ruby [`Array`](rb_cArray).
    #[link_name = "RS_T_ARRAY"]
    pub static T_ARRAY: ruby_value_type;

    /// This type denotes a Ruby [`Hash`](rb_cHash).
    #[link_name = "RS_T_HASH"]
    pub static T_HASH: ruby_value_type;

    /// This type denotes a Ruby [`Struct`](rb_cStruct).
    #[link_name = "RS_T_STRUCT"]
    pub static T_STRUCT: ruby_value_type;

    /// This type denotes a Ruby `Bignum`.
    #[link_name = "RS_T_BIGNUM"]
    pub static T_BIGNUM: ruby_value_type;

    /// This type denotes a Ruby [`IO`](rb_cIO).
    #[link_name = "RS_T_FILE"]
    pub static T_FILE: ruby_value_type;

    /// This type denotes a Ruby `Data` object.
    #[link_name = "RS_T_DATA"]
    pub static T_DATA: ruby_value_type;

    /// This type denotes a Ruby [`MatchData`](rb_cMatch) object.
    #[link_name = "RS_T_MATCH"]
    pub static T_MATCH: ruby_value_type;

    ///  This type denotes a Ruby complex number ([`Complex`](rb_cComplex)).
    #[link_name = "RS_T_COMPLEX"]
    pub static T_COMPLEX: ruby_value_type;

    /// This type denotes a Ruby rational number ([`Rational`](rb_cRational)).
    #[link_name = "RS_T_RATIONAL"]
    pub static T_RATIONAL: ruby_value_type;

    /// This type denotes a Ruby `nil`.
    #[link_name = "RS_T_NIL"]
    pub static T_NIL: ruby_value_type;

    /// This type denotes a Ruby `true`.
    #[link_name = "RS_T_TRUE"]
    pub static T_TRUE: ruby_value_type;

    /// This type denotes a Ruby `false`.
    #[link_name = "RS_T_FALSE"]
    pub static T_FALSE: ruby_value_type;

    /// This type denotes a Ruby [`Symbol`](rb_cSymbol).
    #[link_name = "RS_T_SYMBOL"]
    pub static T_SYMBOL: ruby_value_type;

    /// This type denotes a Ruby `Fixnum` (31-bit or 63-bit integer).
    #[link_name = "RS_T_FIXNUM"]
    pub static T_FIXNUM: ruby_value_type;

    /// This type is generally only for use in Ruby internals
    #[link_name = "RS_T_NONE"]
    pub static T_NONE: ruby_value_type;

    /// This type denotes undefined.
    ///
    /// Generally only for use in Ruby internals.
    #[link_name = "RS_T_UNDEF"]
    pub static T_UNDEF: ruby_value_type;

    /// This type is generally only for use in Ruby internals
    #[link_name = "RS_T_IMEMO"]
    pub static T_IMEMO: ruby_value_type;

    /// This type denotes a Ruby syntax tree node.
    ///
    /// Generally only for use in Ruby internals.
    #[link_name = "RS_T_NODE"]
    pub static T_NODE: ruby_value_type;

    /// This type denotes an included Ruby module.
    ///
    /// Generally only for use in Ruby internals.
    #[link_name = "RS_T_ICLASS"]
    pub static T_ICLASS: ruby_value_type;

    /// This type denotes a Ruby object awaiting finalization.
    ///
    /// Generally only for use in Ruby internals.
    #[link_name = "RS_T_ZOMBIE"]
    pub static T_ZOMBIE: ruby_value_type;


    #[link_name = "RS_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "RS_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "RS_Qnil"]
    pub static Qnil: VALUE;


    /// The `Kernel` module
    ///
    //+ c-module: object.c `VALUE rb_mKernel`
    pub static rb_mKernel: VALUE;

    /// The `Comparable` module
    ///
    //+ c-module: compar.c `VALUE rb_mComparable`
    pub static rb_mComparable: VALUE;

    /// The `Enumerable` module
    ///
    //+ c-module: enum.c `VALUE rb_mEnumerable`
    pub static rb_mEnumerable: VALUE;

    /// The `Errno` module
    ///
    //+ c-module: error.c `VALUE rb_mErrno`
    pub static rb_mErrno: VALUE;

    /// The `FileTest` module
    ///
    //+ c-module: file.c `VALUE rb_mFileTest`
    pub static rb_mFileTest: VALUE;

    /// The `GC` module
    ///
    //+ c-module: gc.c `VALUE rb_mGC`
    pub static rb_mGC: VALUE;

    /// The `Math` module
    ///
    //+ c-module: math.c `VALUE rb_mMath`
    pub static rb_mMath: VALUE;

    /// The `Process` module
    ///
    //+ c-module: process.c `VALUE rb_mProcess`
    pub static rb_mProcess: VALUE;

    /// The `IO::WaitReadable` module
    ///
    //+ c-module: io.c `VALUE rb_mWaitReadable`
    pub static rb_mWaitReadable: VALUE;

    /// The `IO::WaitWritable` module
    ///
    //+ c-module: io.c `VALUE rb_mWaitWritable`
    pub static rb_mWaitWritable: VALUE;


    /// The `BasicObject` class
    ///
    //+ c-class: class.c `VALUE rb_cBasicObject`
    pub static rb_cBasicObject: VALUE;

    /// The `Object` class
    ///
    //+ c-class: class.c `VALUE rb_cObject`
    pub static rb_cObject: VALUE;

    /// The `Array` class
    ///
    //+ c-class: array.c `VALUE rb_cArray`
    pub static rb_cArray: VALUE;

    /// The `Binding` class
    ///
    //+ c-class: proc.c `VALUE rb_cBinding`
    pub static rb_cBinding: VALUE;

    /// The `Class` class
    ///
    //+ c-class: class.c `VALUE rb_cClass`
    pub static rb_cClass: VALUE;

    /// The `Dir` class
    ///
    //+ c-class: dir.c `VALUE rb_cDir`
    pub static rb_cDir: VALUE;

    /// The `Encoding` class
    ///
    //+ c-class: encoding.c `VALUE rb_cEncoding`
    pub static rb_cEncoding: VALUE;

    /// The `Enumerator` class
    ///
    //+ c-class: enumerator.c `VALUE rb_cEnumerator`
    pub static rb_cEnumerator: VALUE;

    /// The `FalseClass` class
    ///
    //+ c-class: object.c `VALUE rb_cFalseClass`
    pub static rb_cFalseClass: VALUE;

    /// The `File` class
    ///
    //+ c-class: file.c `VALUE rb_cFile`
    pub static rb_cFile: VALUE;

    /// The `Complex` class
    ///
    //+ c-class: complex.c `VALUE rb_cComplex`
    pub static rb_cComplex: VALUE;

    /// The `Float` class
    ///
    //+ c-class: numeric.c `VALUE rb_cFloat`
    pub static rb_cFloat: VALUE;

    /// The `Hash` class
    ///
    //+ c-class: hash.c `VALUE rb_cHash`
    pub static rb_cHash: VALUE;

    /// The `IO` class
    ///
    //+ c-class: io.c `VALUE rb_cIO`
    pub static rb_cIO: VALUE;

    /// The `Integer` class
    ///
    //+ c-class: numeric.c `VALUE rb_cInteger`
    pub static rb_cInteger: VALUE;

    /// The `MatchData` class
    ///
    //+ c-class: re.c `VALUE rb_cMatch`
    pub static rb_cMatch: VALUE;

    /// The `Method` class
    ///
    //+ c-class: proc.c `VALUE rb_cMethod`
    pub static rb_cMethod: VALUE;

    /// The `Module` class
    ///
    //+ c-class: class.c `VALUE rb_cModule`
    pub static rb_cModule: VALUE;

    /// The `NilClass` class
    ///
    //+ c-class: object.c `VALUE rb_cNilClass`
    pub static rb_cNilClass: VALUE;

    /// The `Numeric` class
    ///
    //+ c-class: numeric.c `VALUE rb_cNumeric`
    pub static rb_cNumeric: VALUE;

    /// The `Proc` class
    ///
    //+ c-class: proc.c `VALUE rb_cProc`
    pub static rb_cProc: VALUE;

    /// The `Random` class
    ///
    //+ c-class: random.c `VALUE rb_cRandom`
    pub static rb_cRandom: VALUE;

    /// The `Range` class
    ///
    //+ c-class: range.c `VALUE rb_cRange`
    pub static rb_cRange: VALUE;

    /// The `Rational` class
    ///
    //+ c-class: rational.c `VALUE rb_cRational`
    pub static rb_cRational: VALUE;

    /// The `Regexp` class
    ///
    //+ c-class: re.c `VALUE rb_cRegexp`
    pub static rb_cRegexp: VALUE;

    /// The `File::Stat` class
    ///
    //+ c-class: file.c `VALUE rb_cStat`
    pub static rb_cStat: VALUE;

    /// The `String` class
    ///
    //+ c-class: string.c `VALUE rb_cString`
    pub static rb_cString: VALUE;

    /// The `Struct` class
    ///
    //+ c-class: struct.c `VALUE rb_cStruct`
    pub static rb_cStruct: VALUE;

    /// The `Symbol` class
    ///
    //+ c-class: string.c `VALUE rb_cSymbol`
    pub static rb_cSymbol: VALUE;

    /// The `Thread` class
    ///
    //+ c-class: vm.c `VALUE rb_cThread`
    pub static rb_cThread: VALUE;

    /// The `Time` class
    ///
    //+ c-class: time.c `VALUE rb_cTime`
    pub static rb_cTime: VALUE;

    /// The `TrueClass` class
    ///
    //+ c-class: object.c `VALUE rb_cTrueClass`
    pub static rb_cTrueClass: VALUE;

    /// The `UnboundMethod` class
    ///
    //+ c-class: proc.c `VALUE rb_cUnboundMethod`
    pub static rb_cUnboundMethod: VALUE;


    /// The `Exception` class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/Exception.html)
    ///
    //+ c-class: error.c `VALUE rb_eException`
    pub static rb_eException: VALUE;

    /// The `StandardError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/StandardError.html)
    ///
    //+ c-class: error.c `VALUE rb_eStandardError`
    pub static rb_eStandardError: VALUE;

    /// The `SystemExit` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/SystemExit.html)
    ///
    //+ c-class: error.c `VALUE rb_eSystemExit`
    pub static rb_eSystemExit: VALUE;

    /// The `Interrupt` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/Interrupt.html)
    ///
    //+ c-class: error.c `VALUE rb_eInterrupt`
    pub static rb_eInterrupt: VALUE;

    /// The `Signal` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/Signal.html)
    ///
    //+ c-class: error.c `VALUE rb_eSignal`
    pub static rb_eSignal: VALUE;

    /// The `ArgumentError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/ArgumentError.html)
    ///
    //+ c-class: error.c `VALUE rb_eArgError`
    pub static rb_eArgError: VALUE;

    /// The `EOFError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/EOFError.html)
    ///
    //+ c-class: io.c `VALUE rb_eEOFError`
    pub static rb_eEOFError: VALUE;

    /// The `IndexError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/IndexError.html)
    ///
    //+ c-class: error.c `VALUE rb_eIndexError`
    pub static rb_eIndexError: VALUE;

    /// The `StopIteration` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/StopIteration.html)
    ///
    //+ c-class: enumerator.c `VALUE rb_eStopIteration`
    pub static rb_eStopIteration: VALUE;

    /// The `KeyError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/KeyError.html)
    ///
    //+ c-class: error.c `VALUE rb_eKeyError`
    pub static rb_eKeyError: VALUE;

    /// The `RangeError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/RangeError.html)
    ///
    //+ c-class: error.c `VALUE rb_eRangeError`
    pub static rb_eRangeError: VALUE;

    /// The `IOError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/IOError.html)
    ///
    //+ c-class: io.c `VALUE rb_eIOError`
    pub static rb_eIOError: VALUE;

    /// The `RuntimeError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/RuntimeError.html)
    ///
    //+ c-class: error.c `VALUE rb_eRuntimeError`
    pub static rb_eRuntimeError: VALUE;

    /// The `SecurityError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/SecurityError.html)
    ///
    //+ c-class: error.c `VALUE rb_eSecurityError`
    pub static rb_eSecurityError: VALUE;

    /// The `SystemCallError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/SystemCallError.html)
    ///
    //+ c-class: error.c `VALUE rb_eSystemCallError`
    pub static rb_eSystemCallError: VALUE;

    /// The `ThreadError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/ThreadError.html)
    ///
    //+ c-class: thread.c `VALUE rb_eThreadError`
    pub static rb_eThreadError: VALUE;

    /// The `TypeError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/TypeError.html)
    ///
    //+ c-class: error.c `VALUE rb_eTypeError`
    pub static rb_eTypeError: VALUE;

    /// The `ZeroDivisionError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/ZeroDivisionError.html)
    ///
    //+ c-class: numeric.c `VALUE rb_eZeroDivError`
    pub static rb_eZeroDivError: VALUE;

    /// The `NotImplementedError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/NotImplementedError.html)
    ///
    //+ c-class: error.c `VALUE rb_eNotImpError`
    pub static rb_eNotImpError: VALUE;

    /// The `NoMemoryError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/NoMemoryError.html)
    ///
    //+ c-class: error.c `VALUE rb_eNoMemError`
    pub static rb_eNoMemError: VALUE;

    /// The `NoMethodError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/NoMethodError.html)
    ///
    //+ c-class: error.c `VALUE rb_eNoMethodError`
    pub static rb_eNoMethodError: VALUE;

    /// The `FloatDomainError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/FloatDomainError.html)
    ///
    //+ c-class: numeric.c `VALUE rb_eFloatDomainError`
    pub static rb_eFloatDomainError: VALUE;

    /// The `LocalJumpError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/LocalJumpError.html)
    ///
    //+ c-class: proc.c `VALUE rb_eLocalJumpError`
    pub static rb_eLocalJumpError: VALUE;

    /// The `SystemStackError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/SystemStackError.html)
    ///
    //+ c-class: proc.c `VALUE rb_eSysStackError`
    pub static rb_eSysStackError: VALUE;

    /// The `RegexpError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/RegexpError.html)
    ///
    //+ c-class: re.c `VALUE rb_eRegexpError`
    pub static rb_eRegexpError: VALUE;

    /// The `EncodingError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/EncodingError.html)
    ///
    //+ c-class: error.c `VALUE rb_eEncodingError`
    pub static rb_eEncodingError: VALUE;

    /// The `Encoding::CompatibilityError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/Encoding/CompatibilityError.html)
    ///
    //+ c-class: error.c `VALUE rb_eEncCompatError`
    pub static rb_eEncCompatError: VALUE;

    /// The `ScriptError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/ScriptError.html)
    ///
    //+ c-class: error.c `VALUE rb_eScriptError`
    pub static rb_eScriptError: VALUE;

    /// The `NameError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/NameError.html)
    ///
    //+ c-class: error.c `VALUE rb_eNameError`
    pub static rb_eNameError: VALUE;

    /// The `SyntaxError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/SyntaxError.html)
    ///
    //+ c-class: error.c `VALUE rb_eSyntaxError`
    pub static rb_eSyntaxError: VALUE;

    /// The `LoadError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/LoadError.html)
    ///
    //+ c-class: error.c `VALUE rb_eLoadError`
    pub static rb_eLoadError: VALUE;

    /// The `Math::DomainError` exception class
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/Math/DomainError.html)
    ///
    //+ c-class: math.c `VALUE rb_eMathDomainError`
    pub static rb_eMathDomainError: VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to an integer.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into an int
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `#define NUM2INT(x)`
    #[link_name = "RS_NUM2INT"]
    pub fn NUM2INT(num: VALUE) -> c_int;

    /// Converts an integer to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define INT2NUM(x)`
    #[link_name = "RS_INT2NUM"]
    pub fn INT2NUM(i: c_int) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to an unsigned integer.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// * Internally cast from an unsigned long
    /// * Undefined behavior when `num` is negative
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into an unsigned int
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `#define NUM2UINT(x)`
    #[link_name = "RS_NUM2UINT"]
    pub fn NUM2UINT(num: VALUE) -> c_uint;

    /// Converts an unsigned integer to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define UINT2NUM(x)`
    #[link_name = "RS_UINT2NUM"]
    pub fn UINT2NUM(ui: c_uint) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to a long.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into a long
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `#define NUM2LONG(x)`
    #[link_name = "RS_NUM2LONG"]
    pub fn NUM2LONG(num: VALUE) -> c_long;

    /// Converts a long to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define LONG2NUM(x)`
    #[link_name = "RS_LONG2NUM"]
    pub fn LONG2NUM(l: c_long) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to an unsigned long.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// * Undefined behavior when `num` is negative
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into an unsigned long
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `#define NUM2ULONG(x)`
    #[link_name = "RS_NUM2ULONG"]
    pub fn NUM2ULONG(num: VALUE) -> c_ulong;

    /// Converts an unsigned long to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define ULONG2NUM(x)`
    #[link_name = "RS_ULONG2NUM"]
    pub fn ULONG2NUM(ul: c_ulong) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to a long long.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into a long long
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `# define NUM2LL(x)`
    #[link_name = "RS_NUM2LL"]
    pub fn NUM2LL(num: VALUE) -> c_longlong;

    /// Converts a long long to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define LL2NUM(v)`
    #[link_name = "RS_LL2NUM"]
    pub fn LL2NUM(ll: c_longlong) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to an unsigned long long.
    ///
    /// Calls `#to_int` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `num` is negative
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate an [`Integer`](rb_cInteger)
    /// * [`RangeError`](rb_eRangeError)
    ///     * if `num` is too large to convert into an unsigned long long
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `# define NUM2ULL(x)`
    #[link_name = "RS_NUM2ULL"]
    pub fn NUM2ULL(num: VALUE) -> c_ulonglong;

    /// Converts an unsigned long long to a Ruby [`Numeric`](rb_cNumeric).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define ULL2NUM(v)`
    #[link_name = "RS_ULL2NUM"]
    pub fn ULL2NUM(ull: c_ulonglong) -> VALUE;

    /// Converts a Ruby [`Numeric`](rb_cNumeric) to an unsigned long long.
    ///
    /// Calls `#to_f` on `num` if necessary.
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `num` is negative
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `num` doesn't have a conversion
    ///     * if internal conversion doesn't generate a [`Float`](rb_cFloat)
    /// * User-defined code may raise exceptions
    ///
    //+ c-macro: `#define NUM2DBL(x)`
    #[link_name = "RS_NUM2DBL"]
    pub fn NUM2DBL(num: VALUE) -> c_double;

    /// Converts a double to a Ruby [`Float`](rb_cFloat).
    ///
    /// # Safety
    ///
    /// No known concerns.
    ///
    //+ c-macro: `#define DBL2NUM(dbl)`
    #[link_name = "RS_DBL2NUM"]
    pub fn DBL2NUM(num: c_double) -> VALUE;

    /// Returns a C boolean (zero if false, non-zero if true) indicating
    /// whether the object is of the internal Ruby type.
    ///
    /// * `obj` - a Ruby object
    /// * `rtype` - a [`ruby_value_type`]
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `obj` is not a [`VALUE`]
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Checking+Data+Types)
    ///
    //+ c-macro: `#define RB_TYPE_P(obj, type)`
    #[link_name = "RS_RB_TYPE_P"]
    pub fn RB_TYPE_P(obj: VALUE, rtype: ruby_value_type) -> c_int;

    /// Returns the Ruby class of the object.
    ///
    /// * `obj` - a Ruby object
    ///
    /// Similar to [`rb_obj_class`] but may return a singleton class.
    ///
    /// # Safety
    ///
    /// No known issues.
    ///
    /// # Miscellaneous
    ///
    /// * As of 2.5, wraps `rb_class_of`.
    ///
    //+ c-macro: `#define CLASS_OF(v)`
    #[link_name = "RS_CLASS_OF"]
    pub fn CLASS_OF(obj: VALUE) -> VALUE;

    /// Returns the byte length of the Ruby [`String`](rb_cString).
    ///
    /// * `string` - an instance of [`String`](rb_cString)
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `string` is not a `String`
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Convert+VALUE+into+C+Data) |
    ///     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Data+Type+Conversion)
    //+ c-macro: `#define RSTRING_LEN(str)`
    #[link_name = "RS_RSTRING_LEN"]
    pub fn RSTRING_LEN(string: VALUE) -> c_long;

    /// Returns a pointer to the Ruby [`String`](rb_cString) data.
    ///
    /// * `string` - an instance of [`String`](rb_cString)
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `string` is not a `String`
    /// * The returned C string may not be nul-terminated
    /// * It is not known whether the C string may contain interior nul-bytes.
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Convert+VALUE+into+C+Data) |
    ///     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Data+Type+Conversion)
    //+ c-macro: `#define RSTRING_PTR(str)`
    #[link_name = "RS_RSTRING_PTR"]
    pub fn RSTRING_PTR(string: VALUE) -> *const c_char;

    /// Returns the number of elements in the Ruby [`Array`](rb_cArray).
    ///
    /// * `array` - an instance of [`Array`](rb_cArray)
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `array` is not an `Array`
    ///
    //+ c-macro: `#define RARRAY_LEN(a)`
    #[link_name = "RS_RARRAY_LEN"]
    pub fn RARRAY_LEN(array: VALUE) -> c_long;

    /// Returns the number of elements in the Ruby [`Hash`](rb_cHash).
    ///
    /// * `hash` - an instance of [`Hash`](rb_cHash)
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `hash` is not a `Hash`
    ///
    //+ c-macro: `#define RHASH_SIZE(h)`
    #[link_name = "RS_RHASH_SIZE"]
    pub fn RHASH_SIZE(hash: VALUE) -> size_t;

    /// Converts an ASCII-encoded, nul-terminated C string to an [`ID`].
    ///
    /// * `cstr` - nul-terminated C string
    ///
    /// # Safety
    ///
    /// * Undefined behavior if string is not valid ASCII
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-ID+or+Symbol)
    ///
    //+ c-func: symbol.c `ID rb_intern(const char*)`
    pub fn rb_intern(cstr: *const c_char) -> ID;

    /// Converts an ASCII-encoded C string of the given length to an [`ID`].
    ///
    /// * `cstr` - C string
    /// * `len` - number of C chars
    ///
    /// # Safety
    ///
    /// * Undefined behavior if string is not valid ASCII.
    /// * Undefined behavior if string contains a nul-byte.
    /// * String must be at least `len` bytes long.
    ///
    /// # Miscellaneous
    ///
    /// [`rb_intern_const`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1757-L1760)
    /// has very similar behavior.
    ///
    //+ c-func: symbol.c `ID rb_intern2(const char*, long)`
    pub fn rb_intern2(cstr: *const c_char, len: c_long) -> ID;

    /// Convert a string to an [`ID`].
    ///
    /// * `string` - an instance of [`String`](rb_cString)
    ///
    /// # Safety
    ///
    /// * Behavior is undefined if you pass a `VALUE` that is not a string.
    ///
    /// # Ruby Documentation
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-ID+or+Symbol)
    ///
    //+ c-func: symbol.c `ID rb_intern_str(VALUE str)`
    pub fn rb_intern_str(string: VALUE) -> ID;

    /// Convert a [`Symbol`](rb_cSymbol) to an [`ID`].
    ///
    /// * `symbol` - an instance of [`Symbol`](rb_cSymbol)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if `symbol` is not a `Symbol`
    ///
    /// # Miscellaneous
    ///
    /// [`RB_SYM2ID`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L381) and
    /// [`SYM2ID`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L386) are currently
    /// aliases for this.
    ///
    //+ c-func: symbol.c `ID rb_sym2id(VALUE)`
    pub fn rb_sym2id(symbol: VALUE) -> ID;

    /// Convert an [`ID`] to a [`Symbol`](rb_cSymbol).
    ///
    /// * Returns an instance of [`Symbol`](rb_cSymbol)
    ///
    /// # Safety
    ///
    /// * Behavior is undefined if the `ID` is not valid.
    ///
    //+ c-func: symbol.c `VALUE rb_id2sym(ID)`
    pub fn rb_id2sym(id: ID) -> VALUE;

    /// Convert an [`ID`] to a Ruby [`String`](rb_cString).
    ///
    //+ c-func: symbol.c `VALUE rb_id2str(ID)`
    pub fn rb_id2str(id: ID) -> VALUE;

    /// Defines a new class.
    ///
    /// NOTE: If the class is already defined and the superclass is the same
    /// as specified, it will return the already defined class.
    ///
    /// * `name` - an ASCII-encoded, nul-terminated C string
    /// * `superclass` - [`Class`](rb_cClass)
    /// * Returns a [`Class`](rb_cClass)
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `name` string is not valid ASCII.
    ///
    /// ## Exceptions
    ///
    /// * [`TypeError`](rb_eTypeError)
    ///     * if the constant name is already taken, but the constant is not a C class
    ///     * if the class is already defined and the superclass does not match
    /// * [`ArgumentError`](rb_eArgError)
    ///     * if the superclass is null
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Class+and+Module+Definition),
    ///     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Defining+Classes+and+Modules)
    ///
    //+ c-func: class.c `VALUE rb_define_class(const char*,VALUE)`
    pub fn rb_define_class(name: *const c_char, superclass: VALUE) -> VALUE;

    /// Defines a public method on a class.
    ///
    /// * `class` - a [`Class`](rb_cClass)
    /// * `name` - an ASCII-encoded, nul-terminated C string
    /// * `func` - `VALUE func(VALUE obj, [VALUE arg, ]*)`, see below if `arity` is negative
    /// * `arity`
    ///     * maximum is `15`
    ///     * if `-1`, function will be called as: `VALUE func(int argc, VALUE *argv, VALUE obj)`
    ///     * if `-2`, function will be called as: `VALUE func(VALUE obj, VALUE args)`
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `name` string is not valid ASCII
    /// * Undefined behavior if `func` signature doesn't match `arity`
    ///
    /// ## Exceptions
    ///
    /// * [`ArgumentError`](rb_eArgError)
    ///     * if `arity` is not in `-2..15`
    ///
    /// # Ruby Documentation
    ///
    /// * **2.5:**
    ///     [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition),
    ///     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+Definition)
    ///
    //+ c-func: class.c `void rb_define_method(VALUE,const char*,VALUE(*)(ANYARGS),int)`
    pub fn rb_define_method(class: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);

    /// Defines a method on a module, both on the module itself and as a private method
    /// for use by anything including the module.
    ///
    /// * `module` - a [`Module`](rb_cModule)
    ///
    /// See [`rb_define_method`](rb_define_method) for additional details on arguments.
    ///
    /// # Safety
    ///
    /// See [`rb_define_method`](rb_define_method#safety).
    ///
    /// # Ruby Documentaiton
    ///
    /// * [2.5](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition)
    ///
    //+ c-func: class.c `void rb_define_module_function(VALUE,const char*,VALUE(*)(ANYARGS),int)`
    pub fn rb_define_module_function(module: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);

    /// Undefines the named method on a class.
    ///
    /// * `class` - a [`Class`](rb_cClass)
    /// * `name` - an ASCII-encoded, nul-terminated C string
    ///
    /// # Safety
    ///
    /// * Undefined behavior if `name` string is not valid ASCII
    ///
    //+ c-func: class.c `void rb_undef_method(VALUE,const char*)`
    pub fn rb_undef_method(class: VALUE, name: *const c_char);

    /// Gets the object's class' name
    ///
    /// * `obj` - any Ruby object
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    //+ c-func: variable.c `const char *rb_obj_classname(VALUE)`
    pub fn rb_obj_classname(obj: VALUE) -> *const c_char;

    /// Returns a C boolean (zero if false, non-zero if true) indicated whether yield would
    /// execute a block in the current method.
    ///
    /// This is the equivalent of `Kernel#block_given?`.
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    //+ c-func: eval.c `int rb_block_given_p(void)`
    pub fn rb_block_given_p() -> c_int;
}

tests! {
    use super::*;
    use super::super::intern;
    use super::super::testing::{Assertions, ToRuby, lazy_eval, c_bool};
    use std::ptr::null;

    use std::ffi::{CStr, CString};

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
    fn test_types(assert: &mut Assertions) {
        // FIXME: Test more types, especially once we have `rb_funcall`
        let objects = unsafe {
            vec![
                ("T_NONE",     T_NONE,     None), // Not sure we can test this
                ("T_OBJECT",   T_OBJECT,   Some(intern::rb_class_new_instance(0, null(), rb_cObject))),
                ("T_CLASS",    T_CLASS,    Some(rb_cObject)),
                ("T_MODULE",   T_MODULE,   Some(rb_mKernel)),
                ("T_FLOAT",    T_FLOAT,    None), // Could do this with funcall and to_f
                ("T_STRING",   T_STRING,   Some("foo".to_ruby())),
                ("T_REGEXP",   T_REGEXP,   Some(intern::rb_class_new_instance(1, &"[a-z]".to_ruby(), rb_cRegexp))),
                ("T_ARRAY",    T_ARRAY,    Some(intern::rb_ary_new())),
                ("T_HASH",     T_HASH,     Some(intern::rb_hash_new())),
                ("T_STRUCT",   T_STRUCT,   None), // Could do this with funcall and to_f
                ("T_BIGNUM",   T_BIGNUM,   None), // Could do this with funcall and to_i
                ("T_FILE",     T_FILE,     None), // Need to get a file descriptor for this
                ("T_DATA",     T_DATA,     None), // Not sure we can test this
                ("T_MATCH",    T_MATCH,    None), // Can do this with funcall
                ("T_COMPLEX",  T_COMPLEX,  None), // Can do this with funcall
                ("T_RATIONAL", T_RATIONAL, None), // Can do this with funcall
                ("T_NIL",      T_NIL,      Some(Qnil)),
                ("T_TRUE",     T_TRUE,     Some(Qtrue)),
                ("T_FALSE",    T_FALSE,    Some(Qfalse)),
                ("T_SYMBOL",   T_SYMBOL,   Some(rb_id2sym(rb_intern(cstr!("foo"))))),
                ("T_FIXNUM",   T_FIXNUM,   None), // Could do this with funcall and to_i
                ("T_UNDEF",    T_UNDEF,    None), // Not sure we can test this
                ("T_IMEMO",    T_IMEMO,    None), // Not sure we can test this
                ("T_NODE",     T_NODE,     None), // Not sure we can test this
                ("T_ICLASS",   T_ICLASS,   None), // Not sure we can test this
                ("T_ZOMBIE",   T_ZOMBIE,   None) // Not sure we can test this
            ]
        };

        for (name, rtype, value) in objects.into_iter() {
            if let Some(v) = value {
                assert.rs_eq_msg(c_bool(unsafe { RB_TYPE_P(v, rtype) }), true, Some(format!("type of VALUE is {}", name)));
            } else {
                // Sanity check
                assert.rs_eq_msg(rtype.0 >= 0x00, true, Some(format!("{} >= 0x00", name)));
                assert.rs_eq_msg(rtype.0 < 0x1f, true, Some(format!("{} < 0x1f", name)));
            }
        }

        // Ensure that it doesn't only return true
        assert.rs_eq(c_bool(unsafe { RB_TYPE_P("foo".to_ruby(), T_HASH) }), false);
    }

    #[test]
    fn test_class_of(assert: &mut Assertions) {
        assert.rb_eq(unsafe { rb_cString }, unsafe { CLASS_OF("foo".to_ruby()) });

        // `rb_obj_class` on a class returns the class itself, `CLASS_OF` returns the singleton class
        assert.rb_ne(unsafe { intern::rb_obj_class(rb_cString) }, unsafe { CLASS_OF(rb_cString) });
        assert.rb_eq(lazy_eval("String.singleton_class"), unsafe { CLASS_OF(rb_cString) });
    }

    #[test]
    fn test_numeric_conversions(assert: &mut Assertions) {
        let int_max = c_int::max_value();
        let long_max = c_long::max_value();
        let long_long_max = c_longlong::max_value();

        let val = -int_max + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { INT2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2INT(INT2NUM(val)) });

        let val = (int_max as c_uint) + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { UINT2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2UINT(UINT2NUM(val)) });

        let val = -long_max + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { LONG2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2LONG(LONG2NUM(val)) });

        let val = (long_max as c_ulong) + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { ULONG2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2ULONG(ULONG2NUM(val)) });

        let val = -long_long_max + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { LL2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2LL(LL2NUM(val)) });

        let val = (long_long_max as c_ulonglong) + 1;
        assert.rb_eq(lazy_eval(&format!("{}", val)), unsafe { ULL2NUM(val) });
        assert.rs_eq(val, unsafe { NUM2ULL(ULL2NUM(val)) });
    }

    #[test]
    fn test_rstring_len(assert: &mut Assertions) {
        assert.rs_eq(unsafe { RSTRING_LEN("".to_ruby()) }, 0);
        assert.rs_eq(unsafe { RSTRING_LEN("foo".to_ruby()) }, 3);
        assert.rs_eq(unsafe { RSTRING_LEN("this is longer".to_ruby()) }, 14);
        assert.rs_eq(unsafe { RSTRING_LEN("☠️".to_ruby()) }, 6);
    }

    #[test]
    fn test_rstring_ptr(assert: &mut Assertions) {
        let cstr = unsafe { CStr::from_ptr(RSTRING_PTR("foobar".to_ruby()) as *const c_char) };
        let unicode = unsafe { CStr::from_ptr(RSTRING_PTR("☠️".to_ruby()) as *const c_char) };
        assert.rs_eq(cstr.to_string_lossy(), "foobar");
        assert.rs_eq(unicode.to_string_lossy(), "☠️");
    }

    #[test]
    fn test_rarray_len(assert: &mut Assertions) {
        let array = unsafe { intern::rb_ary_new() };

        assert.rs_eq(unsafe { RARRAY_LEN(array) }, 0);

        unsafe { intern::rb_ary_push(array, "foo".to_ruby()); }
        assert.rs_eq(unsafe { RARRAY_LEN(array) }, 1);

        unsafe { intern::rb_ary_push(array, "bar".to_ruby()); }
        assert.rs_eq(unsafe { RARRAY_LEN(array) }, 2);
    }

    #[test]
    fn test_rhash_size(assert: &mut Assertions) {
        let hash = unsafe { intern::rb_hash_new() };

        assert.rs_eq(unsafe { RHASH_SIZE(hash) }, 0);

        unsafe { intern::rb_hash_aset(hash, "foo".to_ruby(), "bar".to_ruby()); }
        assert.rs_eq(unsafe { RHASH_SIZE(hash) }, 1);

        unsafe { intern::rb_hash_aset(hash, "baz".to_ruby(), "qux".to_ruby()); }
        assert.rs_eq(unsafe { RHASH_SIZE(hash) }, 2);
    }

    #[test]
    fn test_id(assert: &mut Assertions) {
        let foo1 = unsafe { rb_intern(cstr!("foo")) };
        let foo2 = unsafe { rb_intern(CString::new("foo").unwrap().into_raw()) };
        let foo3 = unsafe { rb_intern2("foobar".as_ptr() as *const c_char, 3 as c_long) };
        let foo4 = unsafe { rb_intern_str("foo".to_ruby()) };

        let bar = unsafe { rb_intern(cstr!("bar")) };

        assert.rs_eq(foo1, foo1);
        assert.rs_eq(foo1, foo2);
        assert.rs_eq(foo1, foo3);
        assert.rs_eq(foo1, foo4);

        assert.rs_ne(foo1, bar);
        assert.rs_ne(foo2, bar);
        assert.rs_ne(foo3, bar);
        assert.rs_ne(foo4, bar);

        assert.rs_eq(foo1, unsafe { rb_sym2id(rb_id2sym(foo1)) });
        assert.rs_eq(foo2, unsafe { rb_sym2id(rb_id2sym(foo2)) });
        assert.rs_eq(foo3, unsafe { rb_sym2id(rb_id2sym(foo3)) });
        assert.rs_eq(foo4, unsafe { rb_sym2id(rb_id2sym(foo4)) });

        assert.rs_ne(foo1, unsafe { rb_sym2id(rb_id2sym(bar)) });
        assert.rs_ne(foo2, unsafe { rb_sym2id(rb_id2sym(bar)) });
        assert.rs_ne(foo3, unsafe { rb_sym2id(rb_id2sym(bar)) });
        assert.rs_ne(foo4, unsafe { rb_sym2id(rb_id2sym(bar)) });

        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo1) });
        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo2) });
        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo3) });
        assert.rb_eq(lazy_eval(":foo"), unsafe { rb_id2sym(foo4) });
        assert.rb_eq(lazy_eval(":bar"), unsafe { rb_id2sym(bar) });

        assert.rb_eq(lazy_eval("'foo'"), unsafe { rb_id2str(foo1) });
        assert.rb_eq(lazy_eval("'foo'"), unsafe { rb_id2str(foo2) });
        assert.rb_eq(lazy_eval("'foo'"), unsafe { rb_id2str(foo3) });
        assert.rb_eq(lazy_eval("'foo'"), unsafe { rb_id2str(foo4) });
        assert.rb_eq(lazy_eval("'bar'"), unsafe { rb_id2str(bar) });
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

    #[test]
    fn test_obj_classname(assert: &mut Assertions) {
        let class = unsafe { CStr::from_ptr(rb_obj_classname(rb_cObject)) };
        let module = unsafe { CStr::from_ptr(rb_obj_classname(rb_mKernel)) };
        let instance = unsafe {
            let i = intern::rb_class_new_instance(0, null(), rb_cObject);
            CStr::from_ptr(rb_obj_classname(i))
        };

        assert.rs_eq("Class", class.to_string_lossy());
        assert.rs_eq("Module", module.to_string_lossy());
        assert.rs_eq("Object", instance.to_string_lossy());
    }

    #[test]
    fn test_block_given_p(assert: &mut Assertions) {
        extern "C" fn __test_block_given_p__(_self: VALUE) -> VALUE {
            if unsafe { rb_block_given_p() != 0 } {
                unsafe { Qtrue }
            } else {
                unsafe { Qfalse }
            }
        }

        unsafe {
            rb_define_method(
                rb_mKernel,
                cstr!("__test_block_given_p__"),
                ANYARGS::from_arity_1(__test_block_given_p__),
                0
            );
        }

        assert.rb_eq(
            lazy_eval("__test_block_given_p__"),
            unsafe { Qfalse }
        );

        assert.rb_eq(
            lazy_eval("__test_block_given_p__ { }"),
            unsafe { Qtrue }
        );
    }
}
