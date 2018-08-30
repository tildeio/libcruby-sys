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
impl_from_arity!(from_arity_13, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_14, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);
impl_from_arity!(from_arity_15, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE);

extern {
    #[link_name = "RS_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "RS_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "RS_Qnil"]
    pub static Qnil: VALUE;


    /// The `Kernel` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1891) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Kernel.html)
    pub static rb_mKernel: VALUE;

    /// The `Comparable` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1892) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Comparable.html)
    pub static rb_mComparable: VALUE;

    /// The `Enumerable` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1893) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Enumerable.html)
    pub static rb_mEnumerable: VALUE;

    /// The `Errno` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1894) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Errno.html)
    pub static rb_mErrno: VALUE;

    /// The `FileTest` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1895) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/FileTest.html)
    pub static rb_mFileTest: VALUE;

    /// The `GC` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1896) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/GC.html)
    pub static rb_mGC: VALUE;

    /// The `Math` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1897) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Math.html)
    pub static rb_mMath: VALUE;

    /// The `Process` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1898) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Process.html)
    pub static rb_mProcess: VALUE;

    /// The `IO::WaitReadable` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1899) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/IO/WaitReadable.html)
    pub static rb_mWaitReadable: VALUE;

    /// The `IO::WaitWritable` module
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1900) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/IO/WaitWritable.html)
    pub static rb_mWaitWritable: VALUE;


    /// The `BasicObject` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1902) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/BasicObject.html)
    pub static rb_cBasicObject: VALUE;

    /// The `Object` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1903) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Object.html)
    pub static rb_cObject: VALUE;

    /// The `Array` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1904) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Array.html)
    pub static rb_cArray: VALUE;

    /// The `Binding` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1908) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Binding.html)
    pub static rb_cBinding: VALUE;

    /// The `Class` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1909) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Class.html)
    pub static rb_cClass: VALUE;

    /// The `Dir` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1911) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Dir.html)
    pub static rb_cDir: VALUE;

    /// The `Encoding` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1914) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Encoding.html)
    pub static rb_cEncoding: VALUE;

    /// The `Enumerator` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1915) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Enumerator.html)
    pub static rb_cEnumerator: VALUE;

    /// The `FalseClass` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1913) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/FalseClass.html)
    pub static rb_cFalseClass: VALUE;

    /// The `File` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1916) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/File.html)
    pub static rb_cFile: VALUE;

    /// The `Complex` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1934) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Complex.html)
    pub static rb_cComplex: VALUE;

    /// The `Float` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1920) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Float.html)
    pub static rb_cFloat: VALUE;

    /// The `Hash` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1921) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Hash.html)
    pub static rb_cHash: VALUE;

    /// The `IO` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1923) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/IO.html)
    pub static rb_cIO: VALUE;

    /// The `Integer` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1922) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Integer.html)
    pub static rb_cInteger: VALUE;

    /// The `MatchData` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1924) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/MatchData.html)
    pub static rb_cMatch: VALUE;

    /// The `Method` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1925) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Method.html)
    pub static rb_cMethod: VALUE;

    /// The `Module` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1926) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Module.html)
    pub static rb_cModule: VALUE;

    /// The `NilClass` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1928) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/NilClass.html)
    pub static rb_cNilClass: VALUE;

    /// The `Numeric` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1929) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Numeric.html)
    pub static rb_cNumeric: VALUE;

    /// The `Proc` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1930) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Proc.html)
    pub static rb_cProc: VALUE;

    /// The `Random` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1931) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Random.html)
    pub static rb_cRandom: VALUE;

    /// The `Range` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1932) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Range.html)
    pub static rb_cRange: VALUE;

    /// The `Rational` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1933) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Rational.html)
    pub static rb_cRational: VALUE;

    /// The `Regexp` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1935) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Regexp.html)
    pub static rb_cRegexp: VALUE;

    /// The `File::Stat` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1936) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/File/Stat.html)
    pub static rb_cStat: VALUE;

    /// The `String` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1937) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/String.html)
    pub static rb_cString: VALUE;

    /// The `Struct` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1938) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Struct.html)
    pub static rb_cStruct: VALUE;

    /// The `Symbol` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1939) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Symbol.html)
    pub static rb_cSymbol: VALUE;

    /// The `Thread` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1940) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Thread.html)
    pub static rb_cThread: VALUE;

    /// The `Time` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1941) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Time.html)
    pub static rb_cTime: VALUE;

    /// The `TrueClass` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1942) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/TrueClass.html)
    pub static rb_cTrueClass: VALUE;

    /// The `UnboundMethod` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1943) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/UnboundMethod.html)
    pub static rb_cUnboundMethod: VALUE;


    /// The `Exception` class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1945) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Exception.html)
    pub static rb_eException: VALUE;

    /// The `StandardError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1946) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/StandardError.html)
    pub static rb_eStandardError: VALUE;

    /// The `SystemExit` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1947) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/SystemExit.html)
    pub static rb_eSystemExit: VALUE;

    /// The `Interrupt` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1948) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Interrupt.html)
    pub static rb_eInterrupt: VALUE;

    /// The `Signal` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1949) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Signal.html)
    pub static rb_eSignal: VALUE;

    /// The `ArgumentError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1951) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/ArgumentError.html)
    pub static rb_eArgError: VALUE;

    /// The `EOFError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1952) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/EOFError.html)
    pub static rb_eEOFError: VALUE;

    /// The `IndexError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1953) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/IndexError.html)
    pub static rb_eIndexError: VALUE;

    /// The `StopIteration` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1954) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/StopIteration.html)
    pub static rb_eStopIteration: VALUE;

    /// The `KeyError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1955) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/KeyError.html)
    pub static rb_eKeyError: VALUE;

    /// The `RangeError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1956) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/RangeError.html)
    pub static rb_eRangeError: VALUE;

    /// The `IOError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1957) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/IOError.html)
    pub static rb_eIOError: VALUE;

    /// The `RuntimeError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1958) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/RuntimeError.html)
    pub static rb_eRuntimeError: VALUE;

    /// The `SecurityError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1960) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/SecurityError.html)
    pub static rb_eSecurityError: VALUE;

    /// The `SystemCallError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1961) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/SystemCallError.html)
    pub static rb_eSystemCallError: VALUE;

    /// The `ThreadError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1962) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/ThreadError.html)
    pub static rb_eThreadError: VALUE;

    /// The `TypeError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1963) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/TypeError.html)
    pub static rb_eTypeError: VALUE;

    /// The `ZeroDivisionError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1964) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/ZeroDivisionError.html)
    pub static rb_eZeroDivError: VALUE;

    /// The `NotImplementedError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1965) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/NotImplementedError.html)
    pub static rb_eNotImpError: VALUE;

    /// The `NoMemoryError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1966) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/NoMemoryError.html)
    pub static rb_eNoMemError: VALUE;

    /// The `NoMethodError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1967) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/NoMethodError.html)
    pub static rb_eNoMethodError: VALUE;

    /// The `FloatDomainError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1968) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/FloatDomainError.html)
    pub static rb_eFloatDomainError: VALUE;

    /// The `LocalJumpError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1969) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/LocalJumpError.html)
    pub static rb_eLocalJumpError: VALUE;

    /// The `SystemStackError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1970) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/SystemStackError.html)
    pub static rb_eSysStackError: VALUE;

    /// The `RegexpError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1971) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/RegexpError.html)
    pub static rb_eRegexpError: VALUE;

    /// The `EncodingError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1972) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/EncodingError.html)
    pub static rb_eEncodingError: VALUE;

    /// The `Encoding::CompatibilityError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1973) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Encoding/CompatibilityError.html)
    pub static rb_eEncCompatError: VALUE;

    /// The `ScriptError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1975) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/ScriptError.html)
    pub static rb_eScriptError: VALUE;

    /// The `NameError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1976) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/NameError.html)
    pub static rb_eNameError: VALUE;

    /// The `SyntaxError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1977) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/SyntaxError.html)
    pub static rb_eSyntaxError: VALUE;

    /// The `LoadError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1978) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/LoadError.html)
    pub static rb_eLoadError: VALUE;

    /// The `Math::DomainError` exception class
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1980) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/Math/DomainError.html)
    pub static rb_eMathDomainError: VALUE;


    /// Convert a C string to an [`ID`](struct.id.html)
    ///
    /// * `cstr` - nul-terminated C string, treated as ASCII encoded
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1728) |
    ///     [symbol.c](https://github.com/ruby/ruby/blob/v2_5_1/symbol.c#L610-L614) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-ID+or+Symbol)
    pub fn rb_intern(cstr: *const c_char) -> ID;

    /// Convert a C string of the given length to an [`ID`](struct.id.html)
    ///
    /// * `cstr` - C string, treated as ASCII encoded
    /// * `len` - number of C chars
    ///
    /// # Miscellaneous
    ///
    /// [`rb_intern_const`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1757-L1760)
    /// is more or less an alias of this.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1729)
    ///     [symbol.c](https://github.com/ruby/ruby/blob/v2_5_1/symbol.c#L603-L607)
    pub fn rb_intern2(cstr: *const c_char, len: c_long) -> ID;

    /// Convert a string to an [`ID`](struct.id.html)
    ///
    /// * `string`: an instance of [`rb_cString`](static.rb_cString.html)
    ///
    /// # Safety
    ///
    /// * Behavior is undefined if you pass a `VALUE` that is not a string.
    ///
    /// # Defined In
    ///
    /// * **2.3:** [ruby.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/ruby.h#L1708)
    /// * **2.4:** [ruby.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/ruby.h#L1728)
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1730)
    ///     [symbol.c](https://github.com/ruby/ruby/blob/v2_5_1/symbol.c#L616-L626) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-ID+or+Symbol)
    /// * **2.6:** [ruby.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/ruby.h#L1747)
    pub fn rb_intern_str(string: VALUE) -> ID;

    /// Convert a [`rb_cSymbol`](static.rb_cSymbol.html)
    /// to an [`ID`](struct.id.html)
    ///
    /// * `symbol`: an instance of [`rb_cSymbol`](static.rb_cSymbol.html)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eTypeError`](static.rb_eTypeError.html)
    ///     * if `symbol` is not a `rb_cSymbol`
    ///
    /// # Miscellaneous
    ///
    /// [`RB_SYM2ID`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L381) and
    /// [`SYM2ID`](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L386) are currently
    /// aliases for this.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L375) |
    ///     [symbol.c](https://github.com/ruby/ruby/blob/v2_5_1/symbol.c#L697-L722)
    pub fn rb_sym2id(symbol: VALUE) -> ID;

    /// Convert an [`ID`](struct.ID.html) to a [`rb_cSymbol`](static.rb_cSymbol.html)
    ///
    /// * Returns an instance of [`rb_cSymbol`](static.rb_cSymbol.html)
    ///
    /// # Safety
    ///
    /// * Behavior is undefined if the `ID` is not valid.
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L376) |
    ///     [symbol.c](https://github.com/ruby/ruby/blob/v2_5_1/symbol.c#L725-L730)
    pub fn rb_id2sym(id: ID) -> VALUE;

    /// Define a new class
    ///
    /// NOTE: If the class is already defined and the superclass is the same
    /// as specified, it will return the already defined class.
    ///
    /// * `name` - nul-terminated C string, treated as ASCII encoded
    /// * `superclass`: [`rb_cClass`](static.rb_cClass.html)
    /// * Returns a [`rb_cClass`](static.rb_cClass.html)
    ///
    /// # Safety
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eTypeError`](static.rb_eTypeError.html)
    ///     * if the constant name is already taken, but the constant is not a C class
    ///     * if the class is already defined and the superclass does not match
    /// * [`rb_eArgError`](static.rb_eArgError.html)
    ///     * if the superclass is null
    ///
    /// # Defined In
    ///
    /// * **2.3:** [ruby.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/ruby.h#L1656)
    /// * **2.4:** [ruby.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/ruby.h#L1676)
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1678) |
    ///     [class.c](https://github.com/ruby/ruby/blob/v2_5_1/class.c#L645-L673) |
    ///     documentation: [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Class+and+Module+Definition),
    ///                     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Defining+Classes+and+Modules)
    /// * **2.6:** [ruby.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/ruby.h#L1695)
    pub fn rb_define_class(name: *const c_char, superclass: VALUE) -> VALUE;

    /// Defines a public method on a class
    ///
    /// * `class`: a [`rb_cClass`](static.rb_cClass.html)
    /// * `name` - nul-terminated C string, treated as ASCII encoded
    /// * `func` - `VALUE func(VALUE obj, [VALUE arg, ]*)`, see below if `arity` is negative
    /// * `arity`
    ///     * maximum is `15`
    ///     * if `-1`, function will be called as: `VALUE func(int argc, VALUE *argv, VALUE obj)`
    ///     * if `-2`, function will be called as: `VALUE func(VALUE obj, VALUE args)`
    ///
    /// # Safety
    ///
    /// * Behavior is undefined if your passed in function signature doesn't match the
    /// provided arity
    ///
    /// ## Exceptions
    ///
    /// * [`rb_eArgError`](static.rb_eArgError.html)
    ///     * if `arity` is not in `-2..15`
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1715) |
    ///     [class.c](https://github.com/ruby/ruby/blob/v2_5_1/class.c#L1514-L1518) |
    ///     documentation: [usage](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition),
    ///                     [spec](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+Definition)
    pub fn rb_define_method(class: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);

    /// Defines a method on a module, both on the module itself and as a private method
    /// for use by anything including the module.
    ///
    /// * `module`: a [`rb_cModule`](static.rb_cModule.html)
    ///
    /// See [`rb_define_method`](fn.rb_define_method.html) for additional details on arguments.
    ///
    /// # Safety
    ///
    /// See [`rb_define_method`](fn.rb_define_method.html#safety).
    ///
    /// # Defined In
    ///
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1716) |
    ///     [class.c](https://github.com/ruby/ruby/blob/v2_5_1/class.c#L1730-L1735) |
    ///     [documentation](https://ruby-doc.org/core-2.5.1/doc/extension_rdoc.html#label-Method+and+Singleton+Method+Definition)
    pub fn rb_define_module_function(module: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);

    /// Undefines the named method on a class
    ///
    /// * `class`: a [`rb_cClass`](static.rb_cClass.html)
    /// * `name` - nul-terminated C string, treated as ASCII encoded
    ///
    /// # Safety
    ///
    /// * No known issues
    ///
    /// # Defined In
    ///
    /// * **2.3:** [ruby.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/ruby.h#L1697)
    /// * **2.4:** [ruby.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/ruby.h#L1717)
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1719) |
    ///     [class.c](https://github.com/ruby/ruby/blob/v2_5_1/class.c#L1532-L1536)
    /// * **2.6:** [ruby.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/ruby.h#L1736)
    pub fn rb_undef_method(class: VALUE, name: *const c_char);

    /// Gets the object's class' name
    ///
    /// * `obj`: any Ruby object
    ///
    /// # Safety
    ///
    /// No known issues
    ///
    /// # Defined In
    ///
    /// * **2.3:** [ruby.h](https://github.com/ruby/ruby/blob/v2_3_7/include/ruby/ruby.h#L1744)
    /// * **2.4:** [ruby.h](https://github.com/ruby/ruby/blob/v2_4_4/include/ruby/ruby.h#L1764)
    /// * **2.5:**
    ///     [ruby.h](https://github.com/ruby/ruby/blob/v2_5_1/include/ruby/ruby.h#L1791) |
    ///     [variable.c](https://github.com/ruby/ruby/blob/v2_5_1/variable.c#L459-L463)
    /// * **2.6:** [ruby.h](https://github.com/ruby/ruby/blob/v2_6_0_preview2/include/ruby/ruby.h#L1808)
    pub fn rb_obj_classname(obj: VALUE) -> *const c_char;
}

tests! {
    use super::*;
    use super::super::intern;
    use super::super::testing::{Assertions, ToRuby, lazy_eval};
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
}
